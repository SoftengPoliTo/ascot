use std::net::IpAddr;
use std::time::Duration;

use ascot_library::device::DeviceData;

use mdns_sd::{IfKind, ServiceDaemon, ServiceEvent, ServiceInfo};

use tracing::{info, warn};

use crate::device::{build_device_address, Description, Device, Devices, NetworkInformation};
use crate::error::Error;
use crate::request::create_requests;

// Service top-level domain.
//
// It defines the default top-level domain for a service.
const TOP_LEVEL_DOMAIN: &str = "local";

/// Service transport protocol.
#[derive(Debug, PartialEq)]
pub enum TransportProtocol {
    /// TCP service.
    TCP,
    /// UDP service.
    UDP,
}

impl std::fmt::Display for TransportProtocol {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.name().fmt(f)
    }
}

impl TransportProtocol {
    /// Returns the [`TransportProtocol`] name.
    #[must_use]
    pub const fn name(&self) -> &'static str {
        match self {
            Self::TCP => "tcp",
            Self::UDP => "udp",
        }
    }
}

/// Devices discovery.
///
/// It detects all `ascot`-compliant [`Device`]s in a network.
#[derive(Debug, PartialEq)]
pub struct Discovery {
    domain: &'static str,
    transport_protocol: TransportProtocol,
    top_level_domain: &'static str,
    timeout: Duration,
    disable_ipv6: bool,
    disable_ip: Option<IpAddr>,
    disable_network_interface: Option<&'static str>,
}

impl Discovery {
    /// Creates a [`Discovery`].
    #[must_use]
    pub const fn new(domain: &'static str) -> Self {
        Self {
            domain,
            transport_protocol: TransportProtocol::TCP,
            top_level_domain: TOP_LEVEL_DOMAIN,
            timeout: Duration::from_secs(2), // Default timeout of 2s.
            disable_ipv6: false,
            disable_ip: None,
            disable_network_interface: None,
        }
    }

    /// Sets a different timeout.
    #[must_use]
    pub const fn timeout(mut self, timeout: Duration) -> Self {
        self.timeout = timeout;
        self
    }

    /// Sets the service transport protocol.
    #[must_use]
    pub const fn transport_protocol(mut self, transport_protocol: TransportProtocol) -> Self {
        self.transport_protocol = transport_protocol;
        self
    }

    /// Changes service domain.
    #[must_use]
    pub const fn domain(mut self, domain: &'static str) -> Self {
        self.domain = domain;
        self
    }

    /// Sets the service top-level domain.
    #[must_use]
    pub const fn top_level_domain(mut self, top_level_domain: &'static str) -> Self {
        self.top_level_domain = top_level_domain;
        self
    }

    /// Do not discover devices with `IPv6` interfaces.
    #[must_use]
    pub const fn disable_ipv6(mut self) -> Self {
        self.disable_ipv6 = true;
        self
    }

    /// Disables the given IP address.
    #[must_use]
    #[inline]
    pub fn disable_ip(mut self, ip: impl Into<IpAddr>) -> Self {
        self.disable_ip = Some(ip.into());
        self
    }

    /// Disables the given network interface.
    #[must_use]
    pub const fn disable_network_interface(mut self, network_interface: &'static str) -> Self {
        self.disable_network_interface = Some(network_interface);
        self
    }

    pub(crate) async fn discover(&self) -> Result<Devices, Error> {
        // Discover devices.
        let discovery_info = self.discover_devices()?;

        Self::obtain_devices_data(discovery_info).await
    }

    fn discover_devices(&self) -> Result<Vec<ServiceInfo>, Error> {
        // Create a mdns daemon
        let mdns = ServiceDaemon::new()?;

        // Disable IPv6 interface.
        if self.disable_ipv6 {
            mdns.disable_interface(IfKind::IPv6)?;
        }

        // Disable IP.
        if let Some(ip) = self.disable_ip {
            mdns.disable_interface(ip)?;
        }

        // Disable network interface.
        if let Some(network_interface) = self.disable_network_interface {
            mdns.disable_interface(network_interface)?;
        }

        // Service type.
        let service_type = format!(
            "_{}._{}.{}.",
            self.domain,
            self.transport_protocol.name(),
            self.top_level_domain
        );

        // Detects devices.
        let receiver = mdns.browse(&service_type)?;

        // Discovery information.
        let mut discovery_info: Vec<ServiceInfo> = Vec::new();

        // Run for n-seconds in search of devices and saves their information
        // in memory.
        while let Ok(event) = receiver.recv_timeout(self.timeout) {
            if let ServiceEvent::ServiceResolved(info) = event {
                // Check whether there are device addresses.
                //
                // If no address has been found, prints a warning and
                // continue the loop.
                if info.get_addresses().is_empty() {
                    warn!("No device address available for {:?}", info);
                    continue;
                }

                // A scheme is necessary to get in touch with a device,
                // so if it is not present, skip that device.
                if info.get_property("scheme").is_none() {
                    warn!("No `scheme` property found.");
                    continue;
                }

                // If two devices are equal, skip to the next one.
                if Self::check_device_duplicates(&discovery_info, &info) {
                    continue;
                }

                discovery_info.push(info);
            }
        }

        // Stop detection.
        mdns.stop_browse(&service_type)?;

        Ok(discovery_info)
    }

    async fn obtain_devices_data(discovery_info: Vec<ServiceInfo>) -> Result<Devices, Error> {
        // Devices collection.
        let mut devices = Devices::new();

        // Iterate over discovered metadata
        for info in discovery_info {
            // Try to contact each available address for a device
            // to retrieve data.
            for address in info.get_addresses() {
                let complete_address = build_device_address(
                    info.get_property("scheme").map_or("http", |v| v.val_str()),
                    address,
                    info.get_port(),
                );
                info!("Complete address: {complete_address}");

                // Contact devices to retrieve their data
                match reqwest::get(&complete_address).await {
                    Ok(response) => {
                        let device_data: DeviceData = response.json().await?;

                        let requests = create_requests(
                            device_data.route_configs,
                            &complete_address,
                            &device_data.main_route,
                            device_data.environment,
                        );

                        let description = Description::new(
                            device_data.kind,
                            device_data.environment,
                            device_data.main_route.into_owned(),
                        );

                        let network_info = NetworkInformation::new(
                            info.get_fullname().to_string(),
                            info.get_addresses().iter().copied().collect(),
                            info.get_port(),
                            info.get_properties()
                                .iter()
                                .map(|v| (v.key().to_string(), v.val_str().to_string()))
                                .collect(),
                            complete_address,
                        );

                        devices.add(Device::init(network_info, description, requests));

                        // Only a single address is necessary.
                        break;
                    }
                    Err(e) => {
                        warn!("Impossible to contact address {complete_address}: {e}");
                        continue;
                    }
                }
            }
        }

        Ok(devices)
    }

    // A discovered device is equal to another device when:
    //
    // - It has an address with IP and port identical to the ones of
    //   another device address.
    //   Devices belonging to the same local network CANNOT HAVE any IP
    //   and port in common.
    //
    //   OR
    //
    // - It has the same full name of another device belonging to the same
    //   network. A full name, in this case, represents the device ID.
    //   Two devices belonging to the same network CANNOT HAVE the same ID.
    fn check_device_duplicates(discovery_info: &[ServiceInfo], info: &ServiceInfo) -> bool {
        for disco_info in discovery_info {
            // When the addresses have distinct ports, they are always
            // different, so they are not considered.
            if disco_info.get_port() != info.get_port() {
                continue;
            }

            for address in disco_info.get_addresses() {
                if info.get_addresses().contains(address) {
                    return true;
                }
            }

            if disco_info.get_fullname() == info.get_fullname() {
                return true;
            }
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use serial_test::serial;

    use crate::tests::{
        compare_device_data, configure_discovery, light_with_toggle, light_without_toggle,
    };

    async fn discovery_comparison(devices_len: usize) {
        let devices = configure_discovery().discover().await.unwrap();

        // Count devices.
        assert_eq!(devices.len(), devices_len);

        // Iterate over devices and compare data.
        for device in devices {
            compare_device_data(&device);
        }
    }

    #[tokio::test(flavor = "multi_thread", worker_threads = 2)]
    #[serial]
    async fn test_single_device_discovery() {
        let _ = tracing_subscriber::fmt().try_init();

        let (close_tx, close_rx) = tokio::sync::oneshot::channel();

        // Run a device task.
        let device_handle = tokio::spawn(async { light_with_toggle(close_rx).await });

        // Wait for device task to be configured.
        tokio::time::sleep(Duration::from_millis(100)).await;

        // Run discovery comparison.
        discovery_comparison(1).await;

        // Shutdown device server.
        _ = close_tx.send(());

        // Wait for device server to gracefully shutdown.
        _ = device_handle.await;
    }

    #[tokio::test(flavor = "multi_thread", worker_threads = 3)]
    #[serial]
    async fn test_more_devices_discovery() {
        let _ = tracing_subscriber::fmt().try_init();

        let (close_tx1, close_rx1) = tokio::sync::oneshot::channel();
        let (close_tx2, close_rx2) = tokio::sync::oneshot::channel();

        // Run first device task.
        let device1_handle = tokio::spawn(async { light_without_toggle(close_rx1).await });

        // Wait for first device task to be configured.
        tokio::time::sleep(Duration::from_millis(100)).await;

        // Run second device task.
        let device2_handle = tokio::spawn(async { light_with_toggle(close_rx2).await });

        // Wait for second device task to be configured.
        tokio::time::sleep(Duration::from_millis(100)).await;

        // Run discovery comparison.
        discovery_comparison(2).await;

        // Shutdown first device server.
        _ = close_tx1.send(());

        // Wait for first device server to gracefully shutdown.
        _ = device1_handle.await;

        // Shutdown second device server.
        _ = close_tx2.send(());

        // Wait for second device server to gracefully shutdown.
        _ = device2_handle.await;
    }
}
