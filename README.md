# `ascot`

[![Actions][actions badge]][actions]
[![Codecov][codecov badge]][codecov]
[![LICENSE][license badge]][license]

This repository is organized as a Cargo workspace that includes several crates.
The main crate is [ascot](./crates/ascot), a Rust library that provides a set of
APIs and interfaces to:

- Create and manage HTTP `Rest` routes
- Define a device and its methods
- Associate some hazards to a device

It can even run on an embedded system since it is a `no_std` library.

The [ascot-os](./crates/ascot-os) and [ascot-esp32c3](./crates/ascot-esp32c3)
are two Rust libraries which make use of the `ascot` library as dependency to
define the APIs for their respective architectures.

The `ascot-os` library is thought for firmware which run on operating systems.
In the [ascot-os/examples](./crates/ascot-os/examples) directory, two different
device firmware have been implemented as examples: a
[light](./crates/ascot-os/examples/light) and a [fridge](./crates/ascot-os/examples/fridge).

The `ascot-esp32c3` library is thought for firmware which run on a `ESP32-C3`
board.
In the [ascot-esp32c3/devices](./crates/ascot-esp32c3/devices) directory, a simple
device firmware has been implemented as example: a [light](./crates/ascot-esp32c3/devices/light).

## Building

To build the entire workspace with the `debug` profile, run the following
command from the root of the repository:

```console
cargo build
```

To build this workspace with a `release` profile, which enables all time and
memory optimizations, run:

```console
cargo build --release
```

To build only a specific crate, navigate to its corresponding subdirectory
inside [crates](./crates) and run the same build commands described above.

> [!NOTE]
> The `ascot-esp32c3` crate is not part of the workspace. It must
be built separately because it targets a specific architecture
(`riscv32imc-esp-espidf`) and requires a different build process.

<!-- Links -->
[actions]: https://github.com/SoftengPoliTo/ascot/actions
[codecov]: https://codecov.io/gh/SoftengPoliTo/ascot
[license]: https://github.com/SoftengPoliTo/ascot/blob/master/LICENSE

<!-- Badges -->
[actions badge]: https://github.com/SoftengPoliTo/ascot/workflows/ci/badge.svg
[codecov badge]: https://codecov.io/gh/SoftengPoliTo/ascot/branch/master/graph/badge.svg
[license badge]: https://img.shields.io/badge/license-MIT-blue.svg
