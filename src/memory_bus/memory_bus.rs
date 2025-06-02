use crate::memory_bus::bus::BUS;
use crate::memory_bus::e_ram::ExternalRAM;
use crate::memory_bus::echo_ram::EchoRAM;
use crate::memory_bus::h_ram::HRAM;
use crate::memory_bus::interrupt::Interrupt;
use crate::memory_bus::io::IO;
use crate::memory_bus::not_usable::NotUsable;
use crate::memory_bus::oam::OAM;
use crate::memory_bus::rom::ROM;
use crate::memory_bus::v_ram::VRAM;
use crate::memory_bus::w_ram::WRAM;

pub struct MemoryBus {
    pub rom: ROM,
    v_ram: VRAM,
    e_ram: ExternalRAM,
    w_ram: WRAM,
    echo_ram: EchoRAM,
    io: IO,
    h_ram: HRAM,
    oam: OAM,
    interrupt: Interrupt,
    not_usable: NotUsable

}

impl MemoryBus{
    pub fn new () -> Self{
        MemoryBus {
            rom: ROM::new(),
            v_ram: VRAM::new(),
            e_ram: ExternalRAM::new(),
            w_ram: WRAM::new(),
            echo_ram: EchoRAM::new(),
            io: IO::new(),
            h_ram: HRAM::new(),
            oam: OAM::new(),
            interrupt: Interrupt::new(),
            not_usable: NotUsable::new()
        }
    }
    
    pub fn read(&self, addr: u16) -> u8{
        match addr {
            0x0000..=0x7FFF => self.rom.read_byte(addr),
            0x8000..=0x9FFF => self.v_ram.read(addr).expect(&format!("Invalid addr for VRAM {:04X} ",addr)),
            0xA000..=0xBFFF => self.e_ram.read(addr).expect(&format!("Invalid addr for EXTERNAL RAM {:04X} ",addr)),
            0xC000..=0xDFFF => self.w_ram.read(addr).expect(&format!("Invalid addr for WORK RAM {:04X} ",addr)),
            0xE000..=0xFDFF => self.echo_ram.read(addr).expect(&format!("Invalid addr for ECHO RAM {:04X} ",addr)),
            0xFE00..=0xFE9F => self.oam.read(addr).expect(&format!("Invalid addr for OAM {:04X} ",addr)),
            0xFEA0..=0xFEFF => self.not_usable.read(addr).expect(&format!("Invalid addr for NOT USABLE {:04X}",addr)),
            0xFF00..=0xFF7F => self.io.read(addr).expect(&format!("Invalid addr for I/O {:04X} ",addr)),
            0xFF80..=0xFFFE => self.h_ram.read(addr).expect(&format!("Invalid addr for HRAM {:04X} ",addr)),
            0xFFFF..=0xFFFF => self.interrupt.read(addr).expect(&format!("Invalid addr for Interrupt {:04X} ",addr)),
            _ => panic!("Invalid addr {:04X} ", addr)       
        }
    }
    
    pub fn write(&mut self, addr: u16, value: u8) {
        match addr {
            0x8000..=0x9FFF => self.v_ram.write(addr, value).expect(&format!("Invalid addr for VRAM {:04X} ",addr)),
            0xA000..=0xBFFF => self.e_ram.write(addr, value).expect(&format!("Invalid addr for EXTERNAL RAM {:04X} ",addr)),
            0xC000..=0xDFFF => self.w_ram.write(addr, value).expect(&format!("Invalid addr for WORK RAM {:04X} ",addr)),
            0xE000..=0xFDFF => self.echo_ram.write(addr, value).expect(&format!("Invalid addr for ECHO RAM {:04X} ",addr)),
            0xFE00..=0xFE9F => self.oam.write(addr, value).expect(&format!("Invalid addr for OAM {:04X} ",addr)),
            0xFEA0..=0xFEFF => self.not_usable.write(addr, value).expect(&format!("Invalid addr for NOT USABLE {:04X}",addr)),
            0xFF00..=0xFF7F => self.io.write(addr, value).expect(&format!("Invalid addr for I/O {:04X} ",addr)),
            0xFF80..=0xFFFE => self.h_ram.write(addr, value).expect(&format!("Invalid addr for HRAM {:04X} ",addr)),
            0xFFFF          => self.interrupt.write(addr, value).expect(&format!("Invalid addr for Interrupt {:04X} ",addr)),
            _ => panic!("Invalid addr {:04X} ", addr)
        }
    }
    
    
}