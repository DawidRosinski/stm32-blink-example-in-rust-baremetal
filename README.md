# Board
STM32F4Discovery

# Install

This command will install Rust:

`curl https://sh.rustup.rs -sSf | sh`

`cargo install cargo-binutils`

`rustup component add llvm-tools-preview`

`rustup target add thumbv7em-none-eabihf`

# Build

`cargo build --release`

`arm-none-eabi-objcopy.exe -O ihex target/thumbv7em-none-eabihf/release/stm32-blink-example-in-rust-baremetal target/thumbv7em-none-eabihf/release/stm32-blink-example-in-rust-baremetal.hex`

# Memory map

`arm-none-eabi-objdump.exe -Ct target/thumbv7em-none-eabihf/release/stm32-blink-example-in-rust-baremetal | sort`