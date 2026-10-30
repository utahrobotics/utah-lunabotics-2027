# embedded-teensy

Firmware for the Teensy 4.1, built on [teensy4-rs](https://github.com/mciantyre/teensy4-rs)
(`teensy4-bsp`) and the Embassy async runtime; currently a work in progress.

## Current Status(as of 67c6a16)

Embassy executor runs on the board. A blink task toggles the LED once a second,
woken by a GPT2 interrupt. The time driver (`src/time_driver.rs`) is still a stub.

## Development Environment Setup

Linux only(you can probably make it work on MacOS/Unix with some effort), and only guaranteed on Ubuntu 22.04 for now. Disclaimers aside run setup with:
```sh
./setup.sh
```

This installs:

- the `thumbv7em-none-eabihf` target and `llvm-tools`, for the repo's pinned nightly;
- `cargo-binutils` (for `rust-objcopy`);
- PJRC's `teensy_loader_cli`;
- PJRC's udev rules, so flashing doesn't need sudo.


## Build and flash

Run from inside `embedded-teensy/`. From root, cargo builds the robot software instead:

```sh
cargo run --release
```

Press the button on the Teensy when the loader says `Waiting for Teensy device...`.

-  The loader's `Read "…hex": N bytes` line should
  show tens of KB (about 22 KB at this stage). If it says 0 bytes, the build didn't use the Teensy
  memory map: check that `build.rs` sits next to `Cargo.toml`, and press Ctrl-C instead of the
  button.
- There's no debug probe on a stock Teensy; so no probe-rs and no breakpoints. A panic blinks SOS on the LED.

## Layout

| File | What it does |
|---|---|
| `Cargo.toml` | Dependencies. Its own `[workspace]` keeps the robot-PC workspace from building it |
| `.cargo/config.toml` | Build target, plus the runner: build → HEX → `teensy_loader_cli` |
| `build.rs` | Points the linker at teensy4-bsp's memory map (`t4link.x`) |
| `src/main.rs` | Board setup and tasks |
| `src/time_driver.rs` | Embassy time driver on GPT1 (Johnny) |
| `setup.sh` | One-time dependency setup |
