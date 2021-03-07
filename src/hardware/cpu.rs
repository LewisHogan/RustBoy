// Information mostly sourced from https://hacktix.github.io/GBEDG/cpu/

// While in development, we allow for dead_code, this will need to be removed once the rest of the system has been setup.

/// The data registers are all 8-bit, although they can be combined into 16-bit register pairs for certain instructions.
#[allow(unused_variables, dead_code)]
pub struct Registers {
    /// The accumulator regisiter (a) is a data register that is commonly used as the target register for various instructions.
    a: u8,
    /// The flags register (f) stores status bits which are set according to the folowing conditions:
    /// - Bit 7 is the zero flag
    /// - Bit 6 is the addition/subtraction flag
    /// - Bit 5 is the half carry flag
    /// - Bit 4 is the carry flag
    /// - Bits 3-0 are unused and should always be set to 0, regardless of the instruction
    f: u8,
    b: u8,
    c: u8,
    d: u8,
    e: u8,
    sp: u16,
    pc: u16,
    // TODO: IME (Interrupt Master Enable), since this is apparently memory mapped I think it's best to hold off until that system is implemented
}

#[allow(dead_code)]
pub enum Flag {
    Zero,
    Addition,
    HalfCarry,
    Carry,
}

#[allow(dead_code)]
impl Registers {
    pub fn new() -> Self {
        // TODO: Establish default values
        Registers {
            f: 0,
            a: 0,
            b: 0,
            c: 0,
            d: 0,
            e: 0,
            sp: 0,
            pc: 0,
        }
    }

    fn set_flag(&mut self, flag: Flag, is_on: bool) {
        let bitmask = match flag {
            Flag::Zero => 1 << 7,
            Flag::Addition => 1 << 6,
            Flag::HalfCarry => 1 << 5,
            Flag::Carry => 1 << 4,
        };

        self.f = if is_on {
            self.f | bitmask
        } else {
            self.f & !bitmask
        };
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn set_flag_zero_on() {
        let mut registers = Registers::new();

        assert_eq!(registers.f, 0);
        registers.set_flag(Flag::Zero, true);
        assert_eq!(0b1000_0000, registers.f);
    }

    #[test]
    fn set_flag_zero_off() {
        let mut registers = Registers::new();
        
        registers.f = 0xF0;
        registers.set_flag(Flag::Zero, false);
        assert_eq!(0b0111_0000, registers.f);
    }

    #[test]
    fn set_flag_addition_on() {
        let mut registers = Registers::new();

        assert_eq!(registers.f, 0);
        registers.set_flag(Flag::Addition, true);
        assert_eq!(0b0100_0000, registers.f);
    }
    
    #[test]
    fn set_flag_addition_off() {
        let mut registers = Registers::new();
        
        registers.f = 0xF0;
        registers.set_flag(Flag::Addition, false);
        assert_eq!(0b1011_0000, registers.f);
    }

    #[test]
    fn set_flag_halfcarry_on() {
        let mut registers = Registers::new();

        assert_eq!(registers.f, 0);
        registers.set_flag(Flag::HalfCarry, true);
        assert_eq!(0b0010_0000, registers.f);
    }

    #[test]
    fn set_flag_halfcarry_off() {
        let mut registers = Registers::new();
        
        registers.f = 0xF0;
        registers.set_flag(Flag::HalfCarry, false);
        assert_eq!(0b1101_0000, registers.f);
    }

    #[test]
    fn set_flag_carry_on() {
        let mut registers = Registers::new();

        assert_eq!(registers.f, 0);
        registers.set_flag(Flag::Carry, true);
        assert_eq!(0b0001_0000, registers.f);
    }

    #[test]
    fn set_flag_carry_off() {
        let mut registers = Registers::new();
        
        registers.f = 0xF0;
        registers.set_flag(Flag::Carry, false);
        assert_eq!(0b1110_0000, registers.f);
    }

    #[test]
    fn set_multiple_flags_on() {
        let mut registers = Registers::new();

        assert_eq!(registers.f, 0);
        registers.set_flag(Flag::Addition, true);
        registers.set_flag(Flag::Carry, true);
        assert_eq!(0b0101_0000, registers.f);
    }
    
    #[test]
    fn set_multiple_flags_off() {
        let mut registers = Registers::new();

        registers.f = 0xF0;
        registers.set_flag(Flag::Addition, false);
        registers.set_flag(Flag::Carry, false);
        assert_eq!(0b1010_0000, registers.f);
    }
}
