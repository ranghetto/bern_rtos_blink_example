#![no_std]
#![no_main]

#![allow(unused)]

use panic_halt as _;

use cortex_m_rt::entry;
use stm32f4xx_hal::gpio::GpioExt;
use stm32f4xx_hal as f4;

use bern_kernel::exec::process::Process;
use bern_kernel::exec::runnable::Priority;
use bern_kernel::exec::thread::Thread;
use bern_kernel::stack::Stack;
use bern_kernel::sleep;
use bern_kernel::units::frequency::ExtMilliHertz;

use rtt_target::{rtt_init_print, rprintln};

static PROC: &Process = bern_kernel::new_process!(my_process1, 8192);

#[entry]
fn main() -> ! {
    rtt_init_print!();
    let dp = f4::pac::Peripherals::take().expect("Peripherals cannot be accessed");

    let gpio_d = dp.GPIOD.split();

    let mut green_led = gpio_d.pd12.into_push_pull_output();

    bern_kernel::kernel::init();
    bern_kernel::time::set_tick_frequency(1.kHz(), 100.MHz());

    PROC.init(move |c| {
        Thread::new(c)
            .priority(Priority::new(0))
            .stack(Stack::try_new_in(c, 1024).unwrap())
            .spawn(move || {
                loop {
                    green_led.set_high();
                    sleep(100);
                    green_led.set_low();
                    sleep(100);
                    rprintln!("Cycle done.");
                }
            });
    }).unwrap();

    bern_kernel::start()
}
