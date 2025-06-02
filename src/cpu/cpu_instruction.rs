use std::collections::HashMap;
use crate::constants::flags::C_FLAG;
use crate::cpu::cpu::CPU;
use crate::cpu::register::Register;

pub struct CPUInstruction {
    pub mnemonic: &'static str,
    pub cycles: u8,
    pub bytes: u8,
    pub execute: fn(&mut CPU)
}

impl CPUInstruction {
    
    
    /// Adds a given 8-bit value (`r8`) and the carry flag to the accumulator register (`A`).
    ///
    /// # Parameters
    /// - `registers`: A mutable reference to the `Register` structure containing the CPU registers.
    /// - `r8`: The 8-bit value to add to the `A` register.
    ///
    /// This function performs an addition between the accumulator (`A`) register, the provided
    /// 8-bit value (`r8`), and the carry flag, while handling 8-bit arithmetic overflow using
    /// `wrapping_add`. It also updates the flags in the `F` register based on the result of the
    /// addition:
    ///
    /// Once the computation is complete, the result is stored back in the `A` register.
    ///
    /// # Examples
    /// ```rust
    /// // Assuming a properly initialized Register with A = 0x15 and C_FLAG = set.
    /// let mut registers = Register::new();
    /// registers.set_a(0x15);
    /// registers.get_f_mut().set_flag(C_FLAG, true);
    ///
    /// add_a_r8(&mut registers, 0x20);
    /// ```
    pub fn adc_a_r8(registers: &mut Register, r8: u8) {
        let carry:u8 = if registers.get_f_mut().get_flag(C_FLAG) { 1 } else { 0 };
        let a: u8 = registers.get_a();

        //wrapping_add avoids overflow in fixed size (Ex 255 + 1 generate overflows in u8)
        let r = a.wrapping_add(r8).wrapping_add(carry);

        //set flags produced by operation
        let c: bool = (a as u16 + r8 as u16 + carry as u16) > 0xFF;
        let h: bool = ((a & 0xF) + (r8 & 0xF) + carry) > 0xF;
        registers.get_f_mut().set_flags(c,false,h,r == 0);
        //setting value in a = a + r8
        registers.set_a(r);
    }

    /// Performs an addition operation involving the accumulator register (`A`),
    /// a value in memory pointed to by the HL register pair, and (optionally) the carry flag.
    /// HL is a 16-bit address memory to read a value in RAM
    /// The result is stored back in the accumulator register (`A`) and updates flags accordingly.
    ///
    /// # Parameters
    /// - `registers`: A mutable reference to the `Register` object containing the CPU registers.
    ///
    /// # TODO
    /// - Implement the memory read operation to fetch the value at the address pointed to by HL.
    ///
    /// ```
    pub fn add_a_hl(registers: &mut Register){
        let a: u8 = registers.get_a();
        let hl: u16 = registers.get_hl();
        //TODO: Read value from RAM
        let value: u8 = 0x00;
        let carry: u8 = if registers.get_f_mut().get_flag(C_FLAG) {1} else {0};
        let r: u8 = a.wrapping_add(value).wrapping_add(carry);
        let c: bool = (a as u16 + value as u16 + carry as u16) > 0xFF;
        let h: bool = ((a & 0xF) + (value & 0xF) + carry) > 0xF;
        registers.get_f_mut().set_flags(c,false,h,r == 0);
        registers.set_a(r);
    }

    //TODO: Need Memory Map Struct
    //pub fn add_a_n8(registers: &mut Register, address: u8){}


    /// Adds an 8-bit value (`r8`) to the value stored in register A within the given
    /// `registers` struct.
    /// # Arguments
    /// * `registers` - A mutable reference to the `Register` structure containing the CPU registers.
    /// * `r8` - An 8-bit unsigned integer value to be added to the current value in register A.
    pub fn add_a_r8(registers: &mut Register, r8: u8){
        let a: u8 = registers.get_a();
        let r: u8 = a.wrapping_add(r8);
        let c: bool = ((a as u16) + (r8 as u16)) > 0xFF;
        let h: bool = ((a & 0xF) + (r8 & 0xF)) > 0xF;
        registers.get_f_mut().set_flags(c,false,h,r == 0);
        registers.set_a(r);
    }
    
}







