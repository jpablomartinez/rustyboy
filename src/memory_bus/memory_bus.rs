use crate::memory_bus::ram::v_ram::VRAM;
use crate::memory_bus::ram::w_ram::WRAM;
use crate::memory_bus::rom::ROM;


pub struct MemoryBus {
    rom: ROM,
    w_ram: WRAM,
    v_ram: VRAM,
}

impl MemoryBus{
    pub fn new () -> Self{
        MemoryBus {
            rom: ROM::new(),
            w_ram: WRAM::new(),
            //v_ram: VRAM::new()
            v_ram: VRAM {},
        }
    }
}