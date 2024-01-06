#![no_std]
#![no_main]
// #![feature(abi_avr_interrupt)]
// #![feature(asm_experimental_arch)]
// #![feature(asm_const)]

use panic_halt as _;

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);

    let mut led_pin = pins.led_tx.into_output();

    arduino_hal::delay_ms(500);
    led_pin.set_high();
    arduino_hal::delay_ms(500);
    led_pin.set_low();


    loop {}
}
