//! ATmega328P
use core::ptr::write_volatile;

use enumflags2::{bitflags, BitFlags};

// pub const PINB: *mut u8 = 0x23 as *mut u8;
// pub const DDRB: *mut u8 = 0x24 as *mut u8;
// pub const PORTB: *mut u8 = 0x25 as *mut u8;
//
// pub const PINC: *mut u8 = 0x26 as *mut u8;
// pub const DDRC: *mut u8 = 0x27 as *mut u8;
// pub const PORTC: *mut u8 = 0x28 as *mut u8;
//
// pub const PIND: *mut u8 = 0x29 as *mut u8;
// pub const DDRD: *mut u8 = 0x2A as *mut u8;
// pub const PORTD: *mut u8 = 0x2B as *mut u8;

#[bitflags]
#[repr(u8)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum PortB {
    PB0 = 1 << 0,
    PB1 = 1 << 1,
    PB2 = 1 << 2,
    PB3 = 1 << 3,
    PB4 = 1 << 4,
    PB5 = 1 << 5,
    PB6 = 1 << 6,
    PB7 = 1 << 7,
}

impl PortB {
    const PINB: *mut u8 = 0x23 as *mut u8;
    const DDRB: *mut u8 = 0x24 as *mut u8;
    const PORTB: *mut u8 = 0x25 as *mut u8;

    /// ```
    /// PortB::write_port(PortB::PB0 | PortB::PB1);
    /// ```
    pub fn write_port(bits: BitFlags<Self, u8>) {
        unsafe {
            write_volatile(Self::PORTB, bits.bits());
        }
    }
}

// cargo expand --target x86_64-unknown-linux-gnu
