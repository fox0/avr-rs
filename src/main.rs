#![no_std]
#![no_main]
#![feature(lang_items)]
#![feature(abi_avr_interrupt)]

mod arch;

use crate::arch::atmega328p::PortB;

// use core::ptr::{read_volatile, write_volatile};

extern "C" {
    fn __bad_interrupt();
}

#[lang = "eh_personality"]
#[no_mangle]
pub unsafe extern "C" fn rust_eh_personality() -> () {}

#[panic_handler]
unsafe fn panic(_info: &core::panic::PanicInfo) -> ! {
    __bad_interrupt();
    unreachable!()
}

#[no_mangle]
pub extern "avr-interrupt" fn __vector_3() {
    // let prev_value = read_volatile(PORTB);
    // write_volatile(PORTB, prev_value);
}

#[no_mangle]
pub extern "C" fn main() -> ! {
    PortB::write_port(PortB::PB0 | PortB::PB1);

    // Set the upper four physical pins on PORT B to inputs, the lower four to outputs.
    // The AVR interprets '1' in the data direction register as 'output', '0' input
    // for the corresponding pin.
    // core::ptr::write_volatile(DDRB, core::ptr::read_volatile(DDRB) | 0b00001111);

    // Write half of the output pins as high, the other half low.
    loop {}
}
