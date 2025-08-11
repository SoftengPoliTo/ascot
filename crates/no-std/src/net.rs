use core::net::Ipv4Addr;

use alloc::boxed::Box;

use log::*;

use esp_hal::rng::Rng;

use esp_wifi::wifi::WifiDevice;

use embassy_net::{Config, Runner, Stack, StackResources};

use embassy_time::{Duration, Timer};

pub fn new<const SOCK: usize>(mut rng: Rng, wifi_interface: WifiDevice<'static>) -> (Stack<'static>, Runner<'static, WifiDevice<'static>>) {
    let config = Config::dhcpv4(Default::default());
    let seed = (rng.random() as u64) << 32 | rng.random() as u64;
    let resources = Box::leak(Box::new(StackResources::<SOCK>::new()));

    let (stack, runner) = embassy_net::new(wifi_interface, config, resources, seed);

    (stack, runner)
}

#[embassy_executor::task]
pub async fn task(mut runner: Runner<'static, WifiDevice<'static>>) {
    runner.run().await
}

pub async fn get_ip(stack: Stack<'_>) -> Ipv4Addr {
    info!("Waiting till the link is up...");
    loop {
        if stack.is_link_up() {
            break;
        }
        Timer::after(Duration::from_millis(500)).await;
    }

    info!("Waiting to get IP address...");
    loop {
        if let Some(config) = stack.config_v4() {
            info!("Got IP: {}", config.address);
            return config.address.address();
        }
        Timer::after(Duration::from_millis(500)).await;
    }
}
