use crate::error::memory_error::MemoryError;
use crate::memory_bus::bus::BUS;

pub struct EchoRAM{
    r: [u8; 0x1E00] //8 kiB = 8192 bytes = 0x2000
}

impl EchoRAM{
    pub fn new() -> EchoRAM{
        EchoRAM{
            r: [0; 0x1E00]
        }
    }
}

impl BUS for EchoRAM {
    fn read(&self, addr: u16) -> Result<u8, MemoryError> {
        dbg!("Reading from ECHO RAM");
        match addr {
            0xE000..=0xFDFF => Ok(self.r[(addr - 0xE000) as usize]),
            _ => Err(MemoryError::InvalidAddress(addr))
        }
    }

    fn write(&mut self, addr: u16, data: u8) -> Result<(), MemoryError> {
        match addr {
            0xE000..=0xFDFF => {
                self.r[(addr - 0xE000) as usize] = data;
                Ok(())
            },
            _ => Err(MemoryError::InvalidAddress(addr))
        }
    }
}