use std::borrow::Cow;

use tracing::warn;

use crate::device::{Device, Devices};
use crate::discovery::Discovery;
use crate::error::{Error, ErrorKind};
use crate::parameters::Parameters;
use crate::policy::Policy;
use crate::request::Request;
use crate::response::Response;

// TODO: As of now, we are using an id which is the position in the devices
// list. We are going to replace that with a natural language String instead.

fn sender_error(error: impl Into<Cow<'static, str>>) -> Error {
    Error::new(ErrorKind::Sender, error)
}

/// A request sender.
#[derive(Debug, PartialEq)]
pub struct RequestSender<'controller> {
    controller: &'controller Controller,
    request: &'controller Request,
    skip: bool,
}

impl RequestSender<'_> {
    /// Sends a request to a device, getting in return a [`Response`].
    ///
    /// # Errors
    ///
    /// While sending a request to a device, some network failures or timeouts
    /// can prevent the effective sending. Moreover, the same issues can also
    /// affect the returned response.
    pub async fn send(&self) -> Result<Response, Error> {
        self.request
            .retrieve_response(self.skip, || async { self.request.plain_send().await })
            .await
    }

    /// Sends a request to a device with the given [`Parameters`], getting in
    /// return a [`Response`].
    ///
    /// # Errors
    ///
    /// While sending a request to a device, some network failures or timeouts
    /// can prevent the effective sending. Moreover, the same issues can also
    /// affect the returned response.
    pub async fn send_with_parameters(
        &self,
        parameters: &Parameters<'_>,
    ) -> Result<Response, Error> {
        if self.request.parameters_data.is_empty() {
            warn!("The request does not have input parameters.");
            return self.send().await;
        }

        self.request
            .retrieve_response(self.skip, || async {
                self.request.create_response(parameters).await
            })
            .await
    }
}

/// A sender for the requests of a determined device.
#[derive(Debug, PartialEq)]
pub struct DeviceSender<'controller> {
    controller: &'controller Controller,
    device: &'controller Device,
    id: usize,
}

impl DeviceSender<'_> {
    /// Builds the [`RequestSender`] for the given request, identified by its
    /// route, associated with this [`DeviceSender`] instance.
    ///
    ///
    /// # Errors
    ///
    /// An error is returned when the given route **does** not exist.
    pub fn request(&self, route: &str) -> Result<RequestSender, Error> {
        let request = self.device.request(route).ok_or(sender_error(format!(
            "Error in retrieving the request with route `{route}`."
        )))?;

        let skip = if request.hazards.is_empty() {
            false
        } else {
            self.evaluate_privacy_policy(request, route)
        };

        Ok(RequestSender {
            controller: self.controller,
            request,
            skip,
        })
    }

    fn evaluate_privacy_policy(&self, request: &Request, route: &str) -> bool {
        let mut skip = false;

        let global_blocked_hazards = self
            .controller
            .privacy_policy
            .global_blocked_hazards(&request.hazards);

        let local_blocked_hazards = self
            .controller
            .privacy_policy
            .local_blocked_hazards(self.id, &request.hazards);

        if !global_blocked_hazards.is_empty() {
            warn!(
                "The {route} is skipped because it contains the global blocked hazards: {:?}",
                global_blocked_hazards
            );
            skip = true;
        }

        if !local_blocked_hazards.is_empty() {
            warn!(
                "The {route} is skipped because the device contains the local blocked hazards: {:?}",
                local_blocked_hazards
            );
            skip = true;
        }

        skip
    }
}

/// A controller for sending requests.
///
/// It sends or does not send requests to devices according to:
///
/// - A privacy policy
///
/// When the controller receives a response from a device, it forwards it
/// directly to the caller.
#[derive(Debug, PartialEq)]
pub struct Controller {
    discovery: Discovery,
    devices: Devices,
    privacy_policy: Policy,
}

impl Controller {
    /// Creates a [`Controller`] given a [`Discovery`] configuration.
    #[must_use]
    #[inline]
    pub fn new(discovery: Discovery) -> Self {
        Self {
            discovery,
            devices: Devices::new(),
            privacy_policy: Policy::init(),
        }
    }

    /// Creates a [`Controller`] from a [`Discovery`] configuration and
    /// a set of initial [`Devices`].
    ///
    /// This method might be useful when [`Devices`] are retrieved from
    /// a database.
    #[must_use]
    #[inline]
    pub fn from_devices(discovery: Discovery, devices: Devices) -> Self {
        Self {
            discovery,
            devices,
            privacy_policy: Policy::init(),
        }
    }

    /// Sets a [`Policy`].
    #[must_use]
    #[inline]
    pub fn policy(mut self, privacy_policy: Policy) -> Self {
        self.privacy_policy = privacy_policy;
        self
    }

    /// Change preset [`Policy`].
    #[inline]
    pub fn change_policy(&mut self, privacy_policy: Policy) {
        self.privacy_policy = privacy_policy;
    }

    /// Discovers all available [`Devices`] in a network.
    ///
    /// # Errors
    ///
    /// ## Discovery Errors
    ///
    /// During a discovery process some of the most common errors are the
    /// impossibility to connect to a network, disable a particular interface,
    /// or close the discovery process itself.
    ///
    /// ## Sending Requests Errors
    ///
    /// While sending a request to a device to obtain the description of its
    /// structure and all of its routes, some network failures or
    /// timeouts can prevent the effective sending.
    /// Moreover, the same issues can also affect the return response.
    #[inline]
    pub async fn discover(&mut self) -> Result<(), Error> {
        self.devices = self.discovery.discover().await?;
        Ok(())
    }

    /// Returns controller [`Devices`].
    #[must_use]
    pub const fn devices(&self) -> &Devices {
        &self.devices
    }

    /// Builds a [`DeviceSender`] for the [`Device`] with the given identifier.
    ///
    /// # Errors
    ///
    /// An error is returned when there are no devices or the given index
    /// **does** not exist.
    pub fn device(&self, id: usize) -> Result<DeviceSender, Error> {
        if self.devices.is_empty() {
            return Err(sender_error("No devices found."));
        }

        let device = self.devices.get(id).ok_or(sender_error(format!(
            "Error in retrieving the device with identifier {id}."
        )))?;
        Ok(DeviceSender {
            controller: self,
            device,
            id,
        })
    }
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use ascot::hazards::{Hazard, Hazards};
    use ascot::response::{OkResponse, SerialResponse};

    use serde::{Serialize, de::DeserializeOwned};
    use serde_json::json;

    use serial_test::serial;

    use crate::device::Devices;
    use crate::error::Error;
    use crate::parameters::Parameters;
    use crate::policy::Policy;
    use crate::response::Response;

    use crate::device::tests::{create_fridge, create_light, create_unknown};
    use crate::discovery::tests::configure_discovery;
    use crate::tests::{Brightness, check_function_with_device};

    use super::{Controller, DeviceSender, RequestSender, sender_error};

    #[test]
    fn empty_controller() {
        let controller = Controller::new(configure_discovery());

        assert_eq!(
            controller,
            Controller {
                discovery: configure_discovery(),
                devices: Devices::new(),
                privacy_policy: Policy::init(),
            }
        );

        // No devices.
        assert_eq!(controller.device(0), Err(sender_error("No devices found.")));
    }

    #[test]
    fn controller_from_devices() {
        let devices =
            Devices::from_devices(vec![create_light(), create_fridge(), create_unknown()]);

        let controller = Controller::from_devices(configure_discovery(), devices);

        assert_eq!(
            controller,
            Controller {
                discovery: configure_discovery(),
                devices: Devices::from_devices(vec![
                    create_light(),
                    create_fridge(),
                    create_unknown()
                ]),
                privacy_policy: Policy::init(),
            }
        );
    }

    async fn check_ok_response_plain(device_sender: &DeviceSender<'_>, route: &str) {
        check_ok_response(device_sender, route, async move |request_sender| {
            request_sender.send().await
        })
        .await;
    }

    async fn check_ok_response_with_parameters(
        device_sender: &DeviceSender<'_>,
        route: &str,
        parameters: &Parameters<'_>,
    ) {
        check_ok_response(device_sender, route, async move |request_sender| {
            request_sender.send_with_parameters(parameters).await
        })
        .await;
    }

    async fn check_ok_response<'controller, 'a, F>(
        device_sender: &'a DeviceSender<'controller>,
        route: &'a str,
        get_response: F,
    ) where
        F: AsyncFnOnce(RequestSender<'controller>) -> Result<Response, Error>,
        'a: 'controller,
    {
        let request = device_sender.request(route).unwrap();

        let response = get_response(request).await.unwrap();
        if let Response::OkBody(response) = response {
            let ok_response = response.parse_body().await.unwrap();
            assert_eq!(ok_response, OkResponse::ok());
        } else {
            assert!(
                matches!(response, Response::Skipped),
                "Should be a blocked global `LogEnergyConsumption` for `/off` request"
            );
        }
    }

    async fn check_serial_response_plain<T: Serialize + DeserializeOwned + Debug + PartialEq>(
        device_sender: &DeviceSender<'_>,
        route: &str,
        value: T,
    ) {
        check_serial_response(
            device_sender,
            route,
            async move |request_sender| request_sender.send().await,
            value,
        )
        .await;
    }

    async fn check_serial_response_with_parameters<
        T: Serialize + DeserializeOwned + Debug + PartialEq,
    >(
        device_sender: &DeviceSender<'_>,
        route: &str,
        parameters: &Parameters<'_>,
        value: T,
    ) {
        check_serial_response(
            device_sender,
            route,
            async move |request| request.send_with_parameters(parameters).await,
            value,
        )
        .await;
    }

    async fn check_serial_response<'controller, 'a, F, T>(
        device: &'a DeviceSender<'controller>,
        route: &'a str,
        get_response: F,
        value: T,
    ) where
        F: AsyncFnOnce(RequestSender<'controller>) -> Result<Response, Error>,
        T: Serialize + DeserializeOwned + Debug + PartialEq,
        'a: 'controller,
    {
        let request = device.request(route).unwrap();

        let response = get_response(request).await.unwrap();
        if let Response::SerialBody(response) = response {
            let serial_response = response.parse_body::<T>().await.unwrap();
            assert_eq!(serial_response, SerialResponse::new(value));
        } else {
            assert!(
                matches!(response, Response::Skipped),
                "Should be a blocked local `FireHazard` for `/toggle` request"
            );
        }
    }

    async fn controller_checks(controller: Controller) {
        // Wrong device id.
        assert_eq!(
            controller.device(1),
            Err(sender_error(
                "Error in retrieving the device with identifier 1."
            ))
        );

        // Get device.
        let device_sender = controller.device(0).unwrap();

        // Wrong request.
        assert_eq!(
            device_sender.request("/wrong"),
            Err(sender_error(
                "Error in retrieving the request with route `/wrong`."
            ))
        );

        // Run "/on" request and get "Ok" response.
        check_ok_response_plain(&device_sender, "/on").await;

        // Run "/off" request and get "Ok" response.
        check_ok_response_plain(&device_sender, "/off").await;

        // Run "/toggle" request and get "Ok" response.
        check_serial_response_plain(
            &device_sender,
            "/toggle",
            json!({
                "brightness": 0,
            }),
        )
        .await;

        // With parameters
        let mut parameters = Parameters::new();
        parameters.u64("brightness", 5);

        // Run "/on" request and get an "Ok" response with parameters.
        check_ok_response_with_parameters(&device_sender, "/on", &parameters).await;

        // Run "/off" request and get an "Ok" response with parameters.
        check_ok_response_with_parameters(&device_sender, "/off", &parameters).await;

        // Run "/toggle" request and get an "Ok" response with parameters.
        check_serial_response_with_parameters(
            &device_sender,
            "/toggle",
            &parameters,
            Brightness { brightness: 5 },
        )
        .await;
    }

    #[inline]
    async fn controller_without_policy() {
        // Create a controller.
        let mut controller = Controller::new(configure_discovery());

        // Run discovery process.
        controller.discover().await.unwrap();

        // Run controller checks.
        controller_checks(controller).await;
    }

    #[inline]
    async fn controller_with_policy() {
        // Global blocked hazards.
        let global_hazards = Hazards::new().insert(Hazard::LogEnergyConsumption);

        // Local blocked hazards for a specific device.
        let local_hazards = Hazards::new().insert(Hazard::FireHazard);

        // Create both a global policy and a local one.
        let policy = Policy::new(global_hazards).block_device_on_hazards(0, local_hazards);

        // Create a controller.
        let mut controller = Controller::new(configure_discovery()).policy(policy);

        // Run discovery process.
        controller.discover().await.unwrap();

        // Run controller checks.
        controller_checks(controller).await;
    }

    #[tokio::test(flavor = "multi_thread", worker_threads = 2)]
    #[serial]
    async fn test_without_policy_controller() {
        check_function_with_device(async move || controller_without_policy().await).await;
    }

    #[tokio::test(flavor = "multi_thread", worker_threads = 2)]
    #[serial]
    async fn test_with_policy_controller() {
        check_function_with_device(async move || controller_with_policy().await).await;
    }
}
