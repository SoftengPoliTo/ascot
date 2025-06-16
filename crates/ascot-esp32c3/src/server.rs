use esp_idf_svc::http::Method;
use esp_idf_svc::http::server::{Configuration, EspHttpServer};
use esp_idf_svc::io::Write;

use ascot::route::RestKind;

use crate::device::Device;
use crate::error::Result;
use crate::service::{InternalService, ServiceConfig};

// Default port.
const DEFAULT_SERVER_PORT: u16 = 3000;

// Server stack size.
const DEFAULT_STACK_SIZE: usize = 10240;

/// A [`Device`] server.
pub struct Server {
    // Server port.
    port: u16,
    // Stack size
    stack_size: usize,
    // Device.
    device: Device,
    // Service configuration.
    service_config: Option<ServiceConfig>,
}

impl Server {
    /// Creates a [`Server`].
    #[must_use]
    pub const fn new(device: Device) -> Self {
        Self {
            port: DEFAULT_SERVER_PORT,
            stack_size: DEFAULT_STACK_SIZE,
            device,
            service_config: None,
        }
    }

    /// Sets server port.
    #[must_use]
    pub const fn port(mut self, port: u16) -> Self {
        self.port = port;
        self
    }

    /// Sets server stack size.
    #[must_use]
    pub const fn stack_size(mut self, stack_size: usize) -> Self {
        self.stack_size = stack_size;
        self
    }

    /// Sets a server discovery service configuration.
    #[must_use]
    pub const fn service(mut self, service_config: ServiceConfig) -> Self {
        self.service_config = Some(service_config);
        self
    }

    /// Runs the server.
    ///
    ///  # Errors
    ///
    /// It returns an error whether a server fails to start.
    pub fn run(self) -> Result<()> {
        let mut server = EspHttpServer::new(&Configuration {
            stack_size: self.stack_size,
            http_port: self.port,
            ..Default::default()
        })?;

        let (device_route, device_data, routes_data) = self.device.finalize();

        // Format the device description as a pretty string.
        let device_description = serde_json::to_string_pretty(&device_data)?;

        for route in routes_data {
            let method = match route.route_config.rest_kind {
                RestKind::Get => Method::Get,
                RestKind::Post => Method::Post,
                RestKind::Put => Method::Put,
                RestKind::Delete => Method::Delete,
            };
            if let Some(body) = route.body {
                server.fn_handler(
                    &format!("{}{}", device_route, route.route_config.data.path),
                    method,
                    move |req| {
                        // Run body.
                        body()?;

                        // Write response.
                        (route.response)(req)?.write_all(route.content.as_bytes())
                    },
                )?;
            } else {
                server.fn_handler(&route.route_config.data.path, method, move |req| {
                    // Write only response.
                    (route.response)(req)?.write_all(route.content.as_bytes())
                })?;
            }
        }

        // Add main route
        server.fn_handler(device_route, Method::Get, move |req| {
            req.into_response(200, Some("OK"), &[("Content-Type", "application/json")])?
                .write_all(device_description.as_bytes())
        })?;

        if let Some(service_config) = self.service_config {
            // Run service
            InternalService::run(service_config)
        } else {
            loop {
                std::thread::sleep(std::time::Duration::from_secs(1));
            }
        }
    }
}
