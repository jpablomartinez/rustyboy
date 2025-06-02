use crate::error::memory_error::MemoryError;
use crate::memory_bus::bus::BUS;

pub struct NotUsable{
    r: [u8; 0x60] // 128 bytes
}

impl NotUsable{
    pub fn new() -> NotUsable{
        NotUsable{
            r: [0; 0x60]
        }
    }
}

impl BUS for NotUsable {
    fn read(&self, addr: u16) -> Result<u8, MemoryError> {
        dbg!("Reading from NOT USABLE");
        match addr {
            0xFEA0..=0xFEFF => Ok(self.r[(addr - 0xFEA0) as usize]),
            _ => Err(MemoryError::InvalidAddress(addr))
        }
    }

    fn write(&mut self, addr: u16, data: u8) -> Result<(), MemoryError> {
        dbg!("In theory i cannot write here... ");
        match addr {
            0xFEA0..=0xFEFF => {
                self.r[(addr - 0xFEA0) as usize] = data;
                Ok(())
            },
            _ => Err(MemoryError::InvalidAddress(addr))
        }
    }
}