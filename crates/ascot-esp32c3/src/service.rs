use core::net::{Ipv4Addr, Ipv6Addr};

use edge_mdns::buf::VecBufAccess;
use edge_mdns::domain::base::Ttl;
use edge_mdns::host::{Service, ServiceAnswers};
use edge_mdns::io::{self, DEFAULT_SOCKET};
use edge_mdns::{HostAnswersMdnsHandler, host::Host};

use edge_nal::UdpSplit;
use edge_nal_std::Stack;

use embassy_sync::blocking_mutex::raw::NoopRawMutex;
use embassy_sync::signal::Signal;

use esp_idf_svc::hal::task::block_on;

use log::info;

use rand::{RngCore, thread_rng};

use crate::error::Result;

// Service hostname
const SERVICE_HOSTNAME: &str = "ascot";

// Service name
const SERVICE_NAME: &str = "ascot";

#[repr(u8)]
enum Algorithm {
    MdnsSd,
}

/// A mDNS-SD service configurator.
pub struct ServiceConfig {
    algorithm: Algorithm,
    http_address: Ipv4Addr,
    hostname: &'static str,
    domain_name: &'static str,
    properties: &'static [(&'static str, &'static str)],
}

impl ServiceConfig {
    /// Creates a new [`ServiceConfig`] instance for `mDNS-SD`.
    #[must_use]
    pub const fn mdns_sd(http_address: Ipv4Addr) -> Self {
        Self {
            algorithm: Algorithm::MdnsSd,
            http_address,
            hostname: SERVICE_HOSTNAME,
            domain_name: SERVICE_NAME,
            properties: &[],
        }
    }

    /// Sets a service host name.
    #[must_use]
    pub const fn hostname(mut self, hostname: &'static str) -> Self {
        self.hostname = hostname;
        self
    }

    /// Sets a service domain name.
    #[must_use]
    pub const fn domain_name(mut self, domain_name: &'static str) -> Self {
        self.domain_name = domain_name;
        self
    }

    /// Sets service properties.
    #[must_use]
    pub const fn properties(mut self, properties: &'static [(&'static str, &'static str)]) -> Self {
        self.properties = properties;
        self
    }
}

async fn mdns_sd_service(service_config: ServiceConfig) -> Result<()> {
    let ServiceConfig {
        algorithm: _,
        http_address,
        hostname,
        domain_name,
        properties,
    } = service_config;

    // Create stack
    let stack = Stack::new();

    // Create sender and receiver buffers for mdns-sd protocol.
    let (recv_buf, send_buf): (
        VecBufAccess<NoopRawMutex, 1500>,
        VecBufAccess<NoopRawMutex, 1500>,
    ) = (VecBufAccess::new(), VecBufAccess::new());

    // No ipv6 up and running.
    // To have it running, we need to get at least a link-local ipv6 addr
    // first, using an `esp-idf-sys` API call once the wifi is up and running:
    // `esp_idf_svc::sys::esp_netif_create_ip6_linklocal`.
    // Moreover, we can't just pass "0" for the interface.
    // We need to pass `wifi.sta_netif().index()`
    // Sometimes, "0" does work on PCs, but not with ESP-IDF.
    // This API is very picky about having a correct ipv6-capable
    // interface rather than just "all" (= 0).
    let mut socket = io::bind(&stack, DEFAULT_SOCKET, Some(Ipv4Addr::UNSPECIFIED), None).await?;

    let (recv, send) = socket.split();

    info!(
        "About to run an mDNS responder reachable from a PC. \
             It will be addressable using {hostname}.local, \
             so try to `ping {hostname}.local`."
    );

    let host = Host {
        hostname,
        ipv4: http_address,
        ipv6: Ipv6Addr::UNSPECIFIED,
        ttl: Ttl::from_secs(60),
    };

    info!(
        "About to run an mDNS service with name `{domain_name}` of type `HTTPS` \
             on port `443`."
    );

    let service = Service {
        name: domain_name,
        priority: 1,
        weight: 5,
        service: "_https",
        protocol: "_tcp",
        port: 443,
        service_subtypes: &[],
        txt_kvs: properties,
    };

    // A way to notify the mDNS responder that the data in `Host` had changed
    // Not necessary for this example, because the data is hard-coded
    let signal = Signal::new();

    let mdns = io::Mdns::<NoopRawMutex, _, _, _, _>::new(
        Some(Ipv4Addr::UNSPECIFIED),
        // No ipv6 up and running.
        None,
        recv,
        send,
        recv_buf,
        send_buf,
        |buf| thread_rng().fill_bytes(buf),
        &signal,
    );

    mdns.run(HostAnswersMdnsHandler::new(ServiceAnswers::new(
        &host, &service,
    )))
    .await
    .map_err(core::convert::Into::into)
}

pub(crate) struct InternalService;

impl InternalService {
    #[inline]
    pub(crate) fn run(service_config: ServiceConfig) -> Result<()> {
        block_on(match service_config.algorithm {
            Algorithm::MdnsSd => mdns_sd_service(service_config),
        })
    }
}
