#![no_std]
#![no_main]

use esp_idf_svc::hal::{gpio, peripherals, delay::Delay};

#[no_mangle]
fn main() {
    // It is necessary to call this function once. Otherwise some patches to the runtime
    // implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
    esp_idf_svc::sys::link_patches();

    //get peripherals
    let peripherals = peripherals::Peripherals::take().unwrap();
    //get pin 23
    let led = peripherals.pins.gpio5;
    //set pin to output
    let mut led = gpio::PinDriver::output(led).unwrap();

    loop {
        //toggle led
        led.toggle().unwrap();
        //sleep (with core)
        Delay::new_default().delay_ms(1000);
    }

}