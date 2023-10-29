// Attiny 85
#![no_std]
#![no_main]

use panic_halt as _;

#[arduino_hal::entry]
fn main() -> ! {
    //get peripherals
    let dp = arduino_hal::Peripherals::take().unwrap();
    //get pin 1
    let led = arduino_hal::pins!(dp).d1;
    //set pin to output
    let mut led = led.into_output();

    loop {
        //togle led
        led.toggle();
        //sleep (with hal)
        arduino_hal::delay_ms(1000);
    }
}