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
    /// # Example
    /// ```
    /// // Assuming `self.bit` is set to 0b1010_0000
    /// assert!(get_flag(C_FLAG)); // Returns true if `C_FLAG` is set.
    /// assert!(!get_flag(H_FLAG)); // Returns false if `H_FLAG` is not set.
    /// ```
    ///
    /// # Notes
    /// - The function uses a bitwise `AND` operation to determine if the specified flag is active.
    /// - It is important to ensure the `bit` parameter aligns with the predefined constants to avoid unexpected behavior.
    ///
    /// # Debugging
    /// The `debug_assert!` macro is included to validate the input bit against valid options during debug builds, 
    /// helping developers catch invalid usage early.
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
    ///
    /// # Behavior
    /// - If `set` is `true`, the specified flag bit is set to `1`.
    /// - If `set` is `false`, the specified flag bit is cleared to `0`.
    ///
    /// # Example
    /// ```
    /// // Assuming `self.bit` starts as `0b0000_0000`
    /// object.set_flag(true, C_FLAG);  // Sets the carry flag
    /// assert_eq!(object.bit, C_FLAG);
    ///
    /// object.set_flag(false, C_FLAG); // Clears the carry flag
    /// assert_eq!(object.bit, 0);
    ///
    /// // Invalid bit input will panic
    /// // object.set_flag(true, 0b0000_1000); // This will cause a panic
    /// ```
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
    ///                 If `true`, the carry flag is set; if `false`, it is cleared.
    /// - `n` (`bool`): The condition code for the negative flag (N_FLAG). 
    ///                 If `true`, the negative flag is set; if `false`, it is cleared.
    /// - `h` (`bool`): The condition code for the half-carry flag (H_FLAG). 
    ///                 If `true`, the half-carry flag is set; if `false`, it is cleared.
    /// - `z` (`bool`): The condition code for the zero flag (Z_FLAG). 
    ///                 If `true`, the zero flag is set; if `false`, it is cleared.
    ///
    /// # Example
    ///
    /// ```rust
    /// let mut cpu = Cpu::new();
    /// cpu.set_flags(true, false, true, false);
    /// // This will set the carry, clear the negative, set the half-carry, and clear the zero flags.
    /// ```
    ///
    /// Note:
    /// - The specific implementation of `set_flag` is expected to handle the logic
    ///   of toggling individual flags within the flag register.
    /// - The constants `C_FLAG`, `N_FLAG`, `H_FLAG`, and `Z_FLAG` should correspond
    ///   to the respective bit positions or identifiers for these flags in the flag register.
    pub fn set_flags(&mut self, c: bool, n: bool, h: bool, z: bool) {
        self.set_flag(c, C_FLAG);
        self.set_flag(n, N_FLAG);
        self.set_flag(h, H_FLAG);
        self.set_flag(z, Z_FLAG);
    }
    
}