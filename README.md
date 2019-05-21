A simple 'blinking LED' program for the STM32F3 Discovery board which the 'Embedded Rust' ebook targets.

It looks like the ebook is a work-in-progress, since the code samples stop soon after the 'Hello World' section. But that's okay; it's off to a great start, and it provides enough information to get started with the core peripherals.

This project uses the ['cortex-m-quickstart' template](https://github.com/rust-embedded/cortex-m-quickstart) and the [stm32f30x crate](https://crates.io/crates/stm32f30x). Unlike the `stm32f30x-hal` crate, the `stm32f30x` doesn't have helper methods for configuring peripherals and clock settings. It's a very thin wrapper around the peripheral registers which started life as auto-generated output from the [svd2rust crate](https://crates.io/crates/svd2rust)
