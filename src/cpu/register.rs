use crate::cpu::flag::Flag;

pub struct Register {
    a: u8,
    b: u8,
    c: u8,
    d: u8,
    e: u8,
    f: Flag,
    h: u8,
    l: u8
}

impl Register {
    pub fn new () -> Self {
        Register {
            a: 0x0,
            b: 0x0,
            c: 0x0,
            d: 0x0,
            e: 0x0,
            f: Flag::new(),
            h: 0x0,
            l: 0x0
        }
    }
    
    pub fn get_a(&self) -> u8 {
        self.a
    }

    pub fn get_b(&self) -> u8 {
        self.b
    }

    pub fn get_c(&self) -> u8 {
        self.b
    }

    pub fn get_d(&self) -> u8 {
        self.b
    }

    pub fn get_e(&self) -> u8 {
        self.b
    }

    pub fn get_h(&self) -> u8 {
        self.b
    }

    pub fn get_l(&self) -> u8 {
        self.b
    }

    pub fn get_hl(&self) -> u16 {
        ((self.h as u16) << 8) | (self.l as u16)
    }
    
    pub fn get_f_mut(&mut self) -> &mut Flag {
        &mut self.f
    }
    
    pub fn get_memory_addresses_bc(&self) -> u16 {
        (self.b as u16) << 8 | self.c as u16
    }

    pub fn set_a(&mut self, value: u8) {
        self.a = value;
    }

    pub fn set_b(&mut self, value: u8) {
        self.b = value;
    }

    pub fn set_c(&mut self, value: u8) {
        self.c = value;
    }
    
    
    
}