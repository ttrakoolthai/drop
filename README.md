# Drop

**Author**: Tommy Trakoolthai

## Overview

This project implements a fall detection system using the MicroBit v2's built-in accelerometer and speaker. When the device is in free fall (less than 0.5G acceleration), it "yells" by emitting a 1KHz square wave tone through the speaker and displays an exclamation point on the LED matrix. When stationary or not falling, it remains silent and displays a single dot in the center of the LED matrix.

### Features

- Uses the `lsm303agr` crate to obtain real-time acceleration data from the LSM303AGR IMU
- Outputs a 1kHz square wave tone output through the onboard speaker
- Implements non-blocking display updates using the `Display` API
- Computes and detects falling condition using squared magnitude of $x, y, z$ vectors

### Write-up

I began by reviewing the mb2-grayscale and hello-audio examples from our in-class examples (and in our course GitHub).

I incrementally implemented the program by:
- Starting with displaying the single, center LED dot
- Next, I integrated the IMU driver to fetch acceleration data in calibrated mG units. Using this data, I then implemented the calculations required to compute the squared magnitude of the acceleration vector, followed by implementing the comparison logic used to output the "yelling" noise and exclamation point, ensuring the single,
center LED dot reappeared once stationary.

The main difficulty was using the non-blocking display while toggling the speaker at 1kHz. This required calling `display.handle_display_event()` regularly inside both the main loop and the falling loop.

Overall, the assignment was fun to implementing, especially knowing the origin of the assignment (Bart and friend).
---

## Build & Flash Instructions

### 1. **Set up the Rust environment**
```sh
rustup target add thumbv7em-none-eabihf
cargo install cargo-binutils
rustup component add llvm-tools-preview
cargo install probe-rs cargo-embed
```

### 2. **Build the firmware**
```sh
cargo build --release --target thumbv7em-none-eabihf
```

### 3. **Flash the program to the micro:bit**
```sh
cargo embed --release
```

### 4. **Run the program**
```
cargo run --release
```

---

## Sources

- https://github.com/pdx-cs-rust-embedded/mb2-grayscale
- https://github.com/pdx-cs-rust-embedded/hello-audio
- https://docs.rust-embedded.org
- ChatGPT for clarifying hardware behavior and generating comments
