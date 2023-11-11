// Attiny 84
#![no_std]
#![no_main]

use embedded_hal::prelude::_embedded_hal_blocking_delay_DelayMs;
use panic_halt as _;
use avr_device_macros;
use attiny_hal as hal;
use avr_hal_generic;


#[avr_device_macros::entry]
fn main() -> ! {
    //get peripherals
    let dp = hal::Peripherals::take().unwrap();
    //get pin 13
    let led = hal::pins!(dp).pa0;

    //set pin to output
    let mut led = led.into_output();
    //setup delay
    let mut delay = hal::delay::Delay::<hal::clock::MHz1>::new();

    loop {
        //togle led
        led.toggle();
        //sleep (with hal)
        delay.delay_ms(1000u16);
    }
}