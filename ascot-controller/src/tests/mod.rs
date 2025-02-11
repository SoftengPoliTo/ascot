use std::net::Ipv4Addr;
use std::time::Duration;

use ascot_library::device::{DeviceEnvironment, DeviceKind};
use ascot_library::hazards::{Hazard, Hazards};
use ascot_library::parameters::{ParameterKind, Parameters, ParametersData};
use ascot_library::response::ResponseKind;
use ascot_library::route::{RestKind, Route};

use ascot_axum::actions::error::ErrorResponse;
use ascot_axum::actions::ok::{mandatory_ok_stateless, OkResponse};
use ascot_axum::actions::serial::{serial_stateless, SerialResponse};
use ascot_axum::devices::light::Light;
use ascot_axum::extract::Path;
use ascot_axum::server::Server;
use ascot_axum::service::ServiceConfig;

use serde::{Deserialize, Serialize};

use tracing::info;

use crate::device::Device;
use crate::discovery::Discovery;
use crate::request::Request;

const PORT_ONE: u16 = 3000;
const PORT_TWO: u16 = 4000;

const FIRST_DEVICE_ROUTE: &str = "/with-toggle";
const SECOND_DEVICE_ROUTE: &str = "/without-toggle";

pub(crate) const DOMAIN: &str = "ascot";

async fn turn_light_on() -> Result<OkResponse, ErrorResponse> {
    println!("Light on");
    Ok(OkResponse::ok())
}

async fn turn_light_off() -> Result<OkResponse, ErrorResponse> {
    println!("Light off");
    Ok(OkResponse::ok())
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub(crate) struct Brightness {
    pub(crate) brightness: u64,
}

async fn toggle(Path(brightness): Path<u64>) -> Result<SerialResponse<Brightness>, ErrorResponse> {
    println!("Brightness: {brightness}");
    Ok(SerialResponse::new(Brightness { brightness }))
}

async fn light(
    port: u16,
    id: &str,
    with_toggle: bool,
    close_rx: tokio::sync::oneshot::Receiver<()>,
) {
    // Turn light on `PUT` route.
    let light_on_route = Route::put("/on")
        .description("Turn light on.")
        .with_hazard(Hazard::ElectricEnergyConsumption);

    // Turn light off `PUT` route.
    let light_off_route = Route::put("/off").description("Turn light off.");

    // A light device which is going to be run on the server.
    let device = Light::new()
        // This method is mandatory, if not called, a compiler error is raised.
        .turn_light_on(mandatory_ok_stateless(light_on_route, turn_light_on))
        // This method is mandatory, if not called, a compiler error is raised.
        .turn_light_off(mandatory_ok_stateless(light_off_route, turn_light_off));

    let device = if with_toggle {
        // Toggle `PUT` route.
        let toggle_route = Route::get("/toggle")
            .description("Toggle a light.")
            .with_hazard(Hazard::ElectricEnergyConsumption)
            .with_parameters(Parameters::new().rangeu64("brightness", (0, 20, 1)));

        device
            .main_route(FIRST_DEVICE_ROUTE)
            .add_action(serial_stateless(toggle_route, toggle))
            .unwrap()
    } else {
        device.main_route(SECOND_DEVICE_ROUTE)
    };

    info!(
        "Inside the light device {} `toggle` action and port {port}",
        if with_toggle { "with" } else { "without" }
    );

    // Run a discovery service and the device on the server.
    Server::new(device.into_device())
        .address(Ipv4Addr::UNSPECIFIED)
        .port(port)
        .well_known_service(id)
        .discovery_service(ServiceConfig::mdns_sd(id).hostname("ascot").domain(DOMAIN))
        .with_graceful_shutdown(async move {
            _ = close_rx.await;
        })
        .run()
        .await
        .expect("Error in running a device server.");
}

pub(crate) async fn light_with_toggle(close_rx: tokio::sync::oneshot::Receiver<()>) {
    light(PORT_ONE, "light-with-toggle", true, close_rx).await;
}

pub(crate) async fn light_without_toggle(close_rx: tokio::sync::oneshot::Receiver<()>) {
    light(PORT_TWO, "light-without-toggle", false, close_rx).await;
}

fn build_route(device: &Device, route: &str) -> String {
    format!(
        "{}{}{}",
        device.network_info().last_reachable_address,
        device.description().main_route,
        route
    )
}

fn check_request(
    device: &Device,
    route: &str,
    kind: RestKind,
    hazards: Hazards,
    parameters_data: ParametersData,
    response_kind: ResponseKind,
) {
    let request_sender = device.request(route);

    assert_eq!(
        request_sender,
        Some(&Request {
            kind,
            hazards,
            route: build_route(device, route),
            parameters_data,
            response_kind,
            device_environment: DeviceEnvironment::Os,
        })
    );
}

// Device addresses are not considered in the comparisons, because they
// depend on the machine this test is being run on.
pub(crate) fn compare_device_data(device: &Device) {
    // Check port.
    assert!(device.network_info().port == PORT_ONE || device.network_info().port == PORT_TWO);

    // Check scheme.
    let scheme = device.network_info().properties.get("scheme");
    assert!(scheme.is_some_and(|scheme| scheme == "http"));

    // Check path.
    let path = device.network_info().properties.get("path");
    assert!(
        path.is_some_and(|path| path == "/.well-known/light-with-toggle"
            || path == "/.well-known/light-without-toggle")
    );

    // Check device main route.
    assert!(
        device.description().main_route == FIRST_DEVICE_ROUTE
            || device.description().main_route == SECOND_DEVICE_ROUTE
    );

    // Check device information.
    assert_eq!(device.description().kind, DeviceKind::Light);
    assert_eq!(device.description().environment, DeviceEnvironment::Os);

    // Check requests number.
    assert!(
        device.description().main_route == FIRST_DEVICE_ROUTE && device.requests_count() == 3
            || device.description().main_route == SECOND_DEVICE_ROUTE
                && device.requests_count() == 2
    );

    let hazards = Hazards::init(Hazard::ElectricEnergyConsumption);

    if device.description().main_route == FIRST_DEVICE_ROUTE {
        let parameters_data = ParametersData::new().insert(
            "brightness".into(),
            ParameterKind::RangeU64 {
                min: 0,
                max: 20,
                step: 1,
                default: 0,
            },
        );
        // Check "/toggle" request
        check_request(
            device,
            "/toggle",
            RestKind::Get,
            hazards.clone(),
            parameters_data,
            ResponseKind::Serial,
        );
    }

    // Check "/on" request
    check_request(
        device,
        "/on",
        RestKind::Put,
        hazards,
        ParametersData::new(),
        ResponseKind::Ok,
    );

    // Check "/off" request
    check_request(
        device,
        "/off",
        RestKind::Put,
        Hazards::new(),
        ParametersData::new(),
        ResponseKind::Ok,
    );
}

pub(crate) fn configure_discovery() -> Discovery {
    Discovery::new(DOMAIN)
        .timeout(Duration::from_secs(1))
        .disable_ipv6()
        .disable_ip(Ipv4Addr::new(172, 17, 0, 1))
}
