use crate::error::memory_error::MemoryError;
use crate::memory_bus::bus::BUS;

pub struct IO{
    r: [u8; 0x80] // 128 bytes 
}

impl IO{
    pub fn new() -> IO{
        IO{
            r: [0; 0x80]
        }
    }
}

impl BUS for IO {
    fn read(&self, addr: u16) -> Result<u8, MemoryError> {
        dbg!("Reading from I/O");
        match addr {
            0xFF00..=0xFF7F => Ok(self.r[(addr - 0xFF00) as usize]),
            _ => Err(MemoryError::InvalidAddress(addr))
        }
    }

    fn write(&mut self, addr: u16, data: u8) -> Result<(), MemoryError> {
        match addr {
            0x8000..=0xFF7F => {
                self.r[(addr - 0xFF00) as usize] = data;
                Ok(())
            },
            _ => Err(MemoryError::InvalidAddress(addr))
        }
    }
}