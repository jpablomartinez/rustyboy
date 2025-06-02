use crate::error::memory_error::MemoryError;
use crate::memory_bus::bus::BUS;

pub struct OAM{
    r: [u8; 0xA0] // 128 bytes 
}

impl OAM{
    pub fn new() -> OAM{
        OAM{
            r: [0; 0xA0]
        }
    }
}

impl BUS for OAM {
    fn read(&self, addr: u16) -> Result<u8, MemoryError> {
        dbg!("Reading from OAM");
        match addr {
            0xFE00..=0xFE9F => Ok(self.r[(addr - 0xFE00) as usize]),
            _ => Err(MemoryError::InvalidAddress(addr))
        }
    }

    fn write(&mut self, addr: u16, data: u8) -> Result<(), MemoryError> {
        match addr {
            0xFE00..=0xFE9F => {
                self.r[(addr - 0xFE00) as usize] = data;
                Ok(())
            },
            _ => Err(MemoryError::InvalidAddress(addr))
        }
    }
}