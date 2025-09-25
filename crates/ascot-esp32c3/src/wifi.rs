use embassy_executor::Spawner;

use esp_hal::peripherals::WIFI;
use esp_hal::rng::Rng;
use esp_hal::timer::timg::Timer;

use esp_wifi::{
    wifi::{
        wifi_state, ClientConfiguration, Configuration, Interfaces, WifiController, WifiEvent,
        WifiState,
    },
    {wifi, EspWifiController},
};

use log::{error, info};

use crate::error::{Error, ErrorKind, Result};
use crate::mk_static;

const SECONDS_TO_WAIT_FOR_RECONNECTION: u64 = 5;

/// The `Wi-Fi` controller.
pub struct Wifi {
    _esp_wifi_controller: &'static EspWifiController<'static>,
    controller: WifiController<'static>,
    interfaces: Interfaces<'static>,
    spawner: Spawner,
}

impl Wifi {
    /// Configures the [`Wifi`] controller with the given parameters.
    ///
    /// # Errors
    ///
    /// Unable to initialize the `Wi-Fi` controller and retrieve the
    /// corresponding network interfaces.
    pub fn configure(
        timer: Timer<'static>,
        rng: Rng,
        peripherals_wifi: WIFI<'static>,
        spawner: Spawner,
    ) -> Result<Self> {
        let esp_wifi_controller =
            &*mk_static!(EspWifiController<'static>, esp_wifi::init(timer, rng)?);

        let (controller, interfaces) = wifi::new(esp_wifi_controller, peripherals_wifi)?;

        Ok(Self {
            _esp_wifi_controller: esp_wifi_controller,
            controller,
            interfaces,
            spawner,
        })
    }

    /// Connects a device to a `Wi-Fi` access point.
    ///
    /// # Errors
    ///
    /// - Missing `Wi-Fi` SSID
    /// - Missing `Wi-Fi` password
    /// - Failure to set up the `Wi-Fi` configuration
    /// - Failure to spawn the task to connect the device to the access point
    ///   via `Wi-Fi`.
    pub fn connect(mut self, ssid: &str, password: &str) -> Result<Interfaces<'static>> {
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

        self.spawner.spawn(connect(self.controller))?;

        Ok(self.interfaces)
    }
}

#[embassy_executor::task]
async fn connect(mut wifi_controller: WifiController<'static>) {
    info!("Wi-Fi connection task started");
    loop {
        if wifi_state() == WifiState::StaConnected {
            wifi_controller
                .wait_for_event(WifiEvent::StaDisconnected)
                .await;
            embassy_time::Timer::after_secs(SECONDS_TO_WAIT_FOR_RECONNECTION).await;
        }

        if !matches!(wifi_controller.is_started(), Ok(true)) {
            info!("Starting Wi-Fi...");
            wifi_controller
                .start_async()
                .await
                .map_err(Error::from)
                .expect("Impossible to start Wi-Fi");
            info!("Wi-Fi started");
        }

        info!("Attempting to connect...");
        if let Err(e) = wifi_controller.connect_async().await {
            error!("Wi-Fi connect failed: {e:?}");
            embassy_time::Timer::after_secs(SECONDS_TO_WAIT_FOR_RECONNECTION).await;
        } else {
            info!("Wi-Fi connected!");
        }
    }
}
