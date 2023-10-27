#![no_std]
#![no_main]

use panic_halt as _;

#[arduino_hal::entry]
fn main() -> ! {
    //get peripherals
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);

    //get pin 1 and set to output
    let mut led = pins.d1.into_output();

    loop {
        //togle led
        led.toggle();
        //sleep (with hal)
        arduino_hal::delay_ms(1000);
    }
}
