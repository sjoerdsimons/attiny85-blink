#![no_std]
#![no_main]

use embedded_hal::delay::DelayNs;
use panic_halt as _;

#[attiny_hal::entry]
fn main() -> ! {
    let dp = attiny_hal::Peripherals::take().unwrap();
    let pins = attiny_hal::pins!(dp);

    let mut delay = attiny_hal::delay::Delay::<attiny_hal::clock::MHz1>::new();
    let mut led = pins.pb3.into_output();

    loop {
        led.toggle();
        delay.delay_ms(500);
    }
}
