//! ATmega328P
use enumflags2::bitflags;

#[allow(unused)]
#[allow(clippy::upper_case_acronyms)]
#[repr(u8)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum RegisterAddress {
    // I/O Port
    /// Reads state of Input Pins
    /// The PINx register simply reads the value from the pins. This value, as obvious, is digital.
    PINB = 0x23,
    /// Data Direction Register
    /// The DDRx register is responsible for initialising the pins for use either as inputs or outputs.
    /// A 1 bit signifies a pin initialised as output. A 0 bit signifies a pin initialised as input.
    DDRB,
    /// Defines state of Output Pins
    /// The PORTx register determines whether the output state of a pin is HIGH or LOW.
    /// A 1 bit signifies HIGH and 0 signifies LOW.
    /// DDRx takes precedence over PORTx. A pin previously defined as input won’t be affected by PORTx.
    /// The pin must be initialised as an output by DDRx first to manipulate it through PORTx.
    PORTB,
    PINC,
    DDRC,
    PORTC,
    PIND,
    DDRD,
    PORTD,

    // /// Timer Interrupt Flag Register
    // TIFR0 = 0x35,
    /// General Timer/Counter Control Register
    GTCCR = 0x43,

    // EEPROM

    // Watchdog Timer
    /// Watchdog Timer Control Register
    WDTCSR = 0x60,

    // CPU Registers

    // Serial Peripheral Interface

    // Timer/Counter, 8-bit 0

    // External Interrupts

    // Analog Comparator

    // Analog-to-Digital Converter
    // <register caption="The ADC multiplexer Selection Register" name="ADMUX" offset="0x7C" size="1">
    // <register caption="ADC Data Register  Bytes" name="ADC" offset="0x78" size="2" mask="0xFFFF"/>
    // <register caption="The ADC Control and Status register A" name="ADCSRA" offset="0x7A" size="1" ocd-rw="R">
    // <register caption="The ADC Control and Status register B" name="ADCSRB" offset="0x7B" size="1">
    // <register caption="Digital Input Disable Register" name="DIDR0" offset="0x7E" size="1">

    // Timer/Counter, 16-bit
    /// Timer/Counter1 Interrupt Flag register
    TIFR1 = 0x36,
    /// Timer/Counter1 Interrupt Mask Register
    TIMSK1 = 0x6F,
    /// Timer/Counter1 Control Register A
    TCCR1A = 0x80,
    /// Timer/Counter1 Control Register B
    TCCR1B,
    /// Timer/Counter1 Control Register C
    TCCR1C,
    /// Timer/Counter1  Bytes
    TCNT1 = 0x84, // todo H&L
    /// Timer/Counter1 Input Capture Register  Bytes
    ICR1 = 0x86, // todo H&L
    /// Timer/Counter1 Output Compare Register  Bytes
    OCR1A = 0x88, // todo H&L
    /// Timer/Counter1 Output Compare Register  Bytes
    OCR1B = 0x8A, // todo H&L

    // Timer/Counter, 8-bit Async
    /// Timer/Counter2 Interrupt Flag Register
    TIFR2 = 0x37,
    /// Timer/Counter2 Interrupt Mask register
    TIMSK2 = 0x70,
    /// Timer/Counter2 Control Register A
    TCCR2A = 0xB0,
    /// Timer/Counter2 Control Register B
    TCCR2B,
    /// Timer/Counter2
    TCNT2,
    /// Timer/Counter2 Output Compare Register A
    OCR2A,
    /// Timer/Counter2 Output Compare Register B
    OCR2B,
    /// Asynchronous Status Register
    ASSR = 0xB6,

    // Two Wire Serial Interface
    /// TWI Bit Rate register
    TWBR = 0xB8,
    /// TWI Status Register
    TWSR,
    /// TWI (Slave) Address register
    TWAR,
    /// TWI Data register
    TWDR,
    /// TWI Control Register
    TWCR,
    /// TWI (Slave) Address Mask Register
    TWAMR,

    // USART
    /// USART Control and Status Register A
    UCSR0A = 0xC0,
    /// USART Control and Status Register B
    UCSR0B,
    /// USART Control and Status Register C
    UCSR0C = 0xC2,
    // USART Baud Rate Register Bytes
    UBRR0 = 0xC4, // todo H&L
    /// USART I/O Data Register
    UDR0 = 0xC6,
}

#[bitflags]
#[repr(u8)]
#[derive(Debug, Copy, Clone, PartialEq)]
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

// impl PortB {
//     const PINB: *mut u8 = 0x23 as *mut u8;
//     const DDRB: *mut u8 = 0x24 as *mut u8;
//     const PORTB: *mut u8 = 0x25 as *mut u8;
//
//     /// ```
//     /// PortB::write_port(PortB::PB0 | PortB::PB1);
//     /// ```
//     pub fn write_port(bits: BitFlags<Self, u8>) {
//         unsafe {
//             write_volatile(Self::PORTB, bits.bits());
//         }
//     }
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_register_address() {
        assert_eq!(RegisterAddress::DDRD as u8, 0x2Au8);
    }
}

// cargo expand --target x86_64-unknown-linux-gnu
