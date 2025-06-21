use crate::cpu::flag::Flag;
use crate::utils::byte_utils::{format_u16, get_lsb_u16, get_msb_u16};

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
        format_u16(self.h, self.l)
    }
    
    pub fn get_f_mut(&mut self) -> &mut Flag {
        &mut self.f
    }
    
    pub fn get_f(&self) -> &Flag {
        &self.f
    }   
    
    pub fn get_bc(&self) -> u16 {
        format_u16(self.b, self.c)
    }

    pub fn get_de(&self) -> u16 {
        format_u16(self.d, self.e)
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
    
    pub fn set_d(&mut self, value: u8) {
        self.d = value;
    }

    pub fn set_e(&mut self, value: u8) {
        self.e = value;
    }

    pub fn set_h(&mut self, value: u8) {
        self.h = value;
    }

    pub fn set_l(&mut self, value: u8) {
        self.l = value;
    }
    
    pub fn set_hl(&mut self, value: u16) {
        let high: u8 = get_msb_u16(value);
        let low: u8 = get_lsb_u16(value);
        self.h = high;
        self.l = low;
    }
    
    pub fn set_bc(&mut self, value: u16) {
        let high: u8 = get_msb_u16(value);
        let low: u8 = get_lsb_u16(value);
        self.b = high;
        self.c = low;
    }

    pub fn set_de(&mut self, value: u16) {
        let high: u8 = get_msb_u16(value);
        let low: u8 = get_lsb_u16(value);
        self.d = high;
        self.e = low;
    }
    
    
    
}