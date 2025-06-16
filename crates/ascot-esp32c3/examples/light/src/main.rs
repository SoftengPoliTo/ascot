//! HTTP Server with REST API handler
//!
//! Go to 192.168.1.126 to test

use std::sync::{Arc, Mutex};

use ascot::hazards::Hazard;
use ascot::route::Route;

use ascot_esp32c3::device::{DeviceAction, ResponseBuilder};
use ascot_esp32c3::devices::light::Light;
use ascot_esp32c3::server::Server;
use ascot_esp32c3::service::ServiceConfig;
use ascot_esp32c3::wifi::Wifi;

use esp_idf_svc::hal::delay::Ets;
use esp_idf_svc::hal::gpio::PinDriver;
use esp_idf_svc::hal::prelude::Peripherals;
use esp_idf_svc::log::EspLogger;

#[toml_cfg::toml_config]
pub struct DeviceConfig {
    #[default("")]
    ssid: &'static str,
    #[default("")]
    password: &'static str,
    #[default("ascot")]
    hostname: &'static str,
    #[default("ascot")]
    service: &'static str,
}

fn main() -> ascot_esp32c3::error::Result<()> {
    // A hack to make sure that a few patches to the ESP-IDF which are
    // implemented in Rust are linked to the final executable
    esp_idf_svc::sys::link_patches();

    // Bind the log crate to the ESP Logging facilities
    EspLogger::initialize_default();

    // `async-io` uses the ESP IDF `eventfd` syscall to implement async IO.
    // If you use `tokio`, you still have to do the same as it also uses the `eventfd` syscall
    let _event = esp_idf_svc::io::vfs::MountedEventfs::mount(5)?;

    // Retrieve all esp32c3 peripherals (singleton)
    let peripherals = Peripherals::take()?;

    // Retrieve device configuration
    let device_config = DEVICE_CONFIG;

    // Connects to Wi-Fi.
    let wifi = Wifi::connect_wifi(
        device_config.ssid,
        device_config.password,
        peripherals.modem,
    )?;

    // Create the driver for the built-in led in output mode
    let mut built_in_led_output = PinDriver::output(peripherals.pins.gpio8)?;
    // Turn the built-in led off setting the impedance high
    built_in_led_output.set_high()?;
    // Delay 1ms
    Ets::delay_ms(1u32);

    // Create an atomic reference counter accessible in mutual exclusion by
    // server route.
    let temp_led_main = Arc::new(Mutex::new(built_in_led_output));

    let temp_led_on = temp_led_main.clone();
    let light_on_action = DeviceAction::new(
        // Configuration for the `PUT` turn light on route.
        Route::put("On", "/on")
            .description("Turn light on.")
            .with_hazard(Hazard::FireHazard),
        ResponseBuilder(|req| req.into_ok_response(), "Turning led on went well!"),
    )
    .body(move || {
        // Turn built-in led on.
        temp_led_on.lock().unwrap().set_low().unwrap();

        // Add a delay of 1ms
        Ets::delay_ms(1u32);

        Ok(())
    });

    let temp_led_off = temp_led_main.clone();
    let light_off_action = DeviceAction::new(
        // Configuration for the `PUT` turn light off route.
        Route::put("Off", "/off").description("Turn light off."),
        ResponseBuilder(|req| req.into_ok_response(), "Turning led off went well!"),
    )
    .body(move || {
        // Turn built-in led off.
        temp_led_off.lock().unwrap().set_high().unwrap();

        // Add a delay of 1ms
        Ets::delay_ms(1u32);

        Ok(())
    });

    let light = Light::new(light_on_action, light_off_action).build();

    Server::new(light)
        .service(
            ServiceConfig::mdns_sd(wifi.ip)
                .hostname(device_config.hostname)
                .domain_name(device_config.service),
        )
        .run()
}
