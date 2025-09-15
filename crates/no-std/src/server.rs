use core::{future::Future, pin::Pin};

use alloc::boxed::Box;

use embassy_executor::Spawner;
use embassy_net::Stack;
use embassy_time::Duration;
use picoserve::{
    listen_and_serve,
    routing::{NoPathParameters, PathRouter, Router},
    Config, KeepAlive, Timeouts,
};

use log::info;

use crate::device::Device;
use crate::error::Result;
use crate::mdns::Mdns;
use crate::mk_static;
use crate::net::get_ip;

// We cannot use Send because internal structures of Stack does not implement
// that. The same goes for Sync. The function is an FnOnce because it needs
// to be called just one time inside, otherwise there are some problems with
// Fn and FnMut.
type ServerTaskFn = Box<dyn Fn() -> Pin<Box<dyn Future<Output = ()> + 'static>> + 'static>;

macro_rules! web_task {
    ($pool_size_ident:ident, $pool_size_value:tt) => {
        #[embassy_executor::task(pool_size = $pool_size_value)]
        async fn $pool_size_ident(server_task: ServerTaskFn) {
            let leak_task = Box::leak(server_task);
            leak_task().await
        }
    };
}

web_task!(web_task1, 1);
web_task!(web_task2, 2);
web_task!(web_task3, 3);
web_task!(web_task4, 4);
web_task!(web_task5, 5);
web_task!(web_task6, 6);
web_task!(web_task7, 7);
web_task!(web_task8, 8);

#[allow(clippy::similar_names)]
async fn internal_server_run(
    id: usize,
    stack: Stack<'static>,
    app: &'static Router<impl PathRouter>,
    config: &'static Config<Duration>,
    port: u16,
) {
    let mut tcp_rx_buffer = [0; 1024];
    let mut tcp_tx_buffer = [0; 1024];
    let mut http_buffer = [0; 2048];

    listen_and_serve(
        id,
        app,
        config,
        stack,
        port,
        &mut tcp_rx_buffer,
        &mut tcp_tx_buffer,
        &mut http_buffer,
    )
    .await;
}

/// Server configuration.
pub struct ServerConfig(Config<Duration>);

impl Default for ServerConfig {
    fn default() -> Self {
        Self::new()
    }
}

impl ServerConfig {
    /// Creates a [`ServerConfig`].
    #[must_use]
    pub const fn new() -> Self {
        Self(Config {
            timeouts: Timeouts {
                start_read_request: None,
                persistent_start_read_request: None,
                read_request: None,
                write: None,
            },
            connection: KeepAlive::KeepAlive,
        })
    }

    /// Adds the duration of time to wait when starting to read the
    /// first request before the connection is closed due to inactivity.
    #[must_use]
    pub const fn start_read_request(mut self, duration_of_time: Duration) -> Self {
        self.0.timeouts.start_read_request = Some(duration_of_time);
        self
    }

    /// Adds the duration of time to wait when starting to read persistent
    /// (i.e. not the first) requests before the connection is closed
    /// due to inactivity.
    #[must_use]
    pub const fn persistent_start_read_request(mut self, duration_of_time: Duration) -> Self {
        self.0.timeouts.persistent_start_read_request = Some(duration_of_time);
        self
    }

    /// Adds the duration of time to wait when partway reading a request before
    /// the connection is aborted and closed.
    #[must_use]
    pub const fn read_request(mut self, duration_of_time: Duration) -> Self {
        self.0.timeouts.read_request = Some(duration_of_time);
        self
    }

    /// Adds the duration of time to wait when writing the response before
    /// the connection is aborted and closed.
    #[must_use]
    pub const fn write(mut self, duration_of_time: Duration) -> Self {
        self.0.timeouts.write = Some(duration_of_time);
        self
    }

    /// Sets a parameter to keep the connection alive after
    /// the response has been sent, allowing the client to make further requests
    /// on the same TCP connection. This should only be called if
    /// multiple sockets are handling HTTP connections to avoid a single client
    /// hogging the connection and preventing other clients
    /// from making requests.
    #[must_use]
    pub const fn keep_connection_alive(self) -> Self {
        Self(self.0.keep_connection_alive())
    }

    /// Sets a parameter to close the connection after
    /// the response has been sent, i.e. each TCP connection serves
    /// a single request.
    /// This is the default, but allows the configuration to be more explicit.
    #[must_use]
    pub const fn close_connection_after_response(self) -> Self {
        Self(self.0.close_connection_after_response())
    }

    #[inline]
    pub(crate) fn config(self) -> &'static Config<Duration> {
        mk_static!(Config<Duration>, self.0)
    }
}

/// A server.
pub struct Server<
    const WEB_TASK_POOL_SIZE: usize,
    PR: PathRouter<(), NoPathParameters> + Send + 'static,
> {
    device: Device<PR>,
    config: ServerConfig,
    mdns: Mdns,
    port: u16,
}

impl<const WEB_TASK_POOL_SIZE: usize, PR: PathRouter<(), NoPathParameters> + Send + 'static>
    Server<WEB_TASK_POOL_SIZE, PR>
{
    /// Creates a [`Server`].
    pub const fn new(device: Device<PR>, config: ServerConfig, mdns: Mdns) -> Self {
        Self {
            device,
            config,
            mdns,
            port: 80,
        }
    }

    /// Sets server port.
    #[must_use]
    pub const fn port(mut self, port: u16) -> Self {
        self.port = port;
        self
    }

    /// Runs the server.
    ///
    /// # Errors
    pub async fn run(self, stack: Stack<'static>, spawner: Spawner) -> Result<()> {
        // Retrieve IP.
        let ip = get_ip(stack).await;
        info!("Server at IP Address: {ip}");

        // Run mdns.
        self.mdns.run(stack, self.port, spawner)?;

        // Get server configuration.
        let config = self.config.config();

        // TODO: Find a new strategy to obtain a static reference
        // using static_cell. Probably, a new Rust version is necessary.
        let internal_router: &'static Router<PR> =
            unsafe { core::mem::transmute(&self.device.router) };

        for id in 0..WEB_TASK_POOL_SIZE.max(1) {
            let server_task: ServerTaskFn = Box::new(move || {
                Box::pin(internal_server_run(
                    id,
                    stack,
                    internal_router,
                    config,
                    self.port,
                ))
            });

            match WEB_TASK_POOL_SIZE.max(1) {
                1 => {
                    spawner.spawn(web_task1(server_task))?;
                }
                2 => {
                    spawner.spawn(web_task2(server_task))?;
                }
                3 => {
                    spawner.spawn(web_task3(server_task))?;
                }
                4 => {
                    spawner.spawn(web_task4(server_task))?;
                }
                5 => {
                    spawner.spawn(web_task5(server_task))?;
                }
                6 => {
                    spawner.spawn(web_task6(server_task))?;
                }
                7 => {
                    spawner.spawn(web_task7(server_task))?;
                }
                _ => {
                    spawner.spawn(web_task8(server_task))?;
                }
            }
        }

        Ok(())
    }
}
