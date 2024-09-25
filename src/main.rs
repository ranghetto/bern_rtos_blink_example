#![no_std]
#![no_main]

#![allow(unused)]

use core::pin::Pin;
use bern_kernel::exec::interrupt::{Context, InterruptHandler, InterruptStack};
use panic_halt as _;

use cortex_m_rt::entry;
use stm32f4xx_hal::gpio::{Edge, ErasedPin, ExtiPin, GpioExt, Input, Output};
use stm32f4xx_hal as f4;

use bern_kernel::exec::process::Process;
use bern_kernel::exec::runnable::Priority;
use bern_kernel::exec::thread::Thread;
use bern_kernel::stack::Stack;
use bern_kernel::sleep;
use bern_kernel::units::frequency::ExtMilliHertz;
use rtt_target::{rtt_init_print, rprintln};
use stm32f4xx_hal::hal::digital::InputPin;
use stm32f4xx_hal::pac::{Peripherals, EXTI, GPIOA, GPIOD};
use stm32f4xx_hal::rcc::{Clocks, Rcc, RccExt};
use stm32f4xx_hal::syscfg::{SysCfg, SysCfgExt};
use stm32f4xx_hal::time::Hertz;

static PROC: &Process = bern_kernel::new_process!(my_process1, 8192);
fn setup_clocks(rcc: Rcc) -> Clocks {
    rcc.cfgr.sysclk(Hertz::MHz(100)).freeze()
}
struct UsedGPIOA {
    user_btn: f4::gpio::Pin<'A', 0>
}
struct UsedGPIOD {
    green_led: ErasedPin<Output>
}
fn setup_gpio_a(gpio: GPIOA, sys_cfg: &mut SysCfg, exti: &mut EXTI) -> UsedGPIOA{

    let gpio = gpio.split();

    /* USER BUTTON 1: setup interrupt trigger */
    let mut user_btn = gpio.pa0.into_input();
    user_btn.make_interrupt_source(sys_cfg);
    user_btn.trigger_on_edge(exti, Edge::Rising);
    user_btn.enable_interrupt(exti);
    unsafe {
        cortex_m::peripheral::NVIC::unmask(user_btn.interrupt());
    }

    UsedGPIOA {
        user_btn
    }
}
fn setup_gpio_d(gpio: GPIOD) -> UsedGPIOD {

    let gpio = gpio.split();

    /* GREEN LED */
    let mut green_led = gpio.pd12.into_push_pull_output().erase();

    UsedGPIOD {
        green_led
    }
}

fn blink_led_task(mut green_led: ErasedPin<Output>) -> impl FnMut() {
    move || {
        loop {
            green_led.set_high();
            sleep(1000);
            green_led.set_low();
            sleep(1000);
        }
    }
}
fn button_pressed_task(mut user_btn: f4::gpio::Pin<'A', 0>) -> impl FnMut(&Context) {
    move |_c| {
        user_btn.clear_interrupt_pending_bit();
        rprintln!("User button pressed.");
    }
}

#[entry]
fn main() -> ! {
    rtt_init_print!();

    let dp = Peripherals::take().expect("Peripherals cannot be accessed");

    let rcc = dp.RCC.constrain();
    let mut syscfg = dp.SYSCFG.constrain();
    let mut exti = dp.EXTI;
    let gpio_a = dp.GPIOA;
    let gpio_d = dp.GPIOD;

    let clocks = setup_clocks(rcc);
    let gpios_a = setup_gpio_a(gpio_a, &mut syscfg, &mut exti);
    let gpios_d = setup_gpio_d(gpio_d);

    let mut user_btn = gpios_a.user_btn;
    let mut green_led = gpios_d.green_led;

    bern_kernel::kernel::init();
    bern_kernel::time::set_tick_frequency(ExtMilliHertz::kHz(1), ExtMilliHertz::MHz(100));

    PROC.init(move |c| {
        Thread::new(c)
            .priority(Priority::new(0))
            .stack(Stack::try_new_in(c, 1024).unwrap())
            .spawn(blink_led_task(green_led));
        InterruptHandler::new(c)
            .stack(InterruptStack::Kernel)
            .connect_interrupt(user_btn.interrupt() as u16)
            .handler(button_pressed_task(user_btn));
    }).unwrap();

    bern_kernel::start()
}
