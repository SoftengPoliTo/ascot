use ascot::{device::DeviceData, route::RouteConfigs};
use log::*;

use embassy_time::{Duration, Timer};

use esp_wifi::wifi::{Configuration, ClientConfiguration, WifiController, WifiEvent, WifiState};

use crate::error::{Result, ErrorKind, Error};

pub struct S {
    d: DeviceData
}

impl S {
    pub fn new() -> Self {
        Self {
            d: DeviceData::new(ascot::device::DeviceKind::Light, ascot::device::DeviceEnvironment::Esp32, "main", RouteConfigs::new()),
        }
    }
}

pub struct Wifi {
    controller: WifiController<'static>,
}

impl Wifi {
    pub fn new(controller: WifiController<'static>) -> Self {
        Self { controller }
    }

    pub fn configure(&mut self, ssid: &str, password: &str) -> Result<()> {
        if ssid.is_empty() {
            return Err(Error::new(ErrorKind::WiFi, "Missing Wi-Fi SSID"));
        }

        if password.is_empty() {
            return Err(Error::new(ErrorKind::WiFi, "Missing Wi-Fi password"));
        }

        let client_config = Configuration::Client(ClientConfiguration {
            ssid: ssid.into(),
            password: password.into(),
            ..Default::default()
        });

        self.controller.set_configuration(&client_config)?;

        Ok(())
    }

}

/// Task che mantiene la connessione Wi-Fi attiva, riconnettendosi automaticamente
#[embassy_executor::task]
pub async fn connect(mut wifi: Wifi) {
    info!("Wi-Fi connection task started");
    loop {
        match esp_wifi::wifi::wifi_state() {
            WifiState::StaConnected => {
                wifi.controller.wait_for_event(WifiEvent::StaDisconnected).await;
                Timer::after(Duration::from_millis(5000)).await;
            }
            _ => {}
        }

        if !matches!(wifi.controller.is_started(), Ok(true)) {
            info!("Starting Wi-Fi...");
            wifi.controller.start_async().await.unwrap();
            info!("Wi-Fi started");
        }

        info!("Attempting to connect...");
        if let Err(e) = wifi.controller.connect_async().await {
            error!("Wi-Fi connect failed: {:?}", e);
            Timer::after(Duration::from_millis(5000)).await;
        } else {
            info!("Wi-Fi connected!");
        }
    }
}
