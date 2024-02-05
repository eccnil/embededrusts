#![no_std]
#![no_main]

use panic_halt as _;

#[arduino_hal::entry]
fn main() -> ! {
    //get Peripherals
    let dp = arduino_hal::Peripherals::take().unwrap();
    //get pin 12
    let led = arduino_hal::pins!(dp).d12;
    //set pin to output
    let mut led = led.into_output();

    loop {
        //toggle led
        led.toggle();
        //sleep (with hal)
        arduino_hal::delay_ms(1000);
    }
}
