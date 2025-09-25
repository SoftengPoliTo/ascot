use core::fmt::{Debug, Display};
use core::net::{Ipv4Addr, SocketAddr};
use core::pin::Pin;

use alloc::boxed::Box;

use ascot::route::RestKind;

use edge_http::io::server::{Connection, Handler, Server as EdgeServer};
use edge_http::Method;
use edge_nal::TcpBind;
use edge_nal_embassy::{Tcp, TcpBuffers};

use embassy_executor::Spawner;
use embassy_net::Stack;

use embedded_io_async::{Read, Write};

use log::info;

use crate::device::Device;
use crate::error::Error;
use crate::mdns::Mdns;
use crate::response::{EmptyResponse, IntoResponse, Response};
use crate::state::{State, ValueFromRef};

// Default HTTP address.
//
// The entire local network is considered, so the Ipv4 unspecified address is
// used.
const DEFAULT_HTTP_ADDRESS: Ipv4Addr = Ipv4Addr::UNSPECIFIED;

// Default port.
const DEFAULT_SERVER_PORT: u16 = 80;

// Number of sockets used for the HTTP server
const SERVER_SOCKETS: usize = 1;

pub(crate) type InputFn = Box<
    dyn Fn() -> Pin<Box<dyn Future<Output = Response> + Send + Sync + 'static>>
        + Send
        + Sync
        + 'static,
>;

pub(crate) type InputStateFn<S> = Box<
    dyn Fn(State<S>) -> Pin<Box<dyn Future<Output = Response> + Send + Sync + 'static>>
        + Send
        + Sync
        + 'static,
>;

#[derive(Clone, Copy)]
pub(crate) enum FuncType {
    First,
    Second,
}

#[derive(Clone, Copy)]
pub(crate) struct FuncIndex {
    func_type: FuncType,
    index: usize,
}

impl FuncIndex {
    pub(crate) const fn new(func_type: FuncType, index: usize) -> Self {
        Self { func_type, index }
    }
}

/// The `ascot` server.
pub struct Server<
    const TX_SIZE: usize,
    const RX_SIZE: usize,
    const MAXIMUM_HEADERS_COUNT: usize,
    const TIMEOUT: u32,
    S,
> where
    S: ValueFromRef + Send + Sync + 'static,
{
    // HTTP address.
    address: Ipv4Addr,
    // Server port.
    port: u16,
    // HTTP handler.
    handler: ServerHandler<S>,
    // mDNS
    mdns: Mdns,
}

impl<
        const TX_SIZE: usize,
        const RX_SIZE: usize,
        const MAXIMUM_HEADERS_COUNT: usize,
        const TIMEOUT: u32,
        S,
    > Server<TX_SIZE, RX_SIZE, MAXIMUM_HEADERS_COUNT, TIMEOUT, S>
where
    S: ValueFromRef + Send + Sync + 'static,
{
    /// Creates a [`Server`].
    #[inline]
    pub fn new(device: Device<S>, mdns: Mdns) -> Self {
        Self {
            address: DEFAULT_HTTP_ADDRESS,
            port: DEFAULT_SERVER_PORT,
            handler: ServerHandler::new(device),
            mdns,
        }
    }

    /// Sets the `IPv4` address for the server.
    #[must_use]
    pub const fn address(mut self, address: Ipv4Addr) -> Self {
        self.address = address;
        self
    }

    ///  Sets the port number for the server to listen on.
    #[must_use]
    pub const fn port(mut self, port: u16) -> Self {
        self.port = port;
        self
    }

    /// Runs the [`Server`] and the [`Mdns`] task.
    ///
    /// # Errors
    ///
    /// - Failure to bind TCP protocol buffers to the underlying socket
    /// - Failure to spawn the `mDNS` task
    /// - Failure to run the server
    pub async fn run(self, stack: Stack<'static>, spawner: Spawner) -> Result<(), Error> {
        let buffers = TcpBuffers::<SERVER_SOCKETS, TX_SIZE, RX_SIZE>::new();
        let tcp = Tcp::new(stack, &buffers);

        let socket = SocketAddr::new(self.address.into(), self.port);

        let acceptor = tcp.bind(socket).await?;

        let mut server = EdgeServer::<SERVER_SOCKETS, RX_SIZE, MAXIMUM_HEADERS_COUNT>::new();

        // Run mdns.
        // NOTE: Use the same server port for the mDNS-SD service
        self.mdns.run(stack, self.port, spawner)?;

        info!(
            "Starting server on address `{}` and port `{}`",
            self.address, self.port
        );

        // Run server.
        server
            .run(Some(TIMEOUT), acceptor, self.handler)
            .await
            .map_err(core::convert::Into::into)
    }
}

struct ServerHandler<S>
where
    S: ValueFromRef + Send + Sync + 'static,
{
    device: Device<S>,
    not_found_response: Response,
    not_allowed_response: Response,
}

impl<S> ServerHandler<S>
where
    S: ValueFromRef + Send + Sync + 'static,
{
    #[inline]
    fn new(device: Device<S>) -> Self {
        Self {
            device,
            not_found_response: EmptyResponse::not_found().into_response(),
            not_allowed_response: EmptyResponse::not_allowed().into_response(),
        }
    }

    fn check_path(&self, method: Method, path: &str) -> usize {
        let not_correct_route = self.device.route_configs.len();

        // Create an iterator to parse the route in an iterative way. This
        // function removes the trailing '/' which might be present in route
        // definition.
        let mut route_iter = path.split_terminator('/');

        // Every time a `nth(0)` function is being called, the iterator
        // consumes the current element.
        //
        // The first 0-element of a route is always an empty value
        // because **each** path begins with a '/'.
        // For this reason, we are calling the same function twice in
        // a row.
        let empty_token = route_iter.nth(0);

        // Verify if the split function worked. If the first element is equal to
        // the path, return a not found route.
        match empty_token {
            Some(empty_token) => {
                // If the empty token is equal to the entire path,
                // the route is not correct.
                if empty_token == path {
                    return not_correct_route;
                }
            }
            None => return not_correct_route,
        }

        // Retrieve the main route.
        let first_token = route_iter.nth(0);

        match first_token {
            Some(first_token) => {
                // If the first token is not equal to the main route,
                // the route is not correct. Starts from the 1-index
                // in order to skip the "/" placed before the main route.
                if first_token != &self.device.main_route[1..] {
                    return not_correct_route;
                }
            }
            None => return not_correct_route,
        }

        // Get the second token and compare it with the route parameter.
        let second_token = route_iter.nth(0);

        // TODO: Consider a route with parameters.

        for (index, route) in self.device.route_configs.iter().enumerate() {
            match second_token {
                Some(second_token) => {
                    // If the method is not equal to the one in the route,
                    // pass to the next route.
                    //
                    // If the second token is equal to the route path,
                    // the route is correct, therefore return the relative index.
                    if Self::is_same_method(method, route.rest_kind)
                        && second_token == &route.data.path[1..]
                    {
                        return index;
                    }
                }
                None => return not_correct_route,
            }
        }
        not_correct_route
    }

    #[inline]
    async fn run_function(&self, index: usize) -> Response {
        let func_index = self.device.index_array[index];

        match func_index.func_type {
            FuncType::First => {
                let func = &self.device.routes_functions.0[func_index.index];
                func().await
            }
            FuncType::Second => {
                let func = &self.device.routes_functions.1[func_index.index];
                func(State(S::value_from_ref(&self.device.state.0))).await
            }
        }
    }

    #[inline]
    fn is_same_method(method: Method, ascot_method: RestKind) -> bool {
        let compare_method = match ascot_method {
            RestKind::Get => Method::Get,
            RestKind::Put => Method::Put,
            RestKind::Post => Method::Post,
            RestKind::Delete => Method::Delete,
        };

        method == compare_method
    }

    const fn is_method_allowed(method: Method) -> bool {
        !matches!(
            method,
            Method::Get | Method::Post | Method::Put | Method::Delete
        )
    }
}

impl<S: ValueFromRef + Send + Sync + 'static> Handler for ServerHandler<S> {
    type Error<E>
        = edge_http::io::Error<E>
    where
        E: Debug;

    async fn handle<T, const N: usize>(
        &self,
        _task_id: impl Display + Copy,
        conn: &mut Connection<'_, T, N>,
    ) -> Result<(), Self::Error<T::Error>>
    where
        T: Read + Write,
    {
        let headers = conn.headers()?;

        if headers.path == "/" {
            return self.device.main_route_response.write_from_ref(conn).await;
        }

        if Self::is_method_allowed(headers.method) {
            return self.not_allowed_response.write_from_ref(conn).await;
        }

        let index = self.check_path(headers.method, headers.path);

        if index == self.device.route_configs.len() {
            return self.not_found_response.write_from_ref(conn).await;
        }

        let response = self.run_function(index).await;
        response.write(conn).await
    }
}
