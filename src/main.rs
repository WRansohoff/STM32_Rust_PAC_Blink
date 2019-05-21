#![no_std]
#![no_main]

// Halt when the program panics.
extern crate panic_halt;

// Includes.
use cortex_m::peripheral::syst::SystClkSource;
use cortex_m_rt::entry;
use stm32f30x;

#[entry]
fn main() -> ! {
  // Set up SysTick interrupt.
  let cm_p = cortex_m::Peripherals::take().unwrap();
  let mut syst = cm_p.SYST;
  syst.set_clock_source( SystClkSource::Core );
  syst.set_reload( 8_000_000 );
  syst.enable_counter();

  // Set up GPIO pins E8 and E9 as push-pull output.
  let p = stm32f30x::Peripherals::take().unwrap();
  let rcc = p.RCC;
  rcc.ahbenr.write( |w| w.iopeen().set_bit() );
  let gpioe = p.GPIOE;
  gpioe.moder.write( |w| w.moder8().output().moder9().output() );
  gpioe.otyper.write( |w| w.ot8().clear_bit().ot9().clear_bit() );

  loop {
    // Toggle the bits every SysTick tick.
    gpioe.odr.write( |w| w.odr8().set_bit().odr9().clear_bit() );
    while !syst.has_wrapped() {};
    gpioe.odr.write( |w| w.odr8().clear_bit().odr9().set_bit() );
    while !syst.has_wrapped() {};
  }
}
