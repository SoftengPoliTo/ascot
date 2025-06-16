use core::future::Future;

use ascot::response::{OkResponse as AscotOkResponse, ResponseKind};
use ascot::route::Route;

use axum::{
    extract::Json,
    handler::Handler,
    http::StatusCode,
    response::{IntoResponse, Response},
};

use serde::{Deserialize, Serialize};

use super::{DeviceAction, MandatoryAction, error::ErrorResponse};

/// An `Ok` response sends a boolean to notify a receiver that a device action
/// has terminated correctly.
#[derive(Serialize, Deserialize)]
pub struct OkResponse(AscotOkResponse);

impl OkResponse {
    /// Creates an [`OkResponse`].
    #[must_use]
    #[inline]
    pub fn ok() -> Self {
        Self(AscotOkResponse::ok())
    }
}

impl IntoResponse for OkResponse {
    fn into_response(self) -> Response {
        (StatusCode::OK, Json(self.0)).into_response()
    }
}

mod private {
    pub trait OkTypeName<Args> {}
}

impl<F, Fut> private::OkTypeName<()> for F
where
    F: FnOnce() -> Fut,
    Fut: Future<Output = Result<OkResponse, ErrorResponse>> + Send,
{
}

macro_rules! impl_ok_type_name {
    (
        [$($ty:ident),*], $($last:ident)?
    ) => {
        impl<F, Fut, M, $($ty,)* $($last)?> private::OkTypeName<(M, $($ty,)* $($last)?)> for F
        where
            F: FnOnce($($ty,)* $($last)?) -> Fut,
            Fut: Future<Output = Result<OkResponse, ErrorResponse>> + Send,
            {
            }
    };
}
super::all_the_tuples!(impl_ok_type_name);

/// Creates a mandatory stateful [`DeviceAction`] with an [`OkResponse`].
#[inline]
pub fn mandatory_ok_stateful<H, T, S>(handler: H) -> impl FnOnce(Route, S) -> MandatoryAction<false>
where
    H: Handler<T, S> + private::OkTypeName<T>,
    T: 'static,
    S: Clone + Send + Sync + 'static,
{
    move |route: Route, state: S| {
        MandatoryAction::new(DeviceAction::stateful(
            route,
            ResponseKind::Ok,
            handler,
            state,
        ))
    }
}

/// Creates a stateful [`DeviceAction`] with an [`OkResponse`].
#[inline]
pub fn ok_stateful<H, T, S>(route: Route, handler: H) -> impl FnOnce(S) -> DeviceAction
where
    H: Handler<T, S> + private::OkTypeName<T>,
    T: 'static,
    S: Clone + Send + Sync + 'static,
{
    move |state: S| DeviceAction::stateful(route, ResponseKind::Ok, handler, state)
}

/// Creates a mandatory stateless [`DeviceAction`] with an [`OkResponse`].
#[inline]
pub fn mandatory_ok_stateless<H, T, S>(
    handler: H,
) -> impl FnOnce(Route, S) -> MandatoryAction<false>
where
    H: Handler<T, ()> + private::OkTypeName<T>,
    T: 'static,
    S: Clone + Send + Sync + 'static,
{
    move |route: Route, _state: S| {
        MandatoryAction::new(DeviceAction::stateless(route, ResponseKind::Ok, handler))
    }
}

/// Creates a stateless [`DeviceAction`] with an [`OkResponse`].
#[inline]
pub fn ok_stateless<H, T, S>(route: Route, handler: H) -> impl FnOnce(S) -> DeviceAction
where
    H: Handler<T, ()> + private::OkTypeName<T>,
    T: 'static,
    S: Clone + Send + Sync + 'static,
{
    move |_state: S| DeviceAction::stateless(route, ResponseKind::Ok, handler)
}
