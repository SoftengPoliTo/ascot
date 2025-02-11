use std::collections::HashMap;

use ascot_library::device::DeviceEnvironment;
use ascot_library::hazards::Hazards;
use ascot_library::parameters::ParametersData;
use ascot_library::response::ResponseKind;
use ascot_library::route::{RestKind, RouteConfig};

use tracing::warn;

use crate::error::Error;
use crate::parameters::{convert_to_parameter_value, Parameters};
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

/// Request sender.
///
/// It sends a request to a device.
///
/// The request can be formed by the chaining of different parameters or a
/// plain one.
#[derive(Debug, PartialEq)]
pub struct RequestSender {
    pub(crate) kind: RestKind,
    pub(crate) hazards: Hazards,
    pub(crate) route: String,
    pub(crate) parameters_data: ParametersData,
    pub(crate) response_kind: ResponseKind,
    pub(crate) device_environment: DeviceEnvironment,
}

impl RequestSender {
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
            parameters_data,
            response_kind,
            device_environment,
        }
    }

    /// Sends a request to a device getting in return a [`Response`].
    pub async fn send(&self) -> Result<Response, Error> {
        Ok(match self.response_kind {
            ResponseKind::Ok => Response::Ok(OkResponseParser::new(self.plain_send().await?)),
            ResponseKind::Serial => {
                Response::Serial(SerialResponseParser::new(self.plain_send().await?))
            }
            ResponseKind::Info => Response::Info(InfoResponseParser::new(self.plain_send().await?)),
            ResponseKind::Stream => Response::Stream(StreamResponse::new(self.plain_send().await?)),
        })
    }

    /// Sends a request with determined input [`Parameters`] to a device
    /// getting in return a [`Response`].
    pub async fn send_with_params(&self, parameters: Parameters) -> Result<Response, Error> {
        if self.parameters_data.is_empty() {
            warn!("The request does not have input parameters");
            return self.send().await;
        }

        let request_data = self.create_request(&parameters)?;

        Ok(match self.response_kind {
            ResponseKind::Ok => {
                Response::Ok(OkResponseParser::new(self.inputs_send(request_data).await?))
            }
            ResponseKind::Serial => Response::Serial(SerialResponseParser::new(
                self.inputs_send(request_data).await?,
            )),
            ResponseKind::Info => Response::Info(InfoResponseParser::new(
                self.inputs_send(request_data).await?,
            )),
            ResponseKind::Stream => {
                Response::Stream(StreamResponse::new(self.inputs_send(request_data).await?))
            }
        })
    }

    /// Returns request [`Hazards`].
    #[must_use]
    pub fn hazards(&self) -> &Hazards {
        &self.hazards
    }

    /// Returns request [`RestKind`].
    #[must_use]
    pub fn kind(&self) -> RestKind {
        self.kind
    }

    /// Returns [`ParametersData`] associated with the request.
    ///
    /// If [`None`], the request does not contain [`ParametersData`].
    #[must_use]
    pub fn params(&self) -> Option<&ParametersData> {
        if self.parameters_data.is_empty() {
            None
        } else {
            Some(&self.parameters_data)
        }
    }

    async fn plain_send(&self) -> Result<reqwest::Response, Error> {
        let client = reqwest::Client::new();

        Ok(match self.kind {
            RestKind::Get => client.get(&self.route).send(),
            RestKind::Post => client.post(&self.route).send(),
            RestKind::Put => client.put(&self.route).send(),

            RestKind::Delete => client.delete(&self.route).send(),
        }
        .await?)
    }

    async fn inputs_send(&self, request_data: RequestData) -> Result<reqwest::Response, Error> {
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

    fn create_request(&self, parameters: &Parameters) -> Result<RequestData, Error> {
        // Check parameters.
        parameters.check_parameters(&self.parameters_data)?;

        let request = if self.kind == RestKind::Get {
            match self.device_environment {
                DeviceEnvironment::Os => self.axum_get(parameters),
                // The server does not accept arguments.
                DeviceEnvironment::Esp32 => self.route.to_string(),
            }
        } else {
            self.route.to_string()
        };

        Ok(RequestData::new(request, self.create_params(parameters)))
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
                    // TODO: Skip bytes stream
                    continue;
                };
                value.as_string()
            };
            route.push_str(&format!("/{value}"));
        }

        route
    }

    fn create_params(&self, parameters: &Parameters) -> HashMap<String, String> {
        let mut params = HashMap::new();
        for (name, parameter_kind) in &self.parameters_data {
            let (name, value) = if let Some(value) = parameters.get(name) {
                (name, value.as_string())
            } else {
                let Some(value) = convert_to_parameter_value(parameter_kind) else {
                    // TODO: Skip bytes stream
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
    use ascot_library::hazards::Hazard;
    use ascot_library::parameters::{
        ParameterKind, Parameters as LibraryParameters, ParametersData,
    };
    use ascot_library::route::Route;

    use crate::parameters::{parameter_error, Parameters};

    use super::{
        DeviceEnvironment, HashMap, Hazards, RequestData, RequestSender, ResponseKind, RestKind,
        RouteConfig,
    };

    const ADDRESS_ROUTE: &str = "http://ascot.local/";
    const ADDRESS_ROUTE_WITHOUT_SLASH: &str = "http://ascot.local/";
    const COMPLETE_ROUTE: &str = "http://ascot.local/light/route";

    fn plain_request(route: Route, kind: RestKind, hazards: Hazards) {
        let route = route.serialize_data();

        let request_sender =
            RequestSender::new(ADDRESS_ROUTE, "light/", DeviceEnvironment::Os, route);

        assert_eq!(
            request_sender,
            RequestSender {
                kind,
                hazards,
                route: COMPLETE_ROUTE.into(),
                parameters_data: ParametersData::new(),
                response_kind: ResponseKind::Ok,
                device_environment: DeviceEnvironment::Os,
            }
        );
    }

    fn request_with_inputs(route: Route, kind: RestKind, hazards: &Hazards) {
        let route = route
            .with_parameters(
                LibraryParameters::new()
                    .rangeu64_with_default("rangeu64", (0, 20, 1), 5)
                    .rangef64("rangef64", (0., 20., 0.1)),
            )
            .serialize_data();

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

        let request_sender =
            RequestSender::new(ADDRESS_ROUTE, "light/", DeviceEnvironment::Os, route);

        assert_eq!(
            request_sender,
            RequestSender {
                kind,
                hazards: hazards.clone(),
                route: COMPLETE_ROUTE.into(),
                parameters_data,
                response_kind: ResponseKind::Ok,
                device_environment: DeviceEnvironment::Os,
            }
        );

        // Non-existent value.
        assert_eq!(
            request_sender.create_request(&Parameters::new().u64("wrong", 0)),
            Err(parameter_error("`wrong` does not exist".into()))
        );

        // Wrong input type.
        assert_eq!(
            request_sender.create_request(&Parameters::new().f64("rangeu64", 0.)),
            Err(parameter_error("`rangeu64` must be of type `u64`".into()))
        );

        let mut parameters = HashMap::with_capacity(2);
        parameters.insert("rangeu64".into(), "3".into());
        parameters.insert("rangef64".into(), "0".into());

        assert_eq!(
            request_sender.create_request(&Parameters::new().u64("rangeu64", 3)),
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

    fn request_generator(
        route: &str,
        main_route: &str,
        device_environment: DeviceEnvironment,
        route_config: RouteConfig,
    ) {
        assert_eq!(
            RequestSender::new(route, main_route, device_environment, route_config),
            RequestSender {
                kind: RestKind::Put,
                hazards: Hazards::new(),
                route: COMPLETE_ROUTE.into(),
                parameters_data: ParametersData::new(),
                response_kind: ResponseKind::Ok,
                device_environment: DeviceEnvironment::Os,
            }
        );
    }

    #[test]
    fn check_request_generator() {
        let route = Route::put("/route").serialize_data();
        let environment = DeviceEnvironment::Os;

        request_generator(ADDRESS_ROUTE, "light/", environment, route.clone());
        request_generator(ADDRESS_ROUTE_WITHOUT_SLASH, "light", environment, route);
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
    fn create_get_request_with_inputs() {
        request_with_inputs(
            Route::get("/route").description("A GET route."),
            RestKind::Get,
            &Hazards::new(),
        );
    }

    #[test]
    fn create_post_request_with_inputs() {
        let route = Route::post("/route").description("A POST route.");
        request_with_inputs(route, RestKind::Post, &Hazards::new());
    }

    #[test]
    fn create_put_request_with_inputs() {
        let route = Route::put("/route").description("A PUT route.");
        request_with_inputs(route, RestKind::Put, &Hazards::new());
    }

    #[test]
    fn create_delete_request_with_inputs() {
        let route = Route::delete("/route").description("A DELETE route.");
        request_with_inputs(route, RestKind::Delete, &Hazards::new());
    }

    #[test]
    fn create_get_request_with_hazards_and_inputs() {
        let hazards = Hazards::new()
            .insert(Hazard::FireHazard)
            .insert(Hazard::AirPoisoning);

        request_with_inputs(
            Route::get("/route")
                .description("A GET route.")
                .with_hazards(hazards.clone()),
            RestKind::Get,
            &hazards,
        );
    }
}
