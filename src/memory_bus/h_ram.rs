use crate::error::memory_error::MemoryError;
use crate::memory_bus::bus::BUS;

pub struct HRAM  {
    r: [u8; 127],
}

impl HRAM {
    pub fn new() -> Self {
        HRAM {
            r: [0; 127],
        }
    }
}

impl BUS for HRAM {
    fn read(&self, address: u16) -> Result<u8, MemoryError> {
        dbg!("Reading from HRAM");
        match address {
            0xFF80..=0xFFFE => Ok(self.r[(address - 0xC000) as usize]),
            _ => Err(MemoryError::InvalidAddress(address)),
        }
    }

    fn write(&mut self, address: u16, value: u8) -> Result<(), MemoryError>{
        match address {
            0xFF80..=0xFFFE => {
                self.r[(address - 0xC000) as usize] = value;
                Ok(())
            }
            _ => Err(MemoryError::InvalidAddress(address)),
        }
    }
}