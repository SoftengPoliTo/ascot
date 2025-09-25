use core::cell::OnceCell;
use core::net::{Ipv4Addr, Ipv6Addr};

use esp_hal::rng::Rng;

use embassy_executor::Spawner;

use embassy_sync::blocking_mutex::raw::NoopRawMutex;
use embassy_sync::blocking_mutex::CriticalSectionMutex;
use embassy_sync::signal::Signal;

use embassy_net::Stack;

use edge_mdns::buf::VecBufAccess;
use edge_mdns::domain::base::Ttl;
use edge_mdns::host::{Host, Service, ServiceAnswers};
use edge_mdns::io::{self, DEFAULT_SOCKET};
use edge_mdns::HostAnswersMdnsHandler;

use edge_nal::UdpSplit;
use edge_nal_embassy::{Udp, UdpBuffers};

use log::info;

use crate::error::{Error, Result};

// Hostname
const HOSTNAME: &str = "ascot";
// Domain name
const DOMAIN_NAME: &str = "ascot";
// mDNS stack size
const MDNS_STACK_SIZE: usize = 2;
// Buffer length
const BUFFER_LENGTH: usize = 1500;

static RNG: CriticalSectionMutex<OnceCell<Rng>> = CriticalSectionMutex::new(OnceCell::new());

/// The `mDNS-SD` service.
pub struct Mdns {
    hostname: &'static str,
    domain_name: &'static str,
    properties: &'static [(&'static str, &'static str)],
    rng: Rng,
}

impl Mdns {
    /// Creates the [`Mdns`] service.
    #[must_use]
    pub const fn new(rng: Rng) -> Self {
        Self {
            hostname: HOSTNAME,
            domain_name: DOMAIN_NAME,
            properties: &[],
            rng,
        }
    }

    /// Sets the `mDNS-SD` hostname.
    #[must_use]
    pub const fn hostname(mut self, hostname: &'static str) -> Self {
        self.hostname = hostname;
        self
    }

    /// Sets the `mDNS-SD` domain name.
    #[must_use]
    pub const fn domain_name(mut self, domain_name: &'static str) -> Self {
        self.domain_name = domain_name;
        self
    }

    /// Sets the `mDNS-SD` properties.
    #[must_use]
    pub const fn properties(mut self, properties: &'static [(&'static str, &'static str)]) -> Self {
        self.properties = properties;
        self
    }

    pub(crate) fn run(self, stack: Stack<'static>, port: u16, spawner: Spawner) -> Result<()> {
        RNG.lock(|c| _ = c.set(self.rng));

        let ipv4 = stack
            .config_v4()
            .ok_or(Error::new(
                crate::error::ErrorKind::MDns,
                "Unable to retrieve IPv4 configuration.",
            ))?
            .address
            .address();

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
            "About to run an mDNS service with name `{}` of type `HTTP` \
                on port `{port}`.",
            self.domain_name
        );

        let service = Service {
            name: self.domain_name,
            priority: 1,
            weight: 5,
            service: "_https",
            protocol: "_tcp",
            port,
            service_subtypes: &[],
            txt_kvs: self.properties,
        };

        spawner
            .spawn(run_mdns_task(stack, host, service))
            .map_err(core::convert::Into::into)
    }
}

#[embassy_executor::task]
async fn run_mdns_task(stack: Stack<'static>, host: Host<'static>, service: Service<'static>) {
    let (recv_buf, send_buf) = (
        VecBufAccess::<NoopRawMutex, BUFFER_LENGTH>::new(),
        VecBufAccess::<NoopRawMutex, BUFFER_LENGTH>::new(),
    );

    let buffers: UdpBuffers<MDNS_STACK_SIZE, BUFFER_LENGTH, BUFFER_LENGTH, 2> = UdpBuffers::new();
    let udp = Udp::new(stack, &buffers);

    let mut socket = io::bind(&udp, DEFAULT_SOCKET, Some(Ipv4Addr::UNSPECIFIED), None)
        .await
        .expect("Impossible to create the socket");

    let (recv, send) = socket.split();

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
    .expect("mDNS task failed");
}
