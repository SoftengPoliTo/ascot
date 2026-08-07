use std::collections::HashMap;
use std::net::{IpAddr, Ipv4Addr};

use crate::error::{Error, ErrorKind, Result};

// Service domain.
//
// It defines the default domain for a service.
const DOMAIN: &str = "tosca";

// Service top-level domain.
//
// It defines the default top-level domain for a service.
const TOP_LEVEL_DOMAIN: &str = "local";

/// The discovery service transport protocol.
#[derive(Debug, Clone, Copy)]
pub enum TransportProtocol {
    /// TCP-based service.
    TCP,
    /// UDP-based service.
    UDP,
}

impl std::fmt::Display for TransportProtocol {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
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

/// A discovery service configuration.
#[derive(Debug)]
pub struct ServiceConfig<'a> {
    // Instance name.
    pub(crate) instance_name: &'a str,
    // Service host name
    pub(crate) hostname: &'a str,
    // Service domain.
    pub(crate) domain: &'a str,
    // Service transport protocol.
    pub(crate) transport_protocol: TransportProtocol,
    // Top-level domain.
    pub(crate) top_level_domain: &'a str,
    // Service properties.
    pub(crate) properties: HashMap<String, String>,
    // Disable IPv6.
    pub(crate) disable_ipv6: bool,
    // Disable IP.
    pub(crate) disable_ip: Option<IpAddr>,
    // Disable network interface.
    pub(crate) disable_network_interface: Option<&'a str>,
}

impl<'a> ServiceConfig<'a> {
    /// Creates [`ServiceConfig`] for an `mDNS-SD` discovery service.
    #[must_use]
    pub fn mdns_sd(instance_name: &'a str) -> Self {
        Self {
            instance_name,
            hostname: instance_name,
            domain: DOMAIN,
            transport_protocol: TransportProtocol::TCP,
            top_level_domain: TOP_LEVEL_DOMAIN,
            properties: HashMap::new(),
            disable_ipv6: false,
            disable_ip: None,
            disable_network_interface: None,
        }
    }

    /// Sets a discovery service property.
    ///
    /// An example of property could be the server scheme.
    /// i.e. ("scheme", "http")
    #[must_use]
    pub fn property(mut self, property: (impl Into<String>, impl Into<String>)) -> Self {
        let _ = self.properties.insert(property.0.into(), property.1.into());
        self
    }

    /// Sets the service hostname.
    ///
    /// An example might be `tosca`.
    #[must_use]
    pub const fn hostname(mut self, hostname: &'a str) -> Self {
        self.hostname = hostname;
        self
    }

    /// Sets the service transport protocol.
    #[must_use]
    pub const fn transport_protocol(mut self, transport_protocol: TransportProtocol) -> Self {
        self.transport_protocol = transport_protocol;
        self
    }

    /// Sets the service domain.
    ///
    ///
    /// The domain searched by the client service. i.e. tosca
    #[must_use]
    pub const fn domain(mut self, domain: &'a str) -> Self {
        self.domain = domain;
        self
    }

    /// Sets the service top-level domain.
    ///
    /// A common top-level domain is `.local`.
    #[must_use]
    pub const fn top_level_domain(mut self, top_level_domain: &'a str) -> Self {
        self.top_level_domain = top_level_domain;
        self
    }

    /// Excludes devices with `IPv6` interfaces from the discovery service.
    #[must_use]
    pub const fn disable_ipv6(mut self) -> Self {
        self.disable_ipv6 = true;
        self
    }

    /// Excludes the device with the given `IP` from the discovery service.
    #[must_use]
    #[inline]
    pub fn disable_ip(mut self, ip: impl Into<IpAddr>) -> Self {
        self.disable_ip = Some(ip.into());
        self
    }

    /// Disables the given network interface from the discovery service.
    #[must_use]
    pub const fn disable_network_interface(mut self, network_interface: &'a str) -> Self {
        self.disable_network_interface = Some(network_interface);
        self
    }
}

// A new service.
pub(crate) struct Service(mdns_sd::ServiceDaemon);

impl Service {
    // Runs a service.
    #[inline]
    pub(crate) fn run(
        service_config: ServiceConfig<'_>,
        server_address: Ipv4Addr,
        port: u16,
    ) -> Result<Self> {
        Ok(Self(mdns_sd_impl::run(
            service_config,
            server_address,
            port,
        )?))
    }

    // Shutdowns a service.
    #[inline]
    pub(crate) fn shutdown(self) -> Result<()> {
        let rx = self.0.shutdown()?;

        match rx
            .recv()
            .map_err(|e| Error::new(ErrorKind::Service, e.to_string()))?
        {
            mdns_sd::DaemonStatus::Shutdown => Ok(()),
            status => Err(Error::new(
                ErrorKind::Service,
                format!("Unexpected daemon status after shutdown: {status:?}"),
            )),
        }
    }
}

mod mdns_sd_impl {
    use std::net::{IpAddr, Ipv4Addr};

    use mdns_sd::{IfKind, ServiceDaemon, ServiceInfo};

    use tracing::info;

    use crate::error::{Error, ErrorKind};

    use super::ServiceConfig;

    impl From<mdns_sd::Error> for Error {
        fn from(e: mdns_sd::Error) -> Self {
            Self::new(ErrorKind::Service, e.to_string())
        }
    }

    impl From<std::io::Error> for Error {
        fn from(e: std::io::Error) -> Self {
            Self::new(ErrorKind::NotFoundAddress, e.to_string())
        }
    }

    pub(super) fn run(
        service_config: ServiceConfig<'_>,
        server_address: Ipv4Addr,
        server_port: u16,
    ) -> std::result::Result<ServiceDaemon, Error> {
        // Create a new mDNS service daemon
        let mdns = ServiceDaemon::new()?;

        // Disable IPv6.
        if service_config.disable_ipv6 {
            mdns.disable_interface(IfKind::IPv6)?;
        }

        // Disable IP address.
        if let Some(ip) = service_config.disable_ip {
            mdns.disable_interface(ip)?;
        }

        // Disable network interface.
        if let Some(network_interface) = service_config.disable_network_interface {
            mdns.disable_interface(network_interface)?;
        }

        // Create a hostname.
        let hostname = format!(
            "{}.{}.",
            service_config.hostname, service_config.top_level_domain
        );

        // Create a service type.
        let service_type = format!(
            "_{}._{}.{}.",
            service_config.domain,
            service_config.transport_protocol.name(),
            service_config.top_level_domain
        );

        info!("Service instance name: {}", service_config.instance_name);
        info!("Service port: {}", server_port);
        info!("Service domain: {}", service_config.domain);
        info!(
            "Service transport protocol: {}",
            service_config.transport_protocol.name()
        );
        info!(
            "Service top-level domain: {}",
            service_config.top_level_domain
        );
        info!("Service type: {}", service_type);
        info!(
            "Device reachable at this hostname: {}:{}",
            &hostname[0..hostname.len() - 1],
            server_port
        );

        let service = ServiceInfo::new(
            // Service type
            &service_type,
            // Service instance name
            service_config.instance_name,
            // DNS hostname.
            //
            // For the same hostname in the same local network, the service resolves
            // in the same addresses. It is used for A (IPv4) and AAAA (IPv6)
            // records.
            &hostname,
            // Considered IP address which allow to reach out the service.
            IpAddr::V4(server_address),
            // Port on which the service listens to. It has to be same of the
            // server.
            server_port,
            // Service properties
            service_config.properties,
        )?
        .enable_addr_auto();

        mdns.register(service)?;

        Ok(mdns)
    }
}
