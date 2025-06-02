use crate::error::memory_error::MemoryError;
use crate::memory_bus::bus::BUS;

pub struct WRAM  {
    r1: [u8; 0x1000],
    r2: [u8; 0x1000],
}

impl WRAM {
    pub fn new() -> Self {
        WRAM {
            r1: [0; 0x1000],
            r2: [0; 0x1000],
        }
    }
}

impl BUS for WRAM {
    fn read(&self, address: u16) -> Result<u8, MemoryError> {
        dbg!("Reading from WRAM");
        match address {
            0xC000..=0xCFFF => Ok(self.r1[(address - 0xC000) as usize]),
            0xD000..=0xDFFF => Ok(self.r2[(address - 0xD000) as usize]),
            _ => Err(MemoryError::InvalidAddress(address)),
        }
    }

    fn write(&mut self, address: u16, value: u8) -> Result<(), MemoryError>{
        match address {
            0xC000..=0xCFFF => {
                self.r1[(address - 0xC000) as usize] = value;
                Ok(())
            }
            0xD000..=0xDFFF => {
                self.r2[(address - 0xD000) as usize] = value;
                Ok(())
            }
            _ => Err(MemoryError::InvalidAddress(address)),
        }
    }
}