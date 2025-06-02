use crate::error::memory_error::MemoryError;

pub trait BUS {
    fn read(&self, address: u16) -> Result<u8, MemoryError>;
    fn write(&mut self, address: u16, value: u8) -> Result<(), MemoryError>;
}