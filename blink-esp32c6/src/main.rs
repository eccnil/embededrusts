// ESP32c6
use std::time::Duration;
use esp_idf_svc::hal::{gpio, peripherals};

fn main() {
    // It is necessary to call this function once. Otherwise some patches to the runtime
    // implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
    esp_idf_svc::sys::link_patches();

    //get peripherals
    let peripherals = peripherals::Peripherals::take().unwrap();
    //get pin 23
    let led = peripherals.pins.gpio23;
    //set pin to output
    let mut led = gpio::PinDriver::output(led).unwrap();

    loop {
        //toggle led
        led.toggle().unwrap();
        //sleep (with std)
        std::thread::sleep(Duration::from_secs(1));
    }
}