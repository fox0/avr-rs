#![no_std]
#![cfg_attr(not(test), no_main)]
#![feature(lang_items)]
#![feature(abi_avr_interrupt)]
mod arch;

use crate::arch::RegisterAddress;

// extern "C" {
//     fn __bad_interrupt();
// }

#[cfg(not(test))]
#[lang = "eh_personality"]
#[no_mangle]
pub extern "C" fn rust_eh_personality() {}

#[cfg(not(test))]
#[panic_handler]
unsafe fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

#[cfg(not(test))]
#[no_mangle]
pub extern "avr-interrupt" fn __vector_3() {
    // let prev_value = read_volatile(PORTB);
    // write_volatile(PORTB, prev_value);
}

#[cfg(not(test))]
#[no_mangle]
pub extern "C" fn main() -> ! {
    // PortB::write_port(PortB::PB0 | PortB::PB1);
    RegisterAddress::PORTB.write(255);

    // Set the upper four physical pins on PORT B to inputs, the lower four to outputs.
    // The AVR interprets '1' in the data direction register as 'output', '0' input
    // for the corresponding pin.
    // core::ptr::write_volatile(DDRB, core::ptr::read_volatile(DDRB) | 0b00001111);

    // Write half of the output pins as high, the other half low.
    loop {
        let _x = 42u8;
    }
}
