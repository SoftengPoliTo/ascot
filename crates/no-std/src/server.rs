use core::fmt::Display;
use core::{future::Future, marker::PhantomData, pin::Pin};

use alloc::string::String;
// src/server.rs
use alloc::vec::Vec;
use alloc::{boxed::Box, vec};

use embassy_executor::Spawner;
use embassy_net::Stack;
use embassy_time::Duration;
use embedded_io_async::Read;
use picoserve::response::IntoResponse;
use picoserve::routing::{
    MethodHandler, MethodNotAllowed, MethodRouter, NotFound, RequestHandler, RequestHandlerFunction,
};
use picoserve::AppBuilder;
use picoserve::{
    request::Request,
    response::ResponseWriter,
    routing::{IntoPathParameterList, PathDescription, PathRouter, Router},
    ResponseSent,
};

use crate::mk_static;

// pub fn foo<State, PathParameters, T, Handler: RequestHandlerFunction<State, PathParameters, T>, R: IntoResponse>(
//     handler: Handler,
// ) -> R {
//     let r = handler();
//     r
// }

// fn example(routes: Vec<bool>) -> Router<impl PathRouter> {

//     Get(&'static str, Box<dyn RequestHandlerFunction<State, (), String> + Send + Sync>),
//     Post(&'static str, Box<dyn RequestHandlerFunction<State, (), String> + Send + Sync>),
//     // Put, Delete ecc.
// }

enum Method {
    Get,
    Post,
    Put,
    Delete,
}

struct Route {
    method: Method,
    path: &'static str,
}

impl Route {
    pub fn get(path: &'static str) -> Self {
        Self {
            method: Method::Get,
            path
        }
    }


    pub fn post(path: &'static str) -> Self {
        Self {
            method: Method::Get,
            path
        }
    }

    pub fn put(path: &'static str) -> Self {
        Self {
            method: Method::Put,
            path
        }
    }

    pub fn delete(path: &'static str) -> Self {
        Self {
            method: Method::Delete,
            path
        }
    }
    
}

pub struct Server<
    PR: PathRouter<State, CurrentPathParameters>,
    State = (),
    CurrentPathParameters = picoserve::routing::NoPathParameters,
> {
    routes: Vec<Route>,
    router: Router<PR, State, CurrentPathParameters>,
}

impl Server<picoserve::routing::NotFound, (), picoserve::routing::NoPathParameters> {
    pub fn new() -> Self {
        Self {
            routes: Vec::new(),
            router: Router::new(),
        }
    }
}

impl<PR: PathRouter<State, CurrentPathParameters>, State, CurrentPathParameters>
    Server<PR, State, CurrentPathParameters>
{
    pub fn route<PD: picoserve::routing::PathDescription<CurrentPathParameters>>(
        self,
        path_description: PD,
        handler: impl picoserve::routing::MethodHandler<State, PD::Output>,
    ) -> Server<impl PathRouter<State, CurrentPathParameters>, State, CurrentPathParameters> {
        let new_router = self.router.route(path_description, handler);

        Server {
            routes: self.routes,
            router: new_router,
        }
    }
}

// pub fn add_get<State, PathParameters, T, Handler: RequestHandlerFunction<State, PathParameters, T>>(
//     handler: Handler
// ) -> MethodRouter<
//     impl RequestHandler<State, PathParameters>,
//     MethodNotAllowed,
//     MethodNotAllowed,
//     MethodNotAllowed,
// >{
//     get(handler)
// }

fn f() {
    let s = Server::new().route("path", crate::get(|| async move { "Hello" }));
}

// struct Server<P: picoserve::routing::PathRouter<()>> {
//     routes: Vec<Route>,
//     router: Router<P, ()>
// }

// impl<P: picoserve::routing::PathRouter<()>> AppBuilder for Server<P> {
//     type PathRouter = impl picoserve::routing::PathRouter;

//     fn build_app(self) -> Router<Self::PathRouter> {
//         picoserve::Router::new()
//     }
// }

// impl<State, CurrentPathParameters> Server<Router<NotFound, State, CurrentPathParameters>> {
//     pub fn new() -> Self {
//         Self {
//             routes: vec![],
//             router: Router::new()
//         }
//     }
// }

// impl Server {
//     pub fn new() -> Router<<Server as AppBuilder>::PathRouter> {
//         let s = Self { routes: vec![] };
//         s.build_app()
//     }

//     pub fn foo<
//         State,
//         PathParameters,
//         T,
//         Handler: RequestHandlerFunction<State, PathParameters, T>,
//         R: IntoResponse,
//         P: picoserve::routing::PathRouter<()>
//     >(
//         app: Router<P, ()>,
//         handler: Handler,
//     ) -> Router<PathRouter> {
//         app.route("path", handler)
//     }
// }

#[embassy_executor::task]
async fn server(with_routes: bool, stack: embassy_net::Stack<'static>) {
    let port = 80;
    let mut tcp_rx_buffer = [0; 1024];
    let mut tcp_tx_buffer = [0; 1024];
    let mut http_buffer = [0; 2048];

    let config = picoserve::Config::new(picoserve::Timeouts {
        start_read_request: Some(Duration::from_secs(5)),
        persistent_start_read_request: Some(Duration::from_secs(1)),
        read_request: Some(Duration::from_secs(1)),
        write: Some(Duration::from_secs(1)),
    })
    .keep_connection_alive();

    if with_routes {
        let router_with_routes = Router::new();
        let router_with_routes: Router<_, (), _> = router_with_routes
            .route("hello", crate::get(|| async move { "Hello" }))
            .route("world", crate::get(|| async move { "World" }));

        web_task(0, stack, router_with_routes, config).await;
    } else {
        let empty_router = Router::new();

        web_task(0, stack, empty_router, config).await;
    }
}

async fn web_task<P: picoserve::routing::PathRouter<()>>(
    id: usize,
    stack: embassy_net::Stack<'static>,
    app: Router<P, ()>,
    config: picoserve::Config<Duration>,
) -> ! {
    let port = 80;
    let mut tcp_rx_buffer = [0; 1024];
    let mut tcp_tx_buffer = [0; 1024];
    let mut http_buffer = [0; 2048];

    picoserve::listen_and_serve(
        id,
        &app,
        &config,
        stack,
        port,
        &mut tcp_rx_buffer,
        &mut tcp_tx_buffer,
        &mut http_buffer,
    )
    .await
}

// enum Method {
//     Get,
//     Post,
// }
//
// struct Route {
//     path: &'static str,
//     method: Method,
//     handler: Box<dyn Fn() -> Pin<Box<dyn Future<Output = &'static str> + Send>> + Send + Sync>,
// }
//
// Nota come specifichiamo i parametri generici con State=() e Params=()
// fn create_router_with_handlers<I>(
//     routes: I,
// ) -> Router<impl PathRouter<(), ()>, (), ()>
// where
//     I: IntoIterator<Item = Route>,
// {
//     let mut router = Router::new().route("default", post(|| async move { "ciao" }));
//     for route in routes {
//         router = match route.method {
//             Method::Get => {
//             // Qui specifichiamo che get() produce un Handler che implementa RequestHandler<(), ()>
//             router.route(route.path, get(route.handler))
//         }
//         Method::Post => {
//             // Qui idem per post()
//             router.route(route.path, post(route.handler))
//         }
//         }
//     }

//     router
// }

// fn e() -> Router<NotFound, (), ()> {
//     let router = Router::new();

//     router
// }

// fn box_handler<Fut, F>(
//     handler: F,
// ) -> Box<dyn Fn() -> Pin<Box<dyn Future<Output = &'static str> + Send>> + Send + Sync>
// where
//     F: Fn() -> Fut + Send + Sync + 'static,
//     Fut: Future<Output = &'static str> + Send + 'static,
// {
//     Box::new(move || Box::pin(handler()))
// }

// fn example() {
//     let routes = vec![
//         Route {
//             path: "hello",
//             method: Method::Get,
//             handler: box_handler(|| async { "Hello GET" }),
//         },
//         Route {
//             path: "submit",
//             method: Method::Post,
//             handler: box_handler(|| async { "Hello POST" }),
//         },
//     ];

//     let router = create_router_with_handlers(routes);

// }

fn create_router_with_handler<H, F>(handler: H) -> Router<impl PathRouter<()>, ()>
where
    H: Fn() -> F + Copy,
    F: Future<Output = &'static str>,
{
    Router::new().route("get", crate::get(handler))
}

fn example_with_one_handler() {
    let router = create_router_with_handler(|| async { "Hello from closure" });
}

// fn add_post<R>(
//     router: picoserve::Router<R, ()>
// ) -> picoserve::Router<R, ()>
// where
//     R: picoserve::routing::PathRouter<()>,
// {
//     router.route("post", post(|| async move { "POST" }))
// }

// async fn example() {
//     let mut router = create_router();
//     router = add_post(router);
// }

// pub enum HttpMethod {
//     Get,
//     Post,
//     // aggiungi altri se vuoi
// }

// pub struct Route<State, PathParameters, Fut> {
//     pub path: &'static str,
//     pub method: HttpMethod,
//     pub handler: Box<dyn Fn() -> Pin<Box<dyn Future<Output = &'static str> + Send>> + Send + Sync>,
// }

// pub struct RouteCollection<State, PathParameters, Fut> {
//     routes: Vec<Route<State, PathParameters, Fut>>,
// }

// struct AppStateless;

// impl AppBuilder for AppStateless {
//     type PathRouter = impl picoserve::routing::PathRouter;

//     fn build_app(self) -> picoserve::Router<Self::PathRouter> {
//         picoserve::Router::new().route("/hello", get(|| async move {

//         }))
//     }
// }

// pub struct S;

// pub struct AppWithState;

// impl AppWithStateBuilder for AppWithState {
//     type State = S;

//     type PathRouter = impl picoserve::routing::PathRouter<Self::State>;

//     fn build_app(self) -> picoserve::Router<Self::PathRouter, Self::State> {
//         picoserve::Router::new()
//     }
// }

// /// High-level helper for starting picoserve-based servers.
// ///
// /// NOTE: The caller must provide a `&'static AppRouter<App>` constructed
// /// via `picoserve::Router::new().route(...).route(...)` (i.e. the Router is
// /// built at compile-time / code-time). See usage examples after the file.
// pub struct WebServer;

// impl WebServer {
//     /// Create a default config (leaked to `'static`). Uses Box::leak which
//     /// requires `alloc` (present in your firmware crates when using esp_alloc).
//     pub fn default_config() -> &'static picoserve::Config<Duration> {
//         Box::leak(Box::new(
//             picoserve::Config::new(picoserve::Timeouts {
//                 start_read_request: Some(Duration::from_secs(5)),
//                 persistent_start_read_request: Some(Duration::from_secs(1)),
//                 read_request: Some(Duration::from_secs(1)),
//                 write: Some(Duration::from_secs(1)),
//             })
//             .keep_connection_alive(),
//         ))
//     }

//     /// Start a stateless server with `POOL` workers. `app` must be a &'static AppRouter<App>.
//     /// If `config` is None, we use `default_config()`.
//     pub async fn start_stateless<const POOL: usize>(
//         stack: Stack<'static>,
//         app: &'static AppRouter<AppStateless>,
//         config: Option<&'static picoserve::Config<Duration>>,
//         spawner: Spawner,
//     ) {
//         let config = config.unwrap_or_else(|| Self::default_config());

//         // spawn POOL workers; each worker runs `picoserve::listen_and_serve`
//         for id in 0..POOL {
//             // web_worker::<App>(...) is monomorphized here for the concrete App
//             // spawner.must_spawn(web_worker::<App>(id, stack, app, config));
//             spawner.must_spawn(web_worker_stateless(id, stack, app, config));
//         }
//     }

//     // Start a stateful server (App implements AppWithStateBuilder). `state` is the &'static state.
//     pub async fn start_stateful<const POOL: usize>(
//         stack: Stack<'static>,
//         app: &'static AppRouter<AppWithState>,
//         config: Option<&'static picoserve::Config<Duration>>,
//         state: &'static <AppWithState as AppWithStateBuilder>::State,
//         spawner: Spawner,
//     ) {
//         let config = config.unwrap_or_else(|| Self::default_config());

//         for id in 0..POOL {
//             spawner
//                 .spawn(web_worker_stateful(id, stack, app, config, state))
//                 .ok();
//         }
//     }
// }

// /// Per-worker task (stateless)
// #[embassy_executor::task]
// async fn web_worker_stateless(
//     id: usize,
//     stack: Stack<'static>,
//     app: &'static AppRouter<AppStateless>,
//     config: &'static picoserve::Config<Duration>,
// ) -> ! {
//     let mut tcp_rx_buffer = [0u8; 1024];
//     let mut tcp_tx_buffer = [0u8; 1024];
//     let mut http_buffer = [0u8; 2048];

//     // This call typically runs forever handling requests.
//     picoserve::listen_and_serve(
//         id,
//         app,
//         config,
//         stack,
//         80,
//         &mut tcp_rx_buffer,
//         &mut tcp_tx_buffer,
//         &mut http_buffer,
//     )
//     .await
// }

// /// Per-worker task (stateful)
// #[embassy_executor::task]
// async fn web_worker_stateful(
//     id: usize,
//     stack: Stack<'static>,
//     app: &'static AppRouter<AppWithState>,
//     config: &'static picoserve::Config<Duration>,
//     state: &'static <AppWithState as AppWithStateBuilder>::State,
// ) -> ! {
//     let mut tcp_rx_buffer = [0u8; 1024];
//     let mut tcp_tx_buffer = [0u8; 1024];
//     let mut http_buffer = [0u8; 2048];

//     picoserve::listen_and_serve_with_state(
//         id,
//         app,
//         config,
//         stack,
//         80,
//         &mut tcp_rx_buffer,
//         &mut tcp_tx_buffer,
//         &mut http_buffer,
//         state,
//     )
//     .await
// }
