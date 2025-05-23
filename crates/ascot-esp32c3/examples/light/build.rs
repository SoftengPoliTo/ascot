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

fn main() {
    let cfg = std::path::Path::new("cfg.toml");

    // Checks whether device configuration exists
    assert!(
        cfg.exists(),
        "A `cfg.toml` file with Wi-Fi credentials is required! Use `cfg.toml.example` as a template."
    );

    // Track the `cfg.toml` file so that a rebuild is done upon changes to it
    embuild::cargo::track_file(cfg);

    let device_config = DEVICE_CONFIG;
    assert!(
        !(device_config.ssid == "your Wi-Fi SSID"
            || device_config.password == "your Wi-Fi password"),
        "Wi-Fi credentials should be set up in `cfg.toml` file!"
    );

    embuild::espidf::sysenv::output();
}
