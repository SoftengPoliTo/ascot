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
    MethodHandler, MethodNotAllowed, MethodRouter, NotFound, RequestHandler, RequestHandlerFunction
};
use picoserve::{AppBuilder, AppRouter};
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

pub struct MyRouter<
    PR: PathRouter<State, CurrentPathParameters>,
    State = (),
    CurrentPathParameters = picoserve::routing::NoPathParameters,
> {
    routes: Vec<Route>,
    router: Router<PR, State, CurrentPathParameters>,
}

impl MyRouter<picoserve::routing::NotFound, (), picoserve::routing::NoPathParameters> {
    pub fn new() -> Self {
        Self {
            routes: Vec::new(),
            router: Router::new(),
        }
    }
}

impl<PR: PathRouter<State, CurrentPathParameters>, State, CurrentPathParameters>
    MyRouter<PR, State, CurrentPathParameters>
{
    pub fn route<PD: picoserve::routing::PathDescription<CurrentPathParameters>>(
        self,
        path_description: PD,
        handler: impl picoserve::routing::MethodHandler<State, PD::Output>,
    ) -> MyRouter<impl PathRouter<State, CurrentPathParameters>, State, CurrentPathParameters> {
        let new_router = self.router.route(path_description, handler);

        MyRouter {
            routes: self.routes,
            router: new_router,
        }
    }

    pub fn router(
        self,
    ) -> Router<impl PathRouter<State, CurrentPathParameters>, State, CurrentPathParameters> {
        self.router
    }
}

// TODO: Fix
impl<PR: PathRouter<(), picoserve::routing::NoPathParameters>> AppBuilder for MyRouter<PR> {
    type PathRouter = impl picoserve::routing::PathRouter;

    fn build_app(self) -> picoserve::Router<Self::PathRouter> {
        self.router
    }
}

pub trait MyAppBuilder {
    type PathRouter: picoserve::routing::PathRouter;

    fn build_app(self) -> MyRouter<Self::PathRouter>;
}

pub trait MyAppWithStateBuilder {
    type State;
    type PathRouter: picoserve::routing::PathRouter<Self::State>;

    fn build_app(self) -> MyRouter<Self::PathRouter, Self::State>;
}

impl<T: MyAppBuilder> MyAppWithStateBuilder for T {
    type State = ();
    type PathRouter = <Self as MyAppBuilder>::PathRouter;

    fn build_app(self) -> MyRouter<Self::PathRouter, Self::State> {
        <Self as MyAppBuilder>::build_app(self)
    }
}

pub type MyAppRouter<Props> =
    MyRouter<<Props as MyAppWithStateBuilder>::PathRouter, <Props as MyAppWithStateBuilder>::State>;

pub async fn listen_and_serve<P: picoserve::routing::PathRouter<()>>(
    task_id: impl picoserve::LogDisplay,
    app: MyRouter<P, ()>,
    config: &picoserve::Config<embassy_time::Duration>,
    stack: embassy_net::Stack<'_>,
    port: u16,
    tcp_rx_buffer: &mut [u8],
    tcp_tx_buffer: &mut [u8],
    http_buffer: &mut [u8],
) -> ! {
    picoserve::listen_and_serve_with_state(
        task_id,
        &app.router(),
        config,
        stack,
        port,
        tcp_rx_buffer,
        tcp_tx_buffer,
        http_buffer,
        &(),
    )
    .await
}