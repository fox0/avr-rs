// todo instrint avr
use core::ptr::{read_volatile, write_volatile};

// todo if cfg
mod atmega328p;
pub use atmega328p::RegisterAddress;

#[allow(dead_code)]
impl RegisterAddress {
    /// ```
    /// RegisterAddress::PORTB.write(255);
    /// ```
    #[inline(always)]
    pub fn write(self, value: u8) {
        let adr = self as u8 as *mut u8;
        // SAFETY: adr must be valid for writes
        unsafe {
            write_volatile(adr, value);
        }
    }

    /// ```
    /// let value = RegisterAddress::PINB.read();
    /// ```
    #[must_use]
    #[inline(always)]
    pub fn read(self) -> u8 {
        let adr = self as u8 as *mut u8;
        // SAFETY: adr must be valid for reads
        unsafe { read_volatile(adr) }
    }
}
