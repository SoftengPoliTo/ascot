use std::collections::HashMap;
use std::fmt::Write;
use std::future::Future;

use serde::Serialize;

use tracing::error;

use ascot::device::DeviceEnvironment;
use ascot::hazards::Hazards;
use ascot::parameters::ParametersData;
use ascot::response::ResponseKind;
use ascot::route::{RestKind, RouteConfig, RouteConfigs};

use crate::error::Error;
use crate::parameters::{Parameters, convert_to_parameter_value};
use crate::response::{
    InfoResponseParser, OkResponseParser, Response, SerialResponseParser, StreamResponse,
};

fn slash_end(s: &str) -> &str {
    if s.len() > 1 && s.ends_with('/') {
        &s[..s.len() - 1]
    } else {
        s
    }
}

fn slash_start(s: &str) -> &str {
    if s.len() > 1 && s.starts_with('/') {
        &s[1..]
    } else {
        s
    }
}

fn slash_start_end(s: &str) -> &str {
    slash_start(slash_end(s))
}

#[derive(Debug, PartialEq)]
struct RequestData {
    request: String,
    parameters: HashMap<String, String>,
}

impl RequestData {
    const fn new(request: String, parameters: HashMap<String, String>) -> Self {
        Self {
            request,
            parameters,
        }
    }
}

pub(crate) fn create_requests(
    route_configs: RouteConfigs,
    complete_address: &str,
    main_route: &str,
    environment: DeviceEnvironment,
) -> HashMap<String, Request> {
    route_configs
        .into_iter()
        .map(|route| {
            (
                route.data.name.to_string(),
                Request::new(complete_address, main_route, environment, route),
            )
        })
        .collect()
}

/// Request information.
pub struct RequestInfo<'device> {
    /// Route name.
    pub route: &'device str,
    /// Route description.
    pub description: Option<&'device str>,
    /// Rest kind.
    pub rest_kind: RestKind,
    /// Route hazards.
    pub hazards: &'device Hazards,
    /// Parameters data.
    pub parameters_data: &'device ParametersData,
    /// Response kind.
    pub response_kind: ResponseKind,
}

impl<'device> RequestInfo<'device> {
    pub(crate) fn new(route: &'device str, request: &'device Request) -> Self {
        Self {
            route,
            description: request.description.as_deref(),
            rest_kind: request.kind,
            hazards: &request.hazards,
            parameters_data: &request.parameters_data,
            response_kind: request.response_kind,
        }
    }
}

/// A device request.
///
/// It defines a request to be sent to a device.
///
/// A request can be plain, hence without any input parameter, or with some
/// parameters which are used to personalize device operations.
#[derive(Debug, PartialEq, Serialize)]
pub struct Request {
    pub(crate) kind: RestKind,
    pub(crate) hazards: Hazards,
    pub(crate) route: String,
    pub(crate) description: Option<String>,
    pub(crate) parameters_data: ParametersData,
    pub(crate) response_kind: ResponseKind,
    pub(crate) device_environment: DeviceEnvironment,
}

impl Request {
    /// Returns an immutable reference to request [`Hazards`].
    #[must_use]
    pub fn hazards(&self) -> &Hazards {
        &self.hazards
    }

    /// Returns a request [`RestKind`].
    #[must_use]
    pub fn kind(&self) -> RestKind {
        self.kind
    }

    /// Returns an immutable reference to [`ParametersData`] associated with
    /// a request.
    ///
    /// If [`None`], the request **does not** contain any [`ParametersData`].
    #[must_use]
    pub fn parameters_data(&self) -> Option<&ParametersData> {
        self.parameters_data
            .is_empty()
            .then_some(&self.parameters_data)
    }

    pub(crate) fn new(
        address: &str,
        main_route: &str,
        device_environment: DeviceEnvironment,
        route_config: RouteConfig,
    ) -> Self {
        let kind = route_config.rest_kind;
        let route = format!(
            "{}/{}/{}",
            slash_end(address),
            slash_start_end(main_route),
            slash_start_end(&route_config.data.name)
        );
        let hazards = route_config.data.hazards;
        let parameters_data = route_config.data.parameters;
        let response_kind = route_config.response_kind;

        Self {
            kind,
            hazards,
            route,
            description: route_config.data.description.map(|s| s.to_string()),
            parameters_data,
            response_kind,
            device_environment,
        }
    }

    pub(crate) async fn retrieve_response<F, Fut>(
        &self,
        skip: bool,
        retrieve_response: F,
    ) -> Result<Response, Error>
    where
        F: FnOnce() -> Fut,
        Fut: Future<Output = Result<reqwest::Response, Error>>,
    {
        if skip {
            return Ok(Response::Skipped);
        }

        let response = retrieve_response().await?;

        Ok(match self.response_kind {
            ResponseKind::Ok => Response::OkBody(OkResponseParser::new(response)),
            ResponseKind::Serial => Response::SerialBody(SerialResponseParser::new(response)),
            ResponseKind::Info => Response::InfoBody(InfoResponseParser::new(response)),
            ResponseKind::Stream => Response::StreamBody(StreamResponse::new(response)),
        })
    }

    pub(crate) async fn plain_send(&self) -> Result<reqwest::Response, Error> {
        let request_data =
            self.request_data(|| self.axum_get_plain(), || self.create_params_plain());

        self.parameters_send(request_data).await
    }

    pub(crate) async fn create_response(
        &self,
        parameters: &Parameters<'_>,
    ) -> Result<reqwest::Response, Error> {
        let request_data = self.create_request(parameters)?;
        self.parameters_send(request_data).await
    }

    async fn parameters_send(&self, request_data: RequestData) -> Result<reqwest::Response, Error> {
        let RequestData {
            request,
            parameters,
        } = request_data;

        let client = reqwest::Client::new();

        Ok(match self.kind {
            RestKind::Get => client.get(request).send(),
            RestKind::Post => client.post(request).json(&parameters).send(),
            RestKind::Put => client.put(request).json(&parameters).send(),
            RestKind::Delete => client.delete(request).json(&parameters).send(),
        }
        .await?)
    }

    fn request_data<A, F>(&self, axum_get: A, params: F) -> RequestData
    where
        A: FnOnce() -> String,
        F: FnOnce() -> HashMap<String, String>,
    {
        let request =
            if self.kind == RestKind::Get && self.device_environment == DeviceEnvironment::Os {
                axum_get()
            } else {
                self.route.to_string()
            };

        let parameters = params();

        RequestData::new(request, parameters)
    }

    fn create_request(&self, parameters: &Parameters) -> Result<RequestData, Error> {
        // Check parameters.
        parameters.check_parameters(&self.parameters_data)?;

        Ok(self.request_data(
            || self.axum_get(parameters),
            || self.create_params(parameters),
        ))
    }

    fn axum_get_plain(&self) -> String {
        let mut route = self.route.to_string();
        for (_, parameter_kind) in &self.parameters_data {
            let Some(value) = convert_to_parameter_value(parameter_kind) else {
                // TODO: Skip bytes stream
                continue;
            };
            // TODO: Consider returning `Option<String>`
            if let Err(e) = write!(route, "/{}", value.as_string()) {
                error!("Error in adding a path to a route : {e}");
                break;
            }
        }
        route
    }

    fn create_params_plain(&self) -> HashMap<String, String> {
        let mut params = HashMap::new();
        for (name, parameter_kind) in &self.parameters_data {
            let Some(value) = convert_to_parameter_value(parameter_kind) else {
                // FIXME: Skip bytes stream
                continue;
            };
            params.insert(name.to_string(), value.as_string());
        }
        params
    }

    // Axum parameters: hello/{{1}}/{{2}}
    //                  hello/0.5/1
    fn axum_get(&self, parameters: &Parameters) -> String {
        let mut route = String::from(&self.route);
        for (name, parameter_kind) in &self.parameters_data {
            let value = if let Some(value) = parameters.get(name) {
                value.as_string()
            } else {
                let Some(value) = convert_to_parameter_value(parameter_kind) else {
                    // FIXME: Skip bytes stream
                    continue;
                };
                value.as_string()
            };
            // TODO: Consider returning `Option<String>`
            if let Err(e) = write!(route, "/{value}") {
                error!("Error in adding a path to a route : {e}");
                break;
            }
        }

        route
    }

    fn create_params(&self, parameters: &Parameters<'_>) -> HashMap<String, String> {
        let mut params = HashMap::new();
        for (name, parameter_kind) in &self.parameters_data {
            let (name, value) = if let Some(value) = parameters.get(name) {
                (name, value.as_string())
            } else {
                let Some(value) = convert_to_parameter_value(parameter_kind) else {
                    // FIXME: Skip bytes stream
                    continue;
                };
                (name, value.as_string())
            };
            params.insert(name.to_string(), value);
        }
        params
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use ascot::device::DeviceEnvironment;
    use ascot::hazards::{Hazard, Hazards};
    use ascot::parameters::{ParameterKind, Parameters as AscotParameters, ParametersData};
    use ascot::route::{RestKind, Route, RouteConfig};

    use crate::parameters::{Parameters, parameter_error};

    use super::{Request, RequestData, ResponseKind};

    const ADDRESS_ROUTE: &str = "http://ascot.local/";
    const ADDRESS_ROUTE_WITHOUT_SLASH: &str = "http://ascot.local/";
    const COMPLETE_ROUTE: &str = "http://ascot.local/light/route";

    fn plain_request(route: Route, kind: RestKind, hazards: Hazards) {
        let route = route.serialize_data();
        let description = route
            .data
            .description
            .as_ref()
            .map(std::string::ToString::to_string);

        let request = Request::new(ADDRESS_ROUTE, "light/", DeviceEnvironment::Os, route);

        assert_eq!(
            request,
            Request {
                kind,
                hazards,
                route: COMPLETE_ROUTE.into(),
                description,
                parameters_data: ParametersData::new(),
                response_kind: ResponseKind::Ok,
                device_environment: DeviceEnvironment::Os,
            }
        );
    }

    fn request_with_parameters(route: Route, kind: RestKind, hazards: &Hazards) {
        let route = route
            .with_parameters(
                AscotParameters::new()
                    .rangeu64_with_default("rangeu64", (0, 20, 1), 5)
                    .rangef64("rangef64", (0., 20., 0.1)),
            )
            .serialize_data();
        let description = route
            .data
            .description
            .as_ref()
            .map(std::string::ToString::to_string);

        let parameters_data = ParametersData::new()
            .insert(
                "rangeu64".into(),
                ParameterKind::RangeU64 {
                    min: 0,
                    max: 20,
                    step: 1,
                    default: 5,
                },
            )
            .insert(
                "rangef64".into(),
                ParameterKind::RangeF64 {
                    min: 0.,
                    max: 20.,
                    step: 0.1,
                    default: 0.,
                },
            );

        let request = Request::new(ADDRESS_ROUTE, "light/", DeviceEnvironment::Os, route);

        assert_eq!(
            request,
            Request {
                kind,
                hazards: hazards.clone(),
                route: COMPLETE_ROUTE.into(),
                description,
                parameters_data,
                response_kind: ResponseKind::Ok,
                device_environment: DeviceEnvironment::Os,
            }
        );

        // Non-existent parameter.
        assert_eq!(
            request.create_request(Parameters::new().u64("wrong", 0)),
            Err(parameter_error("`wrong` does not exist".into()))
        );

        // Wrong parameter type.
        assert_eq!(
            request.create_request(Parameters::new().f64("rangeu64", 0.)),
            Err(parameter_error("`rangeu64` must be of type `u64`".into()))
        );

        let mut parameters = HashMap::with_capacity(2);
        parameters.insert("rangeu64".into(), "3".into());
        parameters.insert("rangef64".into(), "0".into());

        assert_eq!(
            request.create_request(Parameters::new().u64("rangeu64", 3)),
            Ok(RequestData {
                request: if kind == RestKind::Get {
                    format!("{COMPLETE_ROUTE}/3/0")
                } else {
                    COMPLETE_ROUTE.into()
                },
                parameters,
            })
        );
    }

    fn request_builder(
        route: &str,
        main_route: &str,
        device_environment: DeviceEnvironment,
        route_config: RouteConfig,
    ) {
        assert_eq!(
            Request::new(route, main_route, device_environment, route_config),
            Request {
                kind: RestKind::Put,
                hazards: Hazards::new(),
                route: COMPLETE_ROUTE.into(),
                description: None,
                parameters_data: ParametersData::new(),
                response_kind: ResponseKind::Ok,
                device_environment: DeviceEnvironment::Os,
            }
        );
    }

    #[test]
    fn check_request_builder() {
        let route = Route::put("/route").serialize_data();
        let environment = DeviceEnvironment::Os;

        request_builder(ADDRESS_ROUTE, "light/", environment, route.clone());
        request_builder(ADDRESS_ROUTE_WITHOUT_SLASH, "light", environment, route);
    }

    #[test]
    fn create_plain_get_request() {
        let route = Route::get("/route").description("A GET route.");
        plain_request(route, RestKind::Get, Hazards::new());
    }

    #[test]
    fn create_plain_post_request() {
        let route = Route::post("/route").description("A POST route.");
        plain_request(route, RestKind::Post, Hazards::new());
    }

    #[test]
    fn create_plain_put_request() {
        let route = Route::put("/route").description("A PUT route.");
        plain_request(route, RestKind::Put, Hazards::new());
    }

    #[test]
    fn create_plain_delete_request() {
        let route = Route::delete("/route").description("A DELETE route.");
        plain_request(route, RestKind::Delete, Hazards::new());
    }

    #[test]
    fn create_plain_get_request_with_hazards() {
        let hazards = Hazards::new()
            .insert(Hazard::FireHazard)
            .insert(Hazard::AirPoisoning);
        plain_request(
            Route::get("/route")
                .description("A GET route.")
                .with_hazards(hazards.clone()),
            RestKind::Get,
            hazards,
        );
    }

    #[test]
    fn create_get_request_with_parameters() {
        request_with_parameters(
            Route::get("/route").description("A GET route."),
            RestKind::Get,
            &Hazards::new(),
        );
    }

    #[test]
    fn create_post_request_with_parameters() {
        let route = Route::post("/route").description("A POST route.");
        request_with_parameters(route, RestKind::Post, &Hazards::new());
    }

    #[test]
    fn create_put_request_with_parameters() {
        let route = Route::put("/route").description("A PUT route.");
        request_with_parameters(route, RestKind::Put, &Hazards::new());
    }

    #[test]
    fn create_delete_request_with_parameters() {
        let route = Route::delete("/route").description("A DELETE route.");
        request_with_parameters(route, RestKind::Delete, &Hazards::new());
    }

    #[test]
    fn create_get_request_with_hazards_and_parameters() {
        let hazards = Hazards::new()
            .insert(Hazard::FireHazard)
            .insert(Hazard::AirPoisoning);

        request_with_parameters(
            Route::get("/route")
                .description("A GET route.")
                .with_hazards(hazards.clone()),
            RestKind::Get,
            &hazards,
        );
    }
}
