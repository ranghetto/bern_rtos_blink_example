#![no_std]
#![no_main]

#![allow(unused)]

use panic_halt as _;
//use panic_abort as _; // requires nightly
//use panic_itm as _; // logs messages over ITM; requires ITM support
//use panic_semihosting as _; // logs messages to the host stderr; requires a debugger

use cortex_m_rt::entry;
use cortex_m_semihosting::hprintln;
use dht11::Dht11;
use stm32f4xx_hal as f4;
use stm32f4xx_hal::gpio::{GpioExt, PA5, PinState};
use stm32f4xx_hal::hal::delay::DelayNs;
use stm32f4xx_hal::prelude::{_fugit_RateExtU32, _stm32f4xx_hal_spi_SpiExt};
use stm32f4xx_hal::rcc::RccExt;
use stm32f4xx_hal::serial::SerialExt;
use stm32f4xx_hal::time::U32Ext;
use stm32f4xx_hal::timer::{SysDelay, SysTimerExt};
use stm32f4xx_hal::uart::Config;

#[entry]
fn main() -> ! {
    let cp = cortex_m::Peripherals::take().expect("Core peripherals cannot be accessed");
    let dp = f4::pac::Peripherals::take().expect("Peripherals cannot be accessed");

    let rcc = dp.RCC.constrain();
    let clocks = rcc.cfgr.use_hse(8.MHz()).sysclk(100.MHz()).freeze();

    let mut delay = cp.SYST.delay(&clocks);

    let gpiod = dp.GPIOD.split();

    let mut green_led = gpiod.pd12.into_push_pull_output();

    loop {
        green_led.set_high();
        delay.delay_ms(1000_u32);
        green_led.set_low();
        delay.delay_ms(1000_u32);
    }
}
