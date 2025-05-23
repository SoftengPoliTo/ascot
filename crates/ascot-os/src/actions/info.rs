use core::future::Future;

use ascot::device::DeviceInfo;
use ascot::response::{InfoResponse as AscotInfoResponse, ResponseKind};
use ascot::route::Route;

use axum::{
    extract::Json,
    handler::Handler,
    http::StatusCode,
    response::{IntoResponse, Response},
};

use serde::{Deserialize, Serialize};

use super::{DeviceAction, error::ErrorResponse};

/// Informative response.
///
/// This response provides economy and energy information of a device.
#[derive(Serialize, Deserialize)]
pub struct InfoResponse(AscotInfoResponse);

impl InfoResponse {
    /// Creates an [`InfoResponse`].
    #[must_use]
    pub const fn new(info: DeviceInfo) -> Self {
        Self(AscotInfoResponse::new(info))
    }
}

impl IntoResponse for InfoResponse {
    fn into_response(self) -> Response {
        (StatusCode::OK, Json(self.0)).into_response()
    }
}

mod private {
    pub trait InfoTypeName<Args> {}
}

impl<F, Fut> private::InfoTypeName<()> for F
where
    F: FnOnce() -> Fut,
    Fut: Future<Output = Result<InfoResponse, ErrorResponse>> + Send,
{
}

macro_rules! impl_info_type_name {
    (
        [$($ty:ident),*], $($last:ident)?
    ) => {
        impl<F, Fut, M, $($ty,)* $($last)?> private::InfoTypeName<(M, $($ty,)* $($last)?)> for F
        where
            F: FnOnce($($ty,)* $($last)?) -> Fut,
            Fut: Future<Output = Result<InfoResponse, ErrorResponse>> + Send,
            {
            }
    };
}
super::all_the_tuples!(impl_info_type_name);

/// Creates a stateful [`DeviceAction`] with an [`InfoResponse`].
pub fn info_stateful<H, T, S, I>(route: Route, handler: H) -> impl FnOnce(S, I) -> DeviceAction
where
    H: Handler<T, S> + private::InfoTypeName<T>,
    T: 'static,
    S: Clone + Send + Sync + 'static,
    I: 'static,
{
    move |state: S, _: I| DeviceAction::stateful(route, ResponseKind::Info, handler, state)
}

/// Creates a stateless [`DeviceAction`] with an [`InfoResponse`].
pub fn info_stateless<H, T, S, I>(route: Route, handler: H) -> impl FnOnce(S, I) -> DeviceAction
where
    H: Handler<T, ()> + private::InfoTypeName<T>,
    T: 'static,
    S: Clone + Send + Sync + 'static,
    I: 'static,
{
    move |_state: S, _: I| DeviceAction::stateless(route, ResponseKind::Info, handler)
}
