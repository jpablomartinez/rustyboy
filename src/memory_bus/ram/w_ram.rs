use crate::error::memory_error::MemoryError;
use crate::memory_bus::ram::ram::RAM;

pub struct WRAM  {
    w1: [u8; 0x1000],
    w2: [u8; 0x1000],
}

impl WRAM {
    pub fn new() -> Self {
        WRAM {
            w1: [0; 0x1000],
            w2: [0; 0x1000],
        }
    }
}

impl RAM for WRAM {
    fn read(&self, address: u16) -> Result<u8, MemoryError> {
        match address {
            0xC000..=0xCFFF => Ok(self.w1[(address - 0xC000) as usize]),
            0xD000..=0xDFFF => Ok(self.w1[(address - 0xD000) as usize]),
            _ => Err(MemoryError::InvalidAddress(address)),
        }
    }

    fn write(&mut self, address: u16, value: u8) -> Result<(), MemoryError>{
        match address {
            0xC000..=0xCFFF => {
                self.w1[(address - 0xC000) as usize] = value;
                Ok(())
            }
            0xD000..=0xDFFF => {
                self.w1[(address - 0xD000) as usize] = value;
                Ok(())
            }
            _ => Err(MemoryError::InvalidAddress(address)),
        }
    }
}