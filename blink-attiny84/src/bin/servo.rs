#![no_std]
#![no_main]

use embedded_hal::prelude::_embedded_hal_blocking_delay_DelayMs;
/// run with  `cargo build --bin servo && avrdude -c usbtiny -p attiny84 -Uflash:w:target/avr-attiny84/debug/servo.elf`
/// this will run on a attiny 84

use panic_halt as _;
use avr_device_macros;
use attiny_hal as hal;
use hal::simple_pwm::{Timer0Pwm, Prescaler};
use avr_hal_generic::{self, simple_pwm::IntoPwmPin};

#[avr_device_macros::entry]
fn main() -> ! {
    //get peripherals
    let dp = hal::Peripherals::take().unwrap();

    let timer0: Timer0Pwm = Timer0Pwm::new(dp.TC0, Prescaler::Prescale64);
    let pin = hal::pins!(dp).pa7;
    let pin = pin.into_output();
    let mut pin = pin.into_pwm(&timer0);

    let mut duty = 04;
    let max_duty = 40;
    let min_duty = 05;
    let mut increment = true;

    pin.set_duty(duty);
    pin.enable();

    let mut delay = hal::delay::Delay::<hal::clock::MHz1>::new();

    delay.delay_ms(1000u16);

    loop {
        if duty == max_duty {
            increment = false;
        } else if duty == min_duty {
            increment = true;
        }

        if increment {
            duty += 1;
        } else {
            duty -= 1;
        }

        pin.set_duty(duty);
        pin.enable();
        delay.delay_ms(200u16);
    }
}