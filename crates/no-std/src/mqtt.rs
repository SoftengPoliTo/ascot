use alloc::boxed::Box;

use embassy_net::{dns::DnsQueryType, tcp::TcpSocket, IpAddress, Stack};

use embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex;
use embassy_sync::mutex::Mutex;

use rust_mqtt::client::client::MqttClient;
use rust_mqtt::client::client_config::ClientConfig;
use rust_mqtt::packet::v5::publish_packet::QualityOfService;
use rust_mqtt::utils::rng_generator::CountingRng;

use log::info;

use crate::error::Error;

/// A network packet broker data.
pub enum BrokerData {
    /// URL and port.
    Url(&'static str, u16),

    /// [`IpAddress`] and port.
    Ip(IpAddress, u16),
}

impl BrokerData {
    /// Creates a [`BrokerData`] from `URL` and `port`.
    #[must_use]
    pub const fn url(url: &'static str, port: u16) -> Self {
        Self::Url(url, port)
    }

    /// Creates a [`BrokerData`] from [`IpAddress`] and `port`.
    #[must_use]
    pub const fn ip(ip: IpAddress, port: u16) -> Self {
        Self::Ip(ip, port)
    }
}

pub(crate) struct Mqtt {
    pub(crate) client:
        Mutex<CriticalSectionRawMutex, MqttClient<'static, TcpSocket<'static>, 5, CountingRng>>,
}

impl Mqtt {
    pub(crate) async fn new(stack: Stack<'static>, broker: BrokerData) -> Result<Self, Error> {
        let rx_buffer = Box::leak(Box::new([0u8; 1024]));
        let tx_buffer = Box::leak(Box::new([0u8; 1024]));
        let recv_buffer = Box::leak(Box::new([0u8; 80]));
        let write_buffer = Box::leak(Box::new([0u8; 80]));

        let mut socket = TcpSocket::new(stack, &mut rx_buffer[..], &mut tx_buffer[..]);

        let remote_endpoint = match broker {
            BrokerData::Url(url, port) => {
                let address = stack.dns_query(url, DnsQueryType::A).await.map(|a| a[0])?;
                (address, port)
            }

            BrokerData::Ip(ip, port) => (ip, port),
        };

        info!("Connecting to socket...");

        socket.connect(remote_endpoint).await?;

        info!("Connected to socket!");

        let mut config = ClientConfig::new(
            rust_mqtt::client::client_config::MqttVersion::MQTTv5,
            CountingRng(20000),
        );

        config.add_max_subscribe_qos(QualityOfService::QoS1);

        config.max_packet_size = 100;

        let client = MqttClient::<_, 5, _>::new(
            socket,
            &mut write_buffer[..],
            80,
            &mut recv_buffer[..],
            80,
            config,
        );

        Ok(Self {
            client: Mutex::new(client),
        })
    }

    pub(crate) async fn connect(&mut self) -> Result<(), Error> {
        self.client
            .lock()
            .await
            .connect_to_broker()
            .await
            .map_err(core::convert::Into::into)
    }

    pub(crate) async fn publish(&mut self, topic: &str, payload: &[u8]) -> Result<(), Error> {
        self.client
            .lock()
            .await
            .send_message(
                topic,
                payload,
                rust_mqtt::packet::v5::publish_packet::QualityOfService::QoS1,
                true,
            )
            .await
            .map_err(core::convert::Into::into)
    }
}
