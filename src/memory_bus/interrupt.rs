use crate::error::memory_error::MemoryError;
use crate::memory_bus::bus::BUS;

pub struct Interrupt{
    r: [u8; 0x01] // 128 bytes 
}

impl Interrupt{
    pub fn new() -> Interrupt{
        Interrupt{
            r: [0; 0x01]
        }
    }
}

impl BUS for Interrupt {
    fn read(&self, addr: u16) -> Result<u8, MemoryError> {
        dbg!("Reading from Interrupt");
        match addr {
            0xFFFF..=0xFFFF => Ok(self.r[(addr - 0xFFFF) as usize]),
            _ => Err(MemoryError::InvalidAddress(addr))
        }
    }

    fn write(&mut self, addr: u16, data: u8) -> Result<(), MemoryError> {
        match addr {
            0xFFFF..=0xFFFF => {
                self.r[(addr - 0xFFFF) as usize] = data;
                Ok(())
            },
            _ => Err(MemoryError::InvalidAddress(addr))
        }
    }
}