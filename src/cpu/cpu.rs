use crate::cpu::control::Control;
use crate::cpu::ld::LD;
use crate::memory_bus::memory_bus::MemoryBus;
use super::register::Register;


pub struct CPU {
    pc: u16,
    sp: u16,
    registers: Register,
    cycles: u64,
    is_running: bool,
}

impl CPU {
    pub fn new () -> Self {
        CPU {
            pc: 0x00,
            sp: 0x00,
            registers: Register::new(),
            cycles: 0,
            is_running: true
        }
    }

    pub fn step(&mut self, bus: &mut MemoryBus) {
        let opcode = bus.read(self.pc);
        self.decode(opcode, bus);
    }

    pub fn set_running(&mut self, is_running: bool) {
        self.is_running = is_running;
    }

    pub fn get_pc(&self) -> u16 {
        self.pc
    }

    pub fn get_sp(&self) -> u16 { self.sp }
    
    pub fn get_registers(&mut self) -> &mut Register {
        &mut self.registers
    }

    pub fn change_pc(&mut self, new_pc: u16) {
        self.pc = new_pc;
    }
    
    pub fn set_sp(&mut self, new_sp: u16) {
        self.sp = new_sp;
    }

    pub fn add_cycles(&mut self, c: u64){
        self.cycles += c;
    }
    
    pub fn update_pc_and_cycles(&mut self, pc: u16, cycles: u64) {
        self.pc = pc;
        self.cycles += cycles;
    }

    pub fn decode(&mut self, opcode: u8, bus: &mut MemoryBus) {
        let x: u8 = (opcode & 0b11000000) >> 6; //instruction
        let y: u8 = (opcode & 0b00111000) >> 3; //op1
        let z: u8 = opcode & 0b00000111; //op2

        dbg!(x,y,z);

        match(x,y,z){
            (0, 0, 0) => Control::nop(self), //0x00
            (0, 0, 1) => LD::ld_bc_n16(self, bus), //0x01
            (0, 0, 2) => LD::ld_bc_a(self, bus),
            (0, 0, 3) => Control::inc_bc(self),
            (0, 0, 4) => Control::inc_b(self),
            (0, 0, 5) => Control::dec_b(self),
            (0, 0, 6) => LD::ld_b_n8(self, bus),
            (0, 0, 7) => Control::rlca(self),
            (0, 1, 0) => LD::ld_a16_sp(self, bus),
            (0, 1, 1) => Control::add_hl_bc(self),
            (0, 1, 2) => LD::ld_a_bc(self, bus),
            (0, 1, 3) => Control::dec_bc(self),
            (0, 1, 4) => Control::inc_c(self),
            (0, 1, 5) => Control::dec_c(self),
            (0, 1, 6) => LD::ld_c_n8(self, bus),
            (0, 1, 7) => Control::rrca(self),
            
            (0, 2, 0) => Control::stop(self),
            (0, 2, 1) => LD::ld_de_n16(self, bus),
            (0, 2, 2) => LD::ld_de_a(self, bus),
            (0, 2, 3) => Control::inc_de(self),
            (0, 2, 4) => Control::inc_d(self),
            (0, 2, 5) => Control::dec_d(self),
            (0, 2, 6) => LD::ld_d_n8(self, bus),
            (0, 2, 7) => Control::rla(self),
            (0, 3, 0) => Control::jr_e8(self, bus),
            (0, 3, 1) => Control::add_hl_de(self),
            (0, 3, 2) => LD::ld_a_de(self, bus),
            (0, 3, 3) => Control::dec_de(self),
            (0, 3, 4) => Control::inc_e(self),
            (0, 3, 5) => Control::dec_e(self),
            (0, 3, 6) => LD::ld_e_n8(self, bus),
            (0, 3, 7) => Control::rra(self),
            
            (0, 4, 0) => Control::jr_nz_e8(self, bus),
            (0, 4, 1) => LD::ld_hl_n16(self, bus),
            (0, 4, 2) => LD::ld_hl_plus_a(self, bus),
            (0, 4, 3) => Control::inc_hl(self),
            (0, 4, 4) => Control::inc_h(self),
            (0, 4, 5) => Control::dec_h(self),
            (0, 4, 6) => LD::ld_h_n8(self, bus),
            (0, 4, 7) => Control::daa(self),
            (0, 5, 0) => Control::jr_z_n8(self, bus),
            (0, 5, 1) => Control::add_hl_hl(self),
            (0, 5, 2) => LD::ld_a_hl_plus(self, bus),
            (0, 5, 3) => Control::dec_hl(self),
            (0, 5, 4) => Control::inc_l(self),
            (0, 5, 5) => Control::dec_l(self),
            (0, 5, 6) => LD::ld_l_n8(self, bus),
            (0, 5, 7) => Control::cpl(self),
            
            (0, 6, 0) => Control::jr_nc_e8(self, bus),
            (0, 6, 1) => LD::ld_sp_n16(self, bus),
            (0, 6, 2) => LD::ld_hl_minus_a(self, bus),
            (0, 6, 3) => Control::inc_sp(self),
            (0, 6, 4) => Control::inc_hl_(self, bus),
            (0, 6, 5) => Control::dec_hl_(self, bus),
            (0, 6, 6) => LD::ld_hl_n8(self, bus),
            (0, 6, 7) => Control::scf(self),
            (0, 7, 0) => Control::jr_c_e8(self, bus),
            (0, 7, 1) => Control::add_hl_sp(self),
            (0, 7, 2) => LD::ld_a_hl_minus(self, bus),
            (0, 7, 3) => Control::dec_sp(self),
            (0, 7, 4) => Control::inc_a(self),
            (0, 7, 5) => Control::dec_a(self),
            (0, 7, 6) => LD::ld_a_n8(self, bus),
            (0, 7, 7) => Control::ccf(self),

            (1, dst, src) if dst != 6 && src != 6 => {
                LD::ld_r8_r8(self, dst as usize, src as usize);
            }
            (1, 6, src) => {
                // LD (HL), r
                LD::ld_hl_r(self, bus, src as usize)
            }
            (1, dst, 6) => {
                //LD r, (HL)
                LD::ld_r_hl(self, bus, dst as usize)
            }
            
            /*(1, 0, 0) => LD::ld_r8_r8(self, Register::set_b, Register::get_b),
            (1, 0, 1) => LD::ld_r8_r8(self, Register::set_b, Register::get_c),
            (1, 0, 2) => LD::ld_r8_r8(self, Register::set_b, Register::get_d),
            (1, 0, 3) => LD::ld_r8_r8(self, Register::set_b, Register::get_e),
            (1, 0, 4) => LD::ld_r8_r8(self, Register::set_b, Register::get_h),
            (1, 0, 5) => LD::ld_r8_r8(self, Register::set_b, Register::get_l),
            (1, 0, 6) => todo!(),
            (1, 0, 7) => LD::ld_r8_r8(self, Register::set_b, Register::get_a),
            (1, 1, 0) => LD::ld_r8_r8(self, Register::set_c, Register::get_b),
            (1, 1, 1) => LD::ld_r8_r8(self, Register::set_c, Register::get_c),
            (1, 1, 2) => LD::ld_r8_r8(self, Register::set_c, Register::get_d),
            (1, 1, 3) => LD::ld_r8_r8(self, Register::set_c, Register::get_e),
            (1, 1, 4) => LD::ld_r8_r8(self, Register::set_c, Register::get_h),
            (1, 1, 5) => LD::ld_r8_r8(self, Register::set_c, Register::get_l),
            (1, 1, 6) => todo!(),
            (1, 1, 7) => LD::ld_r8_r8(self, Register::set_c, Register::get_a),
            
            (1, 2, 0) => LD::ld_r8_r8(self, Register::set_d, Register::get_b),
            (1, 2, 1) => LD::ld_r8_r8(self, Register::set_d, Register::get_c),
            (1, 2, 2) => LD::ld_r8_r8(self, Register::set_d, Register::get_d),
            (1, 2, 3) => LD::ld_r8_r8(self, Register::set_d, Register::get_e),
            (1, 2, 4) => LD::ld_r8_r8(self, Register::set_d, Register::get_h),
            (1, 2, 5) => LD::ld_r8_r8(self, Register::set_d, Register::get_l),
            (1, 2, 6) => todo!(),
            (1, 2, 7) => LD::ld_r8_r8(self, Register::set_d, Register::get_a),
            (1, 3, 0) => LD::ld_r8_r8(self, Register::set_e, Register::get_b),
            (1, 3, 1) => LD::ld_r8_r8(self, Register::set_e, Register::get_c),
            (1, 3, 2) => LD::ld_r8_r8(self, Register::set_e, Register::get_d),
            (1, 3, 3) => LD::ld_r8_r8(self, Register::set_e, Register::get_e),
            (1, 3, 4) => LD::ld_r8_r8(self, Register::set_e, Register::get_h),
            (1, 3, 5) => LD::ld_r8_r8(self, Register::set_e, Register::get_l),
            (1, 3, 6) => todo!(),
            (1, 3, 7) => LD::ld_r8_r8(self, Register::set_e, Register::get_a),
            
            (1, 4, 0) => LD::ld_r8_r8(self, Register::set_h, Register::get_b),
            (1, 4, 1) => LD::ld_r8_r8(self, Register::set_h, Register::get_c),
            (1, 4, 2) => LD::ld_r8_r8(self, Register::set_h, Register::get_d),
            (1, 4, 3) => LD::ld_r8_r8(self, Register::set_h, Register::get_e),
            (1, 4, 4) => LD::ld_r8_r8(self, Register::set_h, Register::get_h),
            (1, 4, 5) => LD::ld_r8_r8(self, Register::set_h, Register::get_l),
            (1, 4, 6) => todo!(),
            (1, 4, 7) => LD::ld_r8_r8(self, Register::set_h, Register::get_a),
            (1, 5, 0) => LD::ld_r8_r8(self, Register::set_l, Register::get_b),
            (1, 5, 1) => LD::ld_r8_r8(self, Register::set_l, Register::get_c),
            (1, 5, 2) => LD::ld_r8_r8(self, Register::set_l, Register::get_d),
            (1, 5, 3) => LD::ld_r8_r8(self, Register::set_l, Register::get_e),
            (1, 5, 4) => LD::ld_r8_r8(self, Register::set_l, Register::get_h),
            (1, 5, 5) => LD::ld_r8_r8(self, Register::set_l, Register::get_l),
            (1, 5, 6) => todo!(),
            (1, 5, 7) => LD::ld_r8_r8(self, Register::set_l, Register::get_a),
            
            (1, 6, 0) => todo!(),
            (1, 6, 1) => todo!(),
            (1, 6, 2) => todo!(),
            (1, 6, 3) => todo!(),
            (1, 6, 4) => todo!(),
            (1, 6, 5) => todo!(),
            (1, 6, 6) => todo!(),
            (1, 6, 7) => todo!(),
            (1, 7, 0) => LD::ld_r8_r8(self, Register::set_a, Register::get_b),
            (1, 7, 1) => LD::ld_r8_r8(self, Register::set_a, Register::get_c),
            (1, 7, 2) => LD::ld_r8_r8(self, Register::set_a, Register::get_d),
            (1, 7, 3) => LD::ld_r8_r8(self, Register::set_a, Register::get_e),
            (1, 7, 4) => LD::ld_r8_r8(self, Register::set_a, Register::get_h),
            (1, 7, 5) => LD::ld_r8_r8(self, Register::set_a, Register::get_l),
            (1, 7, 6) => todo!(),
            (1, 7, 7) => LD::ld_r8_r8(self, Register::set_a, Register::get_a),*/
            
            (2, 0, 0) => todo!(),
            (2, 0, 1) => todo!(),
            (2, 0, 2) => todo!(),
            (2, 0, 3) => todo!(),
            (2, 0, 4) => todo!(),
            (2, 0, 5) => todo!(),
            (2, 0, 6) => todo!(),
            (2, 0, 7) => todo!(),
            (2, 1, 0) => todo!(),
            (2, 1, 1) => todo!(),
            (2, 1, 2) => todo!(),
            (2, 1, 3) => todo!(),
            (2, 1, 4) => todo!(),
            (2, 1, 5) => todo!(),
            (2, 1, 6) => todo!(),
            (2, 1, 7) => todo!(),
            
            (2, 2, 0) => todo!(),
            (2, 2, 1) => todo!(),
            (2, 2, 2) => todo!(),
            (2, 2, 3) => todo!(),
            (2, 2, 4) => todo!(),
            (2, 2, 5) => todo!(),
            (2, 2, 6) => todo!(),
            (2, 2, 7) => todo!(),
            (2, 3, 0) => todo!(),
            (2, 3, 1) => todo!(),
            (2, 3, 2) => todo!(),
            (2, 3, 3) => todo!(),
            (2, 3, 4) => todo!(),
            (2, 3, 5) => todo!(),
            (2, 3, 6) => todo!(),
            (2, 3, 7) => todo!(),
            
            (2, 4, 0) => todo!(),
            (2, 4, 1) => todo!(),
            (2, 4, 2) => todo!(),
            (2, 4, 3) => todo!(),
            (2, 4, 4) => todo!(),
            (2, 4, 5) => todo!(),
            (2, 4, 6) => todo!(),
            (2, 4, 7) => todo!(),
            (2, 5, 0) => todo!(),
            (2, 5, 1) => todo!(),
            (2, 5, 2) => todo!(),
            (2, 5, 3) => todo!(),
            (2, 5, 4) => todo!(),
            (2, 5, 5) => todo!(),
            (2, 5, 6) => todo!(),
            (2, 5, 7) => todo!(),
            
            (2, 6, 0) => todo!(),
            (2, 6, 1) => todo!(),
            (2, 6, 2) => todo!(),
            (2, 6, 3) => todo!(),
            (2, 6, 4) => todo!(),
            (2, 6, 5) => todo!(),
            (2, 6, 6) => todo!(),
            (2, 6, 7) => todo!(),
            (2, 7, 0) => todo!(),
            (2, 7, 1) => todo!(),
            (2, 7, 2) => todo!(),
            (2, 7, 3) => todo!(),
            (2, 7, 4) => todo!(),
            (2, 7, 5) => todo!(),
            (2, 7, 6) => todo!(),
            (2, 7, 7) => todo!(),
            
            (3, 0, 0) => todo!(),
            (3, 0, 1) => todo!(),
            (3, 0, 2) => todo!(),
            (3, 0, 3) => todo!(),
            (3, 0, 4) => todo!(),
            (3, 0, 5) => todo!(),
            (3, 0, 6) => todo!(),
            (3, 0, 7) => todo!(),
            (3, 1, 0) => todo!(),
            (3, 1, 1) => todo!(),
            (3, 1, 2) => todo!(),
            (3, 1, 3) => todo!(),
            (3, 1, 4) => todo!(),
            (3, 1, 5) => todo!(),
            (3, 1, 6) => todo!(),
            (3, 1, 7) => todo!(),
            
            (3, 2, 0) => todo!(),
            (3, 2, 1) => todo!(),
            (3, 2, 2) => todo!(),
            (3, 2, 3) => todo!(),
            (3, 2, 4) => todo!(),
            (3, 2, 5) => todo!(),
            (3, 2, 6) => todo!(),
            (3, 2, 7) => todo!(),
            (3, 3, 0) => todo!(),
            (3, 3, 1) => todo!(),
            (3, 3, 2) => todo!(),
            (3, 3, 3) => todo!(),
            (3, 3, 4) => todo!(),
            (3, 3, 5) => todo!(),
            (3, 3, 6) => todo!(),
            (3, 3, 7) => todo!(),
            
            (3, 4, 0) => todo!(),
            (3, 4, 1) => todo!(),
            (3, 4, 2) => todo!(),
            (3, 4, 3) => todo!(),
            (3, 4, 4) => todo!(),
            (3, 4, 5) => todo!(),
            (3, 4, 6) => todo!(),
            (3, 4, 7) => todo!(),
            (3, 5, 0) => todo!(),
            (3, 5, 1) => todo!(),
            (3, 5, 2) => todo!(),
            (3, 5, 3) => todo!(),
            (3, 5, 4) => todo!(),
            (3, 5, 5) => todo!(),
            (3, 5, 6) => todo!(),
            (3, 5, 7) => todo!(),
            
            (3, 6, 0) => todo!(),
            (3, 6, 1) => todo!(),
            (3, 6, 2) => todo!(),
            (3, 6, 3) => todo!(),
            (3, 6, 4) => todo!(),
            (3, 6, 5) => todo!(),
            (3, 6, 6) => todo!(),
            (3, 6, 7) => todo!(),
            (3, 7, 0) => todo!(),
            (3, 7, 1) => todo!(),
            (3, 7, 2) => todo!(),
            (3, 7, 3) => todo!(),
            (3, 7, 4) => todo!(),
            (3, 7, 5) => todo!(),
            (3, 7, 6) => todo!(),
            (3, 7, 7) => todo!(),
            
            _ => panic!("NOT IMPLEMENTED")
        }
    }

}