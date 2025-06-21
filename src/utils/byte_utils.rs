/// Extract the Least Significant Byte (LSB) from a 16-bit unsigned integer.
///
/// This function retrieves the least significant byte of the given 16-bit unsigned integer (`u16`) by
/// performing a bitwise AND operation with `0x00FF` and then casting the result to an 8-bit unsigned
/// integer (`u8`). The returned value represents the lower 8 bits of the input.
///
/// # Returns
///
/// Returns the least significant byte of the input as an 8-bit unsigned integer (`u8`).
pub fn get_lsb_u16(value: u16) -> u8{
    (value & 0x00FF) as u8
}

/// Extracts the most significant byte (MSB) from a 16-bit unsigned integer.
///
/// # Returns
///
/// * The most significant byte (MSB) of the input value as an 8-bit unsigned integer.
pub fn get_msb_u16(value: u16) -> u8{
    (value >> 8) as u8
}

/// Combines two 8-bit unsigned integers into a single 16-bit unsigned integer.
///
/// # Parameters
/// - `high`: The higher 8 bits of the result. This value will occupy the most significant byte.
/// - `low`: The lower 8 bits of the result. This value will occupy the least significant byte.
///
/// # Returns
/// A `u16` value where:
/// - The upper 8 bits come from the `high` parameter.
/// - The lower 8 bits come from the `low` parameter.
pub fn format_u16(high: u8, low: u8) -> u16 {
    ((high as u16) << 8) | (low as u16)
}


pub fn get_lsb_u8(value: u8) -> u8{
    value & 0x01
}