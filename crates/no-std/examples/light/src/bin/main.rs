#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]
#![feature(impl_trait_in_assoc_type)]
#![deny(
    clippy::mem_forget,
    reason = "mem::forget is generally not safe to do with esp_hal types, especially those \
    holding buffers for the duration of a data transfer."
)]

use esp_hal::clock::CpuClock;
use esp_hal::timer::systimer::SystemTimer;
use esp_hal::timer::timg::TimerGroup;

use esp_wifi::EspWifiController;

use log::info;

use embassy_executor::Spawner;

use embassy_time::{Duration, Timer};

use no_std::wifi::Wifi;


#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    loop {}
}

extern crate alloc;

pub const SSID: &str = env!("SSID");
pub const PASSWORD: &str = env!("PASSWORD");

// This creates a default app-descriptor required by the esp-idf bootloader.
// For more information see: <https://docs.espressif.com/projects/esp-idf/en/stable/esp32/api-reference/system/app_image_format.html#application-description>
esp_bootloader_esp_idf::esp_app_desc!();

macro_rules! mk_static {
    ($t:ty,$val:expr) => {{
        static STATIC_CELL: static_cell::StaticCell<$t> = static_cell::StaticCell::new();
        #[deny(unused_attributes)]
        let x = STATIC_CELL.uninit().write(($val));
        x
    }};
}

#[esp_hal_embassy::main]
async fn main(spawner: Spawner) {
    // generator version: 0.5.0

    esp_println::logger::init_logger_from_env();

    let config = esp_hal::Config::default().with_cpu_clock(CpuClock::max());
    let peripherals = esp_hal::init(config);

    esp_alloc::heap_allocator!(size: 64 * 1024);

    let timer0 = SystemTimer::new(peripherals.SYSTIMER);
    esp_hal_embassy::init(timer0.alarm0);

    info!("Embassy initialized!");

    let rng = esp_hal::rng::Rng::new(peripherals.RNG);
    let timer1 = TimerGroup::new(peripherals.TIMG0);

    let wifi_init = &*mk_static!(
        EspWifiController<'static>,
        esp_wifi::init(timer1.timer0, rng.clone()).expect("Failed to initialize Wi-Fi/BLE controller")
    );

    let (wifi_controller, interfaces) = esp_wifi::wifi::new(&wifi_init, peripherals.WIFI)
        .expect("Failed to initialize Wi-Fi controller");

    let mut wifi = Wifi::new(wifi_controller);
    // TODO: Use cfg.toml
    wifi.configure(SSID, PASSWORD).expect("Failed to configure Wi-Fi SSID and/or password");

    let (stack, runner) = no_std::net::new::<8>(rng, interfaces.sta);

    spawner.spawn(no_std::wifi::connect(wifi)).ok();
    spawner.spawn(no_std::net::task(runner)).ok();

    let ip = no_std::net::get_ip(stack).await;
    info!("Got IP Address: {}", ip);

    let mdns = no_std::mdns::Mdns::new();
    spawner.spawn(no_std::mdns::run_mdns_task(mdns, stack, rng)).ok();

    info!("After MDNS Spawn");

    //////// Server /////
    let app_hello = mk_static!(AppRouter<AppPropsHello>, AppPropsHello.build_app());
    let config = mk_static!(
        picoserve::Config<Duration>,
        picoserve::Config::new(picoserve::Timeouts {
            start_read_request: Some(Duration::from_secs(5)),
            persistent_start_read_request: Some(Duration::from_secs(1)),
            read_request: Some(Duration::from_secs(1)),
            write: Some(Duration::from_secs(1)),
        })
        .keep_connection_alive()
    );

    for id in 0..WEB_TASK_POOL_SIZE {
        spawner.must_spawn(web_task_hello(1, stack, app_hello, config));
    }
    /////////////////////

    let mut mqtt = no_std::mqtt::Mqtt::new(stack, no_std::mqtt::Broker::url("broker.mqtt.cool", 1883)).await;
    mqtt.connect().await.expect("Unable to connect to the broker");

    mqtt.publish("ascot/topic/test", "Ciao da Ascot 3".as_bytes()).await.expect("Unable to post the message");
}

//////////////////////////////// SERVER ///////////////////////////////
use picoserve::{routing::get, AppBuilder, AppRouter};

struct AppPropsHello;

impl AppBuilder for AppPropsHello {
    type PathRouter = impl picoserve::routing::PathRouter;

    fn build_app(self) -> picoserve::Router<Self::PathRouter> {
        // picoserve::Router::new().route("/hello", get(|| async move {
        //     log::info!("Received GET /hello");
        //     "Hello!"
        // }))
        picoserve::Router::new()
    }
}

const WEB_TASK_POOL_SIZE: usize = 1;

#[embassy_executor::task(pool_size = WEB_TASK_POOL_SIZE)]
async fn web_task_hello(
    id: usize,
    stack: embassy_net::Stack<'static>,
    app: &'static AppRouter<AppPropsHello>,
    config: &'static picoserve::Config<Duration>,
) -> ! {
    let port = 80;
    let mut tcp_rx_buffer = [0; 1024];
    let mut tcp_tx_buffer = [0; 1024];
    let mut http_buffer = [0; 2048];

    picoserve::listen_and_serve(
        id,
        app,
        config,
        stack,
        port,
        &mut tcp_rx_buffer,
        &mut tcp_tx_buffer,
        &mut http_buffer,
    )
    .await
}
///////////////////////////////////////////////////////////////////////