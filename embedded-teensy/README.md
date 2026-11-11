# embedded-teensy

Firmware for the Teensy 4.1, built on [teensy4-rs](https://github.com/mciantyre/teensy4-rs)
(`teensy4-bsp`) and the Embassy async runtime; currently a work in progress. The plan, stage by
stage, is in [ROADMAP.md](ROADMAP.md).

## Current Status (stage 3 done)

The Embassy executor runs on the board, timed by Johnny's GPT1 time driver
(`src/time_driver.rs`). A blink task toggles the LED every half second and logs each blink with
defmt, over a USB-serial adapter on pin 1 (see [Logs](#logs)).

## Development Environment Setup

Linux only(you can probably make it work on MacOS/Unix with some effort), and only guaranteed on Ubuntu 22.04 for now. Disclaimers aside run setup with:
```sh
./setup.sh
```

This installs:

- the nightly pinned in `rust-toolchain.toml`, with the `thumbv7em-none-eabihf` target and `llvm-tools`;
- `cargo-binutils` (for `rust-objcopy`);
- `defmt-print`, which decodes the firmware's logs;
- PJRC's `teensy_loader_cli`;
- PJRC's udev rules, so flashing doesn't need sudo.

It also adds you to the `dialout` group, so you can read the serial port without sudo.

## Build and flash

Run from inside `embedded-teensy/`. From root, cargo builds the robot software instead:

```sh
cargo run --release
```

Press the button on the Teensy when the loader says `Waiting for Teensy device...`.

-  The loader's `Read "…hex": N bytes` line should
  show tens of KB (about 30 KB at this stage). If it says 0 bytes, the build didn't use the Teensy
  memory map: check that `build.rs` sits next to `Cargo.toml`, and press Ctrl-C instead of the
  button.
- There's no debug probe on a stock Teensy; so no probe-rs and no breakpoints. A panic blinks SOS on the LED.

## Logs

The firmware logs with [defmt](https://defmt.ferrous-systems.com) out of LPUART6 TX (pin 1) at
500 kbaud, so reading the logs takes a 3.3 V USB-serial adapter:

| Adapter | Teensy |
|---|---|
| RX (white on our CP2102 cable) | pin 1 |
| GND (black) | GND |
| TX (green) and 5 V (red) | not connected |

Wire colors differ between cables, so go by the labels on yours, and never connect the 5 V wire
to the Teensy. Then, from `embedded-teensy/`:

```sh
defmt-print -w -e target/thumbv7em-none-eabihf/release/embedded-teensy \
    serial --path /dev/ttyUSB0 --baud 500000
```

- Start it before flashing to catch the first lines after boot. `-w` reloads the firmware file
  after each rebuild, so it can stay running.
- defmt sends encoded binary frames, so reading the port with `cat` shows garbage. That's normal.
- Keep 500 kbaud unless you change both ends. Our CP2102 adapter can't do 1 Mbaud: asked for it,
  it runs at about 921.6k, and defmt-print silently drops every garbled frame.

## Layout

| File | What it does |
|---|---|
| `Cargo.toml` | Dependencies. Its own `[workspace]` keeps the robot-PC workspace from building it |
| `rust-toolchain.toml` | Pins the nightly (the same one as the repo root), plus the Teensy target and `llvm-tools` |
| `.cargo/config.toml` | Build target, the runner (build → HEX → `teensy_loader_cli`), and `DEFMT_LOG` |
| `build.rs` | Points the linker at teensy4-bsp's memory map (`t4link.x`) and defmt's (`defmt.x`) |
| `src/main.rs` | Board setup, logging and tasks |
| `src/time_driver.rs` | Embassy time driver on GPT1 (Johnny) |
| `setup.sh` | One-time dependency setup |
| `ROADMAP.md` | The plan, stage by stage, plus the pin map |
