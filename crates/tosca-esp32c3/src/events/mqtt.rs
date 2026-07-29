use core::num::NonZero;

use alloc::boxed::Box;

use embassy_net::{IpAddress, Stack, tcp::TcpSocket};

use embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex;
use embassy_sync::mutex::Mutex;

use embassy_time::{Duration, with_timeout};

use rust_mqtt::Bytes;
use rust_mqtt::buffer::AllocBuffer;
use rust_mqtt::client::event::Event;
use rust_mqtt::client::options::{ConnectOptions, PublicationOptions, TopicReference};
use rust_mqtt::client::{Client, MqttError};
use rust_mqtt::types::{MqttString, ReasonCode, TopicName};

use log::{info, warn};

use crate::error::{Error, ErrorKind};
use crate::mk_static;

// Timeout duration for broker operations, in seconds.
const BROKER_TIMEOUT: u64 = 2;
// Maximum packet size, in bytes, accepted from the broker.
const MAX_PACKET_SIZE: u32 = 100;
// Size of the socket transmission and reception buffers.
const BUFFER_SIZE: usize = 1024;

// Small fixed capacities suitable for a single-flight MQTT publisher.
type MqttClient = Client<'static, TcpSocket<'static>, AllocBuffer, 1, 1, 1, 1>;

pub(crate) struct Mqtt {
    pub(crate) client: Mutex<CriticalSectionRawMutex, MqttClient>,
}

impl Mqtt {
    #[inline]
    pub(crate) fn new() -> Self {
        let buffer = mk_static!(AllocBuffer, AllocBuffer);
        let client = MqttClient::new(buffer);

        Self {
            client: Mutex::new(client),
        }
    }

    #[inline]
    pub(crate) async fn connect(
        &mut self,
        stack: Stack<'static>,
        remote_endpoint: (IpAddress, u16),
    ) -> Result<(), Error> {
        let rx_buffer = Box::leak(Box::new([0u8; BUFFER_SIZE]));
        let tx_buffer = Box::leak(Box::new([0u8; BUFFER_SIZE]));

        let mut socket = TcpSocket::new(stack, &mut rx_buffer[..], &mut tx_buffer[..]);

        info!(
            "Connecting to broker socket with address `{}` on port `{}`...",
            remote_endpoint.0, remote_endpoint.1
        );

        with_timeout(
            Duration::from_secs(BROKER_TIMEOUT),
            socket.connect(remote_endpoint),
        )
        .await
        .map_err(|_| Error::new(ErrorKind::Timeout, "Broker not available"))??;

        info!("Connected to broker socket");

        let connect_options = ConnectOptions::new()
            .clean_start()
            .maximum_packet_size(NonZero::new(MAX_PACKET_SIZE).unwrap_or(NonZero::<u32>::MAX));

        let mut client = self.client.lock().await;

        match client.connect(socket, &connect_options, None).await {
            Ok(connect_info) => {
                info!("Connected to MQTT broker: {connect_info:?}");
                Ok(())
            }
            Err(error) => {
                // A failed MQTT exchange can leave the client in a recovery-required
                // state. Abort it so a later connection attempt can reuse this wrapper.
                client.abort().await;
                Err(error.into())
            }
        }
    }

    #[inline]
    pub(crate) async fn publish(&mut self, topic: &str, payload: &[u8]) -> Result<(), Error> {
        let mqtt_topic = MqttString::from_str(topic)
            .map_err(|_| Error::new(ErrorKind::Mqtt, "Invalid MQTT topic string"))?;
        let topic_name = TopicName::new(mqtt_topic)
            .ok_or_else(|| Error::new(ErrorKind::Mqtt, "Invalid MQTT topic name"))?;

        let publication_options = PublicationOptions::new(TopicReference::Name(topic_name))
            .at_least_once()
            .retain();

        let mut client = self.client.lock().await;

        let packet_identifier = client
            .publish(&publication_options, Bytes::from(payload))
            .await
            .map_err(<MqttError<'_> as Into<Error>>::into)?
            .ok_or_else(|| Error::new(ErrorKind::Mqtt, "Missing MQTT packet identifier"))?;

        loop {
            let header = with_timeout(Duration::from_secs(BROKER_TIMEOUT), client.poll_header())
                .await
                .map_err(|_| Error::new(ErrorKind::Timeout, "MQTT broker response timeout"))?
                .map_err(<MqttError<'_> as Into<Error>>::into)?;

            match client
                .poll_body(header)
                .await
                .map_err(<MqttError<'_> as Into<Error>>::into)?
            {
                Event::PublishAcknowledged(ack) if ack.packet_identifier == packet_identifier => {
                    if ack.reason_code == ReasonCode::NoMatchingSubscribers {
                        warn!("{}", Error::from(ack.reason_code));
                    }

                    return Ok(());
                }
                Event::PublishRejected(rejection)
                    if rejection.packet_identifier == packet_identifier =>
                {
                    return Err(rejection.reason_code.into());
                }
                _ => {}
            }
        }
    }

    #[inline]
    pub(crate) async fn send_ping(&mut self) -> Result<(), Error> {
        let mut client = self.client.lock().await;

        client
            .ping()
            .await
            .map_err(<MqttError<'_> as Into<Error>>::into)?;

        loop {
            let header = with_timeout(Duration::from_secs(BROKER_TIMEOUT), client.poll_header())
                .await
                .map_err(|_| Error::new(ErrorKind::Timeout, "MQTT broker ping timeout"))?
                .map_err(<MqttError<'_> as Into<Error>>::into)?;

            if let Event::Pingresp = client
                .poll_body(header)
                .await
                .map_err(<MqttError<'_> as Into<Error>>::into)?
            {
                return Ok(());
            }
        }
    }
}
