# Example project for STM32 in Rust Embedded

This is a very simple demo that can be used as a template for new projects. It configures I2C and UART to read a preasure from I2C sensor and print the value via UART. When build with debug configuration it also uses semihosting. It should work with minor changes in all STM32L0 devices and with a few more changes on other microcontrollers.

In first step remember to select proper STM32L0 family for stm32l0xx-hal in `Cargo.toml` and ensure that `memory.x` reflects your device memory configuration.

To build project:

`# cargo build`

For release version (without semihosting), first switch panic hadler in `Cargo.toml` to panic-halt and:

`# cargo build --release`

To load binary using OpenOCD and ST-link:

`# ./openocd_flash.sh target/thumbv6m-none-eabi/debug/stm32_blink`
or:

`# ./openocd_flash.sh target/thumbv6m-none-eabi/release/stm32_blink`

To start gdb for debugging:

`# ./openocd_session.sh`

And in second terminal:

`# ./gdb_session.sh target/thumbv6m-none-eabi/debug/stm32_blink`
