use std::collections::{HashMap, HashSet};
use std::net::IpAddr;

use serde::Serialize;

use ascot::device::{DeviceEnvironment, DeviceKind};
use ascot::route::RouteConfigs;

use crate::request::{Request, RequestInfo, create_requests};

pub(crate) fn build_device_address(scheme: &str, address: &IpAddr, port: u16) -> String {
    format!("{scheme}://{address}:{port}")
}

/// Device network information.
///
/// All data needed to contact a device in a network.
#[derive(Debug, PartialEq, Clone, Serialize)]
pub struct NetworkInformation {
    /// Device complete name.
    pub name: String,
    /// Device addresses.
    pub addresses: HashSet<IpAddr>,
    /// Device port.
    pub port: u16,
    /// Device properties.
    pub properties: HashMap<String, String>,
    /// Device last reachable address.
    pub last_reachable_address: String,
}

impl NetworkInformation {
    /// Creates a [`NetworkInformation`].
    #[must_use]
    pub const fn new(
        name: String,
        addresses: HashSet<IpAddr>,
        port: u16,
        properties: HashMap<String, String>,
        last_reachable_address: String,
    ) -> Self {
        Self {
            name,
            addresses,
            port,
            properties,
            last_reachable_address,
        }
    }
}

/// Device description.
///
/// All properties which describe a device.
#[derive(Debug, PartialEq, Clone, Serialize)]
pub struct Description {
    /// Device kind.
    pub kind: DeviceKind,
    /// Device environment.
    pub environment: DeviceEnvironment,
    /// Device main route.
    pub main_route: String,
}

impl Description {
    /// Creates a [`Description`].
    #[must_use]
    pub const fn new(kind: DeviceKind, environment: DeviceEnvironment, main_route: String) -> Self {
        Self {
            kind,
            environment,
            main_route,
        }
    }
}

/// A compliant device.
#[derive(Debug, PartialEq, Serialize)]
pub struct Device {
    // Information needed to contact a device in a network.
    network_info: NetworkInformation,
    // All data needed to describe a device.
    description: Description,
    // All device requests.
    requests: HashMap<String, Request>,
}

impl Device {
    /// Creates a [`Device`] from [`NetworkInformation`], [`Description`],
    /// and [`RouteConfigs`] data.
    ///
    /// This method might be useful when a device might be created from data
    /// contained in a database.
    #[must_use]
    pub fn new(
        network_info: NetworkInformation,
        description: Description,
        route_configs: RouteConfigs,
    ) -> Self {
        let requests = create_requests(
            route_configs,
            &network_info.last_reachable_address,
            &description.main_route,
            description.environment,
        );

        // TODO: Check if the last reachable address works or it is better to
        // build a new one. Return a Result here, because we have to evaluate
        // data validity.

        Self {
            network_info,
            description,
            requests,
        }
    }

    /// Returns an immutable reference to [`NetworkInformation`].
    #[must_use]
    pub const fn network_info(&self) -> &NetworkInformation {
        &self.network_info
    }

    /// Returns an immutable reference to [`Description`].
    #[must_use]
    pub const fn description(&self) -> &Description {
        &self.description
    }

    /// Returns requests information as a vector of [`RequestInfo`].
    #[must_use]
    #[inline]
    pub fn requests_info(&self) -> Vec<RequestInfo> {
        self.requests
            .iter()
            .map(|(route, sender)| RequestInfo::new(route, sender))
            .collect()
    }

    /// Returns the number of available requests for a device.
    #[must_use]
    #[inline]
    pub fn requests_count(&self) -> usize {
        self.requests.len()
    }

    /// Returns the [`Request`] associated with the given route.
    ///
    /// If [`None`], the given route **does not** exist.
    #[must_use]
    #[inline]
    pub fn request(&self, route: &str) -> Option<&Request> {
        self.requests.get(route)
    }

    pub(crate) const fn init(
        network_info: NetworkInformation,
        description: Description,
        requests: HashMap<String, Request>,
    ) -> Self {
        Self {
            network_info,
            description,
            requests,
        }
    }
}

/// A collection of [`Device`]s.
#[derive(Debug, PartialEq, Serialize)]
pub struct Devices(Vec<Device>);

impl Default for Devices {
    fn default() -> Self {
        Self::new()
    }
}

impl IntoIterator for Devices {
    type Item = Device;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl<'a> IntoIterator for &'a Devices {
    type Item = &'a Device;
    type IntoIter = std::slice::Iter<'a, Device>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl Devices {
    /// Creates a [`Device`]s collection.
    #[must_use]
    pub const fn new() -> Self {
        Self(Vec::new())
    }

    /// Creates [`Devices`] from a vector of [`Device`]s.
    #[must_use]
    pub const fn from_devices(devices: Vec<Device>) -> Self {
        Self(devices)
    }

    /// Adds a [`Device`].
    #[inline]
    pub fn add(&mut self, device: Device) {
        self.0.push(device);
    }

    /// Checks whether the collection is empty.
    #[must_use]
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    /// Returns the number of [`Device`] contained in a collection.
    #[must_use]
    #[inline]
    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// Gets a [`Device`] reference identified by the given index.
    #[must_use]
    #[inline]
    pub fn get(&self, index: usize) -> Option<&Device> {
        self.0.get(index)
    }

    /// Returns an iterator over [`Device`]s.
    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, Device> {
        self.0.iter()
    }
}

#[cfg(test)]
pub(crate) mod tests {
    use std::collections::{HashMap, HashSet};

    use ascot::device::{DeviceEnvironment, DeviceKind};
    use ascot::hazards::{Hazard, Hazards};
    use ascot::parameters::Parameters;
    use ascot::route::{Route, RouteConfigs};

    use super::{Description, Device, Devices, NetworkInformation, build_device_address};

    fn create_network_info(address: &str, port: u16) -> NetworkInformation {
        let ip_address = address.parse().unwrap();

        let complete_address = build_device_address("http", &ip_address, port);

        let mut addresses = HashSet::new();
        addresses.insert(ip_address);
        addresses.insert("172.0.0.1".parse().unwrap());

        let mut properties = HashMap::new();
        properties.insert("scheme".into(), "http".into());

        NetworkInformation::new(
            "device-name1._ascot._tcp.local.".into(),
            addresses,
            port,
            properties,
            complete_address,
        )
    }

    fn create_description(device_kind: DeviceKind, main_route: &str) -> Description {
        Description::new(device_kind, DeviceEnvironment::Os, main_route.into())
    }

    pub(crate) fn create_light() -> Device {
        let network_info = create_network_info("192.168.1.174", 5000);
        let description = create_description(DeviceKind::Light, "light/");

        let light_on_route = Route::put("/on")
            .description("Turn light on.")
            .with_hazard(Hazard::ElectricEnergyConsumption);

        let light_off_route = Route::put("/off")
            .description("Turn light off.")
            .with_hazard(Hazard::LogEnergyConsumption);

        let toggle_route = Route::get("/toggle")
            .description("Toggle a light.")
            .with_hazards(
                Hazards::new()
                    .insert(Hazard::FireHazard)
                    .insert(Hazard::ElectricEnergyConsumption),
            )
            .with_parameters(Parameters::new().rangeu64("brightness", (0, 20, 1)));

        let route_configs = RouteConfigs::new()
            .insert(light_on_route.serialize_data())
            .insert(light_off_route.serialize_data())
            .insert(toggle_route.serialize_data());

        Device::new(network_info, description, route_configs)
    }

    pub(crate) fn create_fridge() -> Device {
        let network_info = create_network_info("192.168.1.175", 6000);
        let description = create_description(DeviceKind::Fridge, "fridge/");

        let increase_temperature_route = Route::put("/increase-temperature")
            .description("Increase temperature.")
            .with_hazards(
                Hazards::new()
                    .insert(Hazard::ElectricEnergyConsumption)
                    .insert(Hazard::SpoiledFood),
            )
            .with_parameters(Parameters::new().rangef64_with_default(
                "increment",
                (1., 4., 0.1),
                2.,
            ));

        let decrease_temperature_route = Route::put("/decrease-temperature")
            .description("Decrease temperature.")
            .with_hazards(
                Hazards::new()
                    .insert(Hazard::ElectricEnergyConsumption)
                    .insert(Hazard::SpoiledFood),
            )
            .with_parameters(Parameters::new().rangef64_with_default(
                "decrement",
                (1., 4., 0.1),
                2.,
            ));

        let route_configs = RouteConfigs::new()
            .insert(increase_temperature_route.serialize_data())
            .insert(decrease_temperature_route.serialize_data());

        Device::new(network_info, description, route_configs)
    }

    pub(crate) fn create_unknown() -> Device {
        let network_info = create_network_info("192.168.1.176", 5500);
        let description = create_description(DeviceKind::Unknown, "ip-camera/");

        let camera_stream_route = Route::get("/stream")
            .description("View camera stream.")
            .with_hazards(
                Hazards::new()
                    .insert(Hazard::ElectricEnergyConsumption)
                    .insert(Hazard::VideoDisplay)
                    .insert(Hazard::VideoRecordAndStore),
            );

        let screenshot_route = Route::get("/take-screenshot")
            .description("Take a screenshot.")
            .with_hazards(
                Hazards::new()
                    .insert(Hazard::ElectricEnergyConsumption)
                    .insert(Hazard::TakeDeviceScreenshots)
                    .insert(Hazard::TakePictures),
            );

        let route_configs = RouteConfigs::new()
            .insert(camera_stream_route.serialize_data())
            .insert(screenshot_route.serialize_data());

        Device::new(network_info, description, route_configs)
    }

    #[test]
    fn check_devices() {
        let devices_vector = vec![create_light(), create_fridge(), create_unknown()];

        let devices_from_vector = Devices::from_devices(devices_vector);

        let mut devices = Devices::new();

        // A device is empty when being created.
        assert!(devices.is_empty());

        devices.add(create_light());
        devices.add(create_fridge());
        devices.add(create_unknown());

        // Compare devices created with two different methods.
        assert_eq!(devices_from_vector, devices);

        // A device must not be empty.
        assert!(!devices.is_empty());

        // Check number of elements in devices.
        assert_eq!(devices.len(), 3);

        // Get a non-existent device.
        assert_eq!(devices.get(1000), None);

        // Get a reference to a device. The order is important.
        assert_eq!(devices.get(2), Some(&create_unknown()));
    }
}
