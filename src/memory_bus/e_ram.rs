use crate::error::memory_error::MemoryError;
use crate::memory_bus::bus::BUS;

pub struct ExternalRAM{
    r: [u8; 0x2000] //8 kiB = 8192 bytes = 0x2000 
}

impl ExternalRAM{
    pub fn new() -> ExternalRAM{
        ExternalRAM{
            r: [0; 0x2000]
        }
    }
}

impl BUS for ExternalRAM {
    fn read(&self, addr: u16) -> Result<u8, MemoryError> {
        dbg!("Reading from External RAM");
        match addr {
            0xA000..=0xBFFF => Ok(self.r[(addr - 0xA000) as usize]),
            _ => Err(MemoryError::InvalidAddress(addr))
        }
    }

    fn write(&mut self, addr: u16, data: u8) -> Result<(), MemoryError> {
        match addr {
            0xA000..=0xBFFF => {
                self.r[(addr - 0xA000) as usize] = data;
                Ok(())
            },
            _ => Err(MemoryError::InvalidAddress(addr))
        }
    }
}