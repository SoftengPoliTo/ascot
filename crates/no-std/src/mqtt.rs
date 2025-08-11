use alloc::boxed::Box;

use embassy_net::Stack;
use embassy_net::dns::DnsQueryType;
use embassy_net::tcp::TcpSocket;

use embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex;
use embassy_sync::mutex::Mutex;

use rust_mqtt::client::client::MqttClient;
use rust_mqtt::client::client_config::ClientConfig;
use rust_mqtt::packet::v5::publish_packet::QualityOfService;
use rust_mqtt::packet::v5::reason_codes::ReasonCode;
use rust_mqtt::utils::rng_generator::CountingRng;

use log::{error, info};

use crate::error::Result;

pub enum Broker {
    /// URL and port.
    Url(&'static str, u16),
    /// IP address and port.
    Ip(embassy_net::IpAddress, u16),
}

impl Broker {
    pub fn url(url: &'static str, port: u16) -> Self {
        Self::Url(url, port)
    }

    pub fn ip(ip: embassy_net::IpAddress, port: u16) -> Self {
        Self::Ip(ip, port)
    }
}

pub struct Mqtt {
    client: Mutex<CriticalSectionRawMutex, MqttClient<'static, TcpSocket<'static>, 5, CountingRng>>,
}

impl Mqtt {
    pub async fn new(stack: Stack<'static>, broker: Broker) -> Self {
        let rx_buffer = Box::leak(Box::new([0u8; 1024]));
        let tx_buffer = Box::leak(Box::new([0u8; 1024]));
        let recv_buffer = Box::leak(Box::new([0u8; 80]));
        let write_buffer = Box::leak(Box::new([0u8; 80]));

        // Converti i Box in slice mutabili (Box<[T]> implementa DerefMut)
        let mut socket = TcpSocket::new(stack, &mut rx_buffer[..], &mut tx_buffer[..]);

        let remote_endpoint = match broker {
            Broker::Url(url, port) => {
                let address = match stack.dns_query(url, DnsQueryType::A).await.map(|a| a[0]) {
                    Ok(address) => address,
                    Err(e) => {
                        error!("DNS lookup error: {e:?}");
                        panic!();
                    }
                };
                (address, port)
            }
            Broker::Ip(ip, port) => (ip, port),
        };

        info!("Connecting...");
        if let Err(e) = socket.connect(remote_endpoint).await {
            error!("Connect error: {:?}", e);
            panic!();
        }
        info!("Connected!");

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

        Self {
            client: Mutex::new(client),
        }
    }

    pub async fn connect(&mut self) -> Result<()> {
        match self.client.lock().await.connect_to_broker().await {
            Ok(()) => {}
            Err(mqtt_error) => match mqtt_error {
                ReasonCode::NetworkError => {
                    error!("MQTT Network Error");
                    panic!();
                }
                _ => {
                    error!("Other MQTT Error: {:?}", mqtt_error);
                    panic!();
                }
            },
        }

        Ok(())
    }

    pub async fn publish(&mut self, topic: &'static str, payload: &[u8]) -> Result<()> {
        match self
            .client
            .lock()
            .await
            .send_message(
                topic,
                payload,
                rust_mqtt::packet::v5::publish_packet::QualityOfService::QoS1,
                true,
            )
            .await
        {
            Ok(()) => {}
            Err(mqtt_error) => match mqtt_error {
                ReasonCode::NetworkError => {
                    error!("MQTT Network Error");
                    panic!();
                }
                _ => {
                    error!("Other MQTT Error: {:?}", mqtt_error);
                    panic!();
                }
            },
        }

        Ok(())
    }
}
