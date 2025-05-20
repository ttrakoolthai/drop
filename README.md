# Drop

**Author**: Tommy Trakoolthai

## Overview

### Controls

### Features

### Write-up

---

## Build & Flash Instructions

### 1. **Set up the Rust environment**
```
rustup target add thumbv7em-none-eabihf
cargo install cargo-binutils
rustup component add llvm-tools-preview
cargo install probe-rs cargo-embed
```
### 2. **Build the firmware**
```
cargo build --release --target thumbv7em-none-eabihf
```

### 3. **Flash the program to the micro:bit**
```
cargo embed --release
```

### 4. **Run the program**
```
cargo run --release
```

# Sources
https://docs.rust-embedded.org/discovery/microbit/
https://github.com/pdx-cs-rust-embedded
https://relm4.org/docs/next/nanorand/rand/trait.SeedableRng.html

ChatGPT for issues with code and to help with documentation.
