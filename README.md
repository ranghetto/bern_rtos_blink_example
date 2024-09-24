# Bern RTOS blink example

## Pre-requisites

- **probe-rs:** install with install scripts or from package manager ([instructions](https://probe.rs/docs/getting-started/installation/));
- **rust nightly channel:** install with command `rustup toolchain install nightly`;
- **toolchain:** install the toolchain you need using `rustup target your-target-here --toolchain nightly`

## Configuration

In `.cargo/config.toml` set your chip code after `--chip` flag in `runner` configuration. You can find the list of chips [here](https://probe.rs/targets/?manufacturer=SHOW_ALL_MANUFACTURERS&family=SHOW_ALL_FAMILIES).
Then set the `target` toolchain you installed.

In `conf/conf.rs` set up the memory layout of your microcontroller. For basic usage just change memory sizes according
to the datasheet memory mapping section.

In `Cargo.toml` config the `HAL` dependency of your microcontroller and the related feature, if needed.

In `main.rs` row `33` set the frequency that works best for your use case.

## Usage
The system is built to be used with normal `cargo build` and `cargo run` commands for respectively:
- build the binary to flash in the microcontroller;
- build the binary, flash, open serial monitor and run.