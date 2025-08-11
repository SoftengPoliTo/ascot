use core::net::{Ipv4Addr, Ipv6Addr};
use core::cell::OnceCell;

use esp_hal::rng::Rng;

use embassy_sync::blocking_mutex::CriticalSectionMutex;
use embassy_sync::blocking_mutex::raw::NoopRawMutex;
use embassy_sync::signal::Signal;

use embassy_net::Stack;

use edge_mdns::buf::VecBufAccess;
use edge_mdns::host::{Host, Service, ServiceAnswers};
use edge_mdns::io::{self, DEFAULT_SOCKET};
use edge_mdns::HostAnswersMdnsHandler;
use edge_mdns::domain::base::Ttl;

use edge_nal::UdpSplit;
use edge_nal_embassy::{Udp, UdpBuffers};

use log::{info, error};

use crate::error::{Error, Result};

// Hostname.
const HOSTNAME: &str = "ascot";

// Domain name.
const DOMAIN_NAME: &str = "ascot";

static RNG: CriticalSectionMutex<OnceCell<Rng>> = CriticalSectionMutex::new(OnceCell::new());

const MDNS_STACK_SIZE: usize = 2;

pub struct Mdns {
    hostname: &'static str,
    domain_name: &'static str,
    properties: &'static [(&'static str, &'static str)]
}

impl Mdns {
    /// Creates a new [`Mdns`] instance.
    pub const fn new() -> Self {
        Self {
            hostname: HOSTNAME,
            domain_name: DOMAIN_NAME,
            properties: &[],
        }
    }

    /// Sets hostname.
    #[must_use]
    pub const fn hostname(mut self, hostname: &'static str) -> Self {
        self.hostname = hostname;
        self
    }

    /// Sets domain name.
    #[must_use]
    pub const fn domain_name(mut self, domain_name: &'static str) -> Self {
        self.domain_name = domain_name;
        self
    }

    /// Sets properties.
    #[must_use]
    pub const fn properties(mut self, properties: &'static [(&'static str, &'static str)]) -> Self {
        self.properties = properties;
        self
    }

    async fn mdns_task(self, stack: Stack<'static>, rng: Rng) -> Result<()> {
        RNG.lock(|c| _ = c.set(rng));

        let ipv4 = stack.config_v4().ok_or(Error::new(crate::error::ErrorKind::Mdns, "Unable to retrieve IPv4 configuration."))?.address.address();

        let (recv_buf, send_buf) = (
            VecBufAccess::<NoopRawMutex, 1500>::new(),
            VecBufAccess::<NoopRawMutex, 1500>::new(),
        );

        let buffers: UdpBuffers<MDNS_STACK_SIZE, 1500, 1500, 2> = UdpBuffers::new();
        let udp = Udp::new(stack, &buffers);

        let mut socket = io::bind(
            &udp,
            DEFAULT_SOCKET,
            Some(Ipv4Addr::UNSPECIFIED),
            None
        )
        .await?;

        let (recv, send) = socket.split();

        info!(
            "About to run an mDNS responder reachable from a PC. \
                It will be addressable using {}.local, \
                so try to `ping {}.local`.",
                self.hostname, self.hostname
        );

        let host = Host {
            hostname: self.hostname,
            ipv4,
            ipv6: Ipv6Addr::UNSPECIFIED,
            ttl: Ttl::from_secs(60),
        };

        info!(
            "About to run an mDNS service with name `{}` of type `HTTPS` \
                on port `443`.",
                self.domain_name
        );

        let service = Service {
            name: self.domain_name,
            priority: 1,
            weight: 5,
            service: "_https",
            protocol: "_tcp",
            port: 443,
            service_subtypes: &[],
            txt_kvs: self.properties,
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
            |buf| {
                RNG.lock(|c| c.get().map(|r| r.clone().read(buf)));
            },
            &signal,
        );

        mdns.run(HostAnswersMdnsHandler::new(ServiceAnswers::new(
            &host, &service,
        )))
        .await
        .map_err(core::convert::Into::into)
    }
}

#[embassy_executor::task]
pub async fn run_mdns_task(mdns: Mdns, stack: Stack<'static>, rng: Rng) {
    if let Err(e) = mdns.mdns_task(stack, rng).await {
        error!("mDNS task failed: {:?}", e);
    }
}
