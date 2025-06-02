use crate::error::memory_error::MemoryError;
use crate::memory_bus::bus::BUS;

pub struct VRAM{
    r: [u8; 0x2000] //8 kiB = 8192 bytes = 0x2000 
}

impl VRAM{
    pub fn new() -> VRAM{
        VRAM{
            r: [0; 0x2000]
        }
    }
}

impl BUS for VRAM {
    fn read(&self, addr: u16) -> Result<u8, MemoryError> {
        dbg!("Reading from VRAM");
        match addr { 
            0x8000..=0x9FFF => Ok(self.r[(addr - 0x8000) as usize]),
            _ => Err(MemoryError::InvalidAddress(addr))
        }
    }
    
    fn write(&mut self, addr: u16, data: u8) -> Result<(), MemoryError> {
        match addr { 
            0x8000..=0x9FFF => {
                self.r[(addr - 0x8000) as usize] = data;
                Ok(())
            },
            _ => Err(MemoryError::InvalidAddress(addr))
        }
    }
}