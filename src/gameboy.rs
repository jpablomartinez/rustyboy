use crate::memory_bus::rom::ROM;
use crate::cpu::cpu::CPU;
use crate::memory_bus::memory_bus::MemoryBus;

pub struct Gameboy {
    pub cpu: CPU,
    pub memory_bus: MemoryBus
}

#[warn(unused_must_use)]
impl Gameboy {
    pub fn new() -> Self{
        Self{cpu: CPU::new(), memory_bus: MemoryBus::new()}
    }
    pub fn start(&mut self, path: &str) {
        let r = self.memory_bus.rom.read(path);
        print!("ROM read result: ");
        match r {
            Ok(_) => println!("Success"),
            Err(e) => println!("Error: {}", e),
        }
    }
}