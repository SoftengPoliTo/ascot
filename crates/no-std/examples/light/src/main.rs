#![no_std]
#![no_main]
#![deny(
    clippy::mem_forget,
    reason = "mem::forget is generally not safe to do with esp_hal types, especially those \
    holding buffers for the duration of a data transfer."
)]

extern crate alloc;

use ascot::route::{LightOffRoute, LightOnRoute};

use esp_hal::clock::CpuClock;
use esp_hal::gpio::{Input, InputConfig, Level, Output, OutputConfig, Pull};
use esp_hal::rng::Rng;
use esp_hal::timer::{systimer::SystemTimer, timg::TimerGroup};
use esp_hal::Config;

use log::info;

use embassy_executor::Spawner;
use embassy_sync::{blocking_mutex::raw::CriticalSectionRawMutex, signal::Signal};
use embassy_time::{Duration, Timer};

use no_std::{
    devices::light::Light,
    get,
    mdns::Mdns,
    net::Net,
    server::{Server, ServerConfig},
    wifi::Wifi,
};

const MAX_HEAP_SIZE: usize = 128 * 1024;
const MILLISECONDS_TO_WAIT: u64 = 100;

// Signal which notifies the led change of state.
static NOTIFY_LED: Signal<CriticalSectionRawMutex, LedInput> = Signal::new();

// This creates a default app-descriptor required by the esp-idf bootloader.
// For more information see: <https://docs.espressif.com/projects/esp-idf/en/stable/esp32/api-reference/system/app_image_format.html#application-description>
esp_bootloader_esp_idf::esp_app_desc!();

#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    loop {}
}

#[toml_cfg::toml_config]
struct DeviceConfig {
    #[default("")]
    ssid: &'static str,
    #[default("")]
    password: &'static str,
}

#[derive(Clone, Copy)]
enum LedInput {
    On,
    Off,
    Button,
}

#[embassy_executor::task]
async fn press_button(mut button: Input<'static>) {
    loop {
        // Wait for Button Press
        button.wait_for_rising_edge().await;
        info!("Button Pressed!");

        // Notify led to change its state.
        NOTIFY_LED.signal(LedInput::Button);

        // Wait for some time before starting the loop again.
        Timer::after_millis(MILLISECONDS_TO_WAIT).await;
    }
}

// Set led to on.
fn led_on(led: &mut Output<'static>) {
    led.set_low();
    info!("Led is on!");
}

// Set led to off.
fn led_off(led: &mut Output<'static>) {
    led.set_high();
    info!("Led is off!");
}

#[embassy_executor::task]
async fn change_led(mut led: Output<'static>) {
    loop {
        // Wait for until a signal is received.
        let led_input = NOTIFY_LED.wait().await;

        match led_input {
            LedInput::On => {
                led_on(&mut led);
            }
            LedInput::Off => {
                led_off(&mut led);
            }
            LedInput::Button => {
                // Turn the led on or off depending on its current state.
                //
                // Check whether the led is on since it is a pull-up led.
                if led.is_set_high() {
                    led_on(&mut led);
                } else {
                    led_off(&mut led);
                }
            }
        }

        // Wait for some time before starting the loop again.
        Timer::after_millis(MILLISECONDS_TO_WAIT).await;
    }
}

#[esp_hal_embassy::main]
async fn main(spawner: Spawner) {
    esp_println::logger::init_logger_from_env();

    let config = Config::default().with_cpu_clock(CpuClock::max());
    let peripherals = esp_hal::init(config);

    esp_alloc::heap_allocator!(size: MAX_HEAP_SIZE);

    let timer0 = SystemTimer::new(peripherals.SYSTIMER);
    esp_hal_embassy::init(timer0.alarm0);

    info!("Embassy initialized!");

    let rng = Rng::new(peripherals.RNG);
    let timer1 = TimerGroup::new(peripherals.TIMG0);

    // Retrieve device configuration
    let device_config = DEVICE_CONFIG;

    let interfaces = Wifi::config(timer1.timer0, rng, peripherals.WIFI, spawner)
        .expect("Failed to configure Wi-Fi")
        .connect(device_config.ssid, device_config.password)
        .expect("Failed to connect to Wi-Fi");

    // The number of tasks in the stack must be increased depending on the
    // needs. If the number of task is less than the actual number of tasks,
    // there may be malfunctions.
    //
    // In this case, the value is 13 because we have:
    // - 8 server tasks
    // - 1 wifi task
    // - 1 mdns task
    // - 1 stack task
    // - 1 task to check if a button is pressed
    // - 1 task to check if a led state is changed
    let stack =
        Net::create::<13>(rng, interfaces.sta, spawner).expect("Failed to create network stack.");

    // Input button
    let button = Input::new(
        peripherals.GPIO9,
        InputConfig::default().with_pull(Pull::Up),
    );

    // Output led.
    let led = Output::new(peripherals.GPIO8, Level::High, OutputConfig::default());

    spawner
        .spawn(press_button(button))
        .expect("Impossible to spawn the task to press the button task");
    spawner
        .spawn(change_led(led))
        .expect("Impossible to spawn the task to change the led");

    let device = Light::turn_light_on(
        LightOnRoute::put("On").description("Turn light on."),
        get(|| async move {
            // Notify led to turn led on.
            NOTIFY_LED.signal(LedInput::On);

            log::info!("Led turned on through GET route!");

            // Wait for some time before starting the loop again.
            Timer::after_millis(MILLISECONDS_TO_WAIT).await;
        }),
    )
    .turn_light_off(
        LightOffRoute::put("Off").description("Turn light off."),
        get(|| async move {
            // Notify led to turn led off.
            NOTIFY_LED.signal(LedInput::Off);

            log::info!("Led turned off through GET route!");

            // Wait for some time before starting the loop again.
            Timer::after_millis(MILLISECONDS_TO_WAIT).await;
        }),
    )
    .build();

    let config = ServerConfig::new()
        .start_read_request(Duration::from_secs(5))
        .persistent_start_read_request(Duration::from_secs(1))
        .read_request(Duration::from_secs(1))
        .write(Duration::from_secs(1))
        .keep_connection_alive();

    // Create a server which manages 8 concurrent requests.
    Server::<8, _>::new(device, config, Mdns::new(rng))
        .run(stack, spawner)
        .await
        .expect("Failed to run the server");
}
