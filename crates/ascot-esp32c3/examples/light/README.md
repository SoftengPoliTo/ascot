# Light Firmware

[![LICENSE][license badge]][license]

An `Ascot` light firmware to turn the built-in LED on an `ESP32-C3`
board on and off.

It implements an `HTTP` server that manages the state of the board's
built-in LED via `REST` requests:

- `light/on` route turns the LED on
- `light/off` route turns the LED off
- `light/toggle` route toggles the LED

For each request, the server responds with the _final_ status of the operation
invoked by that request.

The board can be discovered by another node on the same network via
an `mDNS-SD` service using the default domain `ascot`.

## Build Process

To build the firmware run:

```console
cargo build --release
```

To flash and run the firmware on an `ESP32-C3` board:

```console
cargo run --release
```

> [!IMPORTANT]
> Always use the release profile [--release] when building esp-hal crate.
  The dev profile can potentially be one or more orders of magnitude
  slower than release profile, and may cause issues with timing-senstive
  peripherals and/or devices.

## Board usage on WSL

Support for connecting `USB` devices is not natively available on [Windows
Subsystem for Linux (WSL)](https://learn.microsoft.com/en-us/windows/wsl/).

In order to use the `ESP32-C3` board with `WSL`, follow this
[guide](https://learn.microsoft.com/en-us/windows/wsl/connect-usb) and manually
connect the `USB` port used by the board to `WSL`.

## Usage Prerequisites

- Rename `cfg.toml.example` to `cfg.toml` and populate it with your
Wi-Fi credentials: `SSID` and `PASSWORD`
- Connect the board to a laptop via a `USB-C` cable to view the logs
- Pin the project to a specific `nightly` version for more stability, if needed

<!-- Links -->
[license]: https://github.com/SoftengPoliTo/ascot/blob/master/LICENSE

<!-- Badges -->
[license badge]: https://img.shields.io/badge/license-MIT-blue.svg
