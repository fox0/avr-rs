#![no_std]
#![no_main]
#![feature(lang_items)]
#![feature(abi_avr_interrupt)]

use core::ptr::{read_volatile, write_volatile};

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

/// The data direction register for PORT B, which is mapped to 0x24 in memory on the atmega328.
const DDRB: *mut u8 = 0x24 as *mut u8;
/// The pin status register for PORT B, which is mapped to 0x25 in memory on the atmega328.
const PORTB: *mut u8 = 0x25 as *mut u8;

#[no_mangle]
pub unsafe extern "avr-interrupt" fn __vector_3() {
    let prev_value = read_volatile(PORTB);
    write_volatile(PORTB, prev_value);
}

#[no_mangle]
pub unsafe extern fn main() -> ! {
    // unsafe {
    // Set the upper four physical pins on PORT B to inputs, the lower four to outputs.
    // The AVR interprets '1' in the data direction register as 'output', '0' input
    // for the corresponding pin.
    // core::ptr::write_volatile(DDRB, core::ptr::read_volatile(DDRB) | 0b00001111);

    // Write half of the output pins as high, the other half low.
    write_volatile(PORTB, 0b00001010);
    // }
    loop {}
}
