#![deny(unsafe_code)]
#![allow(clippy::empty_loop)]
#![no_std]
#![no_main]

// set the panic handler
extern crate panic_rtt_target;
use rtt_target::{rprintln, rtt_init_print};

use cortex_m_rt::entry;


use nb::block;

use stm32f4xx_hal::{
    pac::{Peripherals},
    prelude::*,
    timer::Timer,
};

// Display interface pins:
//
//   PA4 - CS
//   PA5 - SCLK
//   PA7 - MOSI


#[entry]
fn main() -> ! {
  rtt_init_print!();

  rprintln!("Initializing...");

  let cp = cortex_m::Peripherals::take().unwrap();
  let dp = Peripherals::take().unwrap();

  let rcc = dp.RCC.constrain();

  let clocks = rcc.cfgr.use_hse(25.MHz()).sysclk(32.MHz()).freeze();

  let gpioc = dp.GPIOC.split();

  let mut led = gpioc.pc13.into_push_pull_output();

  // Configure the syst timer to trigger periodic updates
  let mut timer = Timer::syst(cp.SYST, &clocks).counter_hz();
  timer.start(5.Hz()).unwrap();

  rprintln!("Looping...");

  loop {
    block!(timer.wait()).unwrap();
    led.set_high();
    block!(timer.wait()).unwrap();
    led.set_low();
  }
}


