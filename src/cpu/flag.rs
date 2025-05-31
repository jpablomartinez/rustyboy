use crate::constants::flags::{C_FLAG, H_FLAG, N_FLAG, Z_FLAG};

pub struct Flag{
    bit: u8,
}

impl Flag {
    pub fn new() -> Flag {
        Flag {
            bit: 0x00,
        }
    }
    
    /// Retrieves the state of a specific flag bit from the `bit` field.
    ///
    /// # Parameters
    /// - `bit`: A `u8` value representing the flag bit to be checked. 
    ///   This must be one of the predefined constants: `C_FLAG`, `N_FLAG`, `H_FLAG`, or `Z_FLAG`.
    ///
    /// # Returns
    /// - `true` if the specified flag bit is set (`1`).
    /// - `false` if the specified flag bit is not set (`0`).
    ///
    /// # Panics
    /// The function will panic in debug builds if the `bit` parameter does not match 
    /// one of the allowed flag constants. The panic message will include the invalid bit value.
    ///
    pub fn get_flag(&self, bit: u8) -> bool {
        debug_assert!(bit == C_FLAG || bit == N_FLAG || bit == H_FLAG || bit == Z_FLAG,
                      "Invalid flag bit: {:#08b}", bit);
        self.bit & bit != 0
    }
    
    /// Changes the state of a specific flag bit in the bit field.
    ///
    /// # Parameters
    /// - `set`: A boolean value indicating whether the flag should be set (`true`) or cleared (`false`).
    /// - `bit`: The specific bit to modify, which must correspond to one of the valid flag constants 
    ///   (`C_FLAG`, `N_FLAG`, `H_FLAG`, or `Z_FLAG`).
    ///
    /// # Panics
    /// This method will panic if the provided `bit` value is not equal to one of the valid flag constants:
    /// - `C_FLAG`: Carry flag
    /// - `N_FLAG`: Negative flag
    /// - `H_FLAG`: Half-carry flag
    /// - `Z_FLAG`: Zero flag
    /// The panic message will include the invalid `bit` value in binary format.
    pub fn set_flag(&mut self, set:bool, bit: u8){
        debug_assert!(bit == C_FLAG || bit == N_FLAG || bit == H_FLAG || bit == Z_FLAG,
                      "Invalid flag bit: {:#08b}", bit);
        if set {
            self.bit |= bit; 
        } else {
            self.bit &= !bit;
        }
    }
    
    /// Sets the specified flags in the current object's flag register.
    ///
    /// This function updates the flag register by setting or clearing the given
    /// flags based on the passed boolean values. Each flag corresponds to a specific
    /// condition, and its state is toggled accordingly.
    ///
    /// # Parameters
    ///
    /// - `c` (`bool`): The condition code for the carry flag (C_FLAG).
    /// - `n` (`bool`): The condition code for the negative flag (N_FLAG).
    /// - `h` (`bool`): The condition code for the half-carry flag (H_FLAG).
    /// - `z` (`bool`): The condition code for the zero flag (Z_FLAG).
    pub fn set_flags(&mut self, c: bool, n: bool, h: bool, z: bool) {
        self.set_flag(c, C_FLAG);
        self.set_flag(n, N_FLAG);
        self.set_flag(h, H_FLAG);
        self.set_flag(z, Z_FLAG);
    }
    
}