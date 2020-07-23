#![no_main]
#![no_std]

extern crate panic_semihosting;

use cortex_m_rt::entry;
use stm32l0xx_hal::{pac, prelude::*, rcc::Config, serial};
use core::fmt::Write;
use shared_bus::CortexMBusManager;

#[cfg(debug_assertions)]
use cortex_m_semihosting::hprintln;

mod bmp280;

#[entry]
fn main() -> ! {
    let dp = pac::Peripherals::take().unwrap();
    let cp = cortex_m::Peripherals::take().unwrap();

    // Configure the clock.
    let mut rcc = dp.RCC.freeze(Config::hsi16());

    // Acquire the GPIOA peripheral. This also enables the clock for GPIOA in
    // the RCC register.
    let gpioa = dp.GPIOA.split(&mut rcc);
    let gpiob = dp.GPIOB.split(&mut rcc);

    // Configure PA5 as output.
    let mut led = gpioa.pa5.into_push_pull_output();

    // Get the delay provider.
    let mut delay = cp.SYST.delay(rcc.clocks);

    let sda = gpiob.pb9.into_open_drain_output();
    let scl = gpiob.pb8.into_open_drain_output();
    let i2c = dp.I2C1.i2c(sda, scl, 100.khz(), &mut rcc);
    let i2c_bus = CortexMBusManager::new(i2c);

    let tx_pin = gpioa.pa2;
    let rx_pin = gpioa.pa3;
    let serial = dp
        .USART2
        .usart((tx_pin, rx_pin), serial::Config::default(), &mut rcc)
        .unwrap();
    let (mut tx, _) = serial.split();

    let mut bmp280 = bmp280::Bmp280::new(i2c_bus.acquire(), true).unwrap();

    loop {
        led.set_high().unwrap();
        delay.delay_ms(250_u16);

        led.set_low().unwrap();
        delay.delay_ms(250_u16);

        #[cfg(debug_assertions)]
        hprintln!("{}", bmp280.get_pressure().unwrap()).unwrap();
        
        writeln!(tx, "{}", bmp280.get_pressure().unwrap()).unwrap();
    }

}