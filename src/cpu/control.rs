use std::ptr::write_volatile;
use crate::constants::flags::{C_FLAG, Z_FLAG};
use crate::cpu::cpu::CPU;
use crate::memory_bus::memory_bus::MemoryBus;
use crate::utils::byte_utils::{format_u16, get_lsb_u16, get_msb_u16};

pub struct Control;

impl Control {
    pub fn nop (cpu: &mut CPU){
        cpu.update_pc_and_cycles(cpu.get_pc() + 1, 4);
    }

    pub fn ld_bc_n16(cpu: &mut CPU, memory_bus: &mut  MemoryBus) {
        let low_byte = memory_bus.read(cpu.get_pc() + 1);
        let high_byte = memory_bus.read(cpu.get_pc() + 2);
        cpu.get_registers().set_b(high_byte);
        cpu.get_registers().set_c(low_byte);
        cpu.update_pc_and_cycles(cpu.get_pc() + 3, 12);
    }

    pub fn ld_bc_a(cpu: &mut CPU, memory_bus: &mut MemoryBus){
        let a: u8 = cpu.get_registers().get_a();
        let address: u16 = cpu.get_registers().get_bc();
        memory_bus.write(address, a);
        cpu.update_pc_and_cycles(cpu.get_pc() + 1, 8);
    }

    pub fn inc_bc(cpu: &mut CPU){
        let bc: u16 = cpu.get_registers().get_bc();
        let tmp: u16 = bc.wrapping_add(1);
        cpu.get_registers().set_b(get_msb_u16(tmp));
        cpu.get_registers().set_c(get_lsb_u16(tmp));
        cpu.update_pc_and_cycles(cpu.get_pc() + 1, 8);
    }

    pub fn inc_b(cpu: &mut CPU){
        let b: u8 = cpu.get_registers().get_b();
        let r: u8 = b.wrapping_add(1);
        let h: bool = (b & 0x0F) + 1 > 0xF;
        let c: bool = cpu.get_registers().get_f_mut().get_flag(C_FLAG);
        cpu.get_registers().get_f_mut().set_flags(c,false, h,r == 0);
        cpu.get_registers().set_b(r);
        cpu.update_pc_and_cycles(cpu.get_pc() + 1, 4);
    }

    pub fn dec_b(cpu: &mut CPU){
        let b: u8 = cpu.get_registers().get_b();
        let r: u8 = b.wrapping_sub(1);
        let h: bool = (b & 0x0F) == 0x00;
        let c: bool = cpu.get_registers().get_f_mut().get_flag(C_FLAG);
        cpu.get_registers().get_f_mut().set_flags(c,true, h,r == 0);
        cpu.get_registers().set_b(r);
        cpu.update_pc_and_cycles(cpu.get_pc() + 1, 4);
    }

    pub fn ld_b_n8(cpu: &mut CPU, memory_bus: &mut MemoryBus){
        let value: u8 = memory_bus.read(cpu.get_pc() + 1);
        cpu.get_registers().set_b(value);
        cpu.update_pc_and_cycles(cpu.get_pc() + 2, 8);
    }

    pub fn rlca (cpu: &mut CPU){
        let a: u8 = cpu.get_registers().get_a();
        let msm: u8 = (a & 0x80) >> 7;
        let r: u8 = (a << 1) | msm;
        cpu.get_registers().set_a(r);
        cpu.get_registers().get_f_mut().set_flags(msm == 1,false,false,false);
        cpu.update_pc_and_cycles(cpu.get_pc() + 1, 4);
    }

    pub fn ld_a16_sp(cpu: &mut CPU, memory_bus: &mut MemoryBus){
        let a_low_byte = memory_bus.read(cpu.get_pc() + 1);
        let a_high_byte = memory_bus.read(cpu.get_pc() + 2);
        //TODO: a16 == 0xFFFF check this if works
        let a16: u16 = format_u16(a_high_byte, a_low_byte);
        let sp_lsb: u8 =  get_lsb_u16(cpu.get_sp());
        let sp_msb: u8 = get_msb_u16(cpu.get_sp());
        memory_bus.write(a16, sp_lsb);
        memory_bus.write(a16.wrapping_add(1), sp_msb);
        cpu.update_pc_and_cycles(cpu.get_pc() + 3, 20);
    }

    pub fn add_hl_bc(cpu: &mut CPU){
        let bc: u16 = cpu.get_registers().get_bc();
        let hl: u16 = cpu.get_registers().get_hl();
        let r: u16 = bc.wrapping_add(hl);
        cpu.get_registers().set_hl(r);

        let h: bool = ((hl & 0x0FFF) + (bc & 0x0FFF)) > 0x0FFF;
        let c: bool = (hl as u32 + bc as u32) > 0xFFFF;
        
        //Z flag is not affected by ADD HL, xx — preserved explicitly
        let z: bool = cpu.get_registers().get_f_mut().get_flag(Z_FLAG);
        cpu.get_registers().get_f_mut().set_flags(c,false,h,z);
        cpu.update_pc_and_cycles(cpu.get_pc() + 1, 8);
    }

    pub fn ld_a_bc(cpu: &mut CPU, memory_bus: &mut MemoryBus){
        let bc: u16 = cpu.get_registers().get_bc();
        let value: u8 = memory_bus.read(bc);
        cpu.get_registers().set_a(value);
        cpu.update_pc_and_cycles(cpu.get_pc() + 1, 8);
    }

    pub fn dec_bc(cpu: &mut CPU){
        cpu.add_cycles(8);
    }

    pub fn inc_c(cpu: &mut CPU){
        cpu.add_cycles(4);
    }

    pub fn dec_c(cpu: &mut CPU){
        cpu.add_cycles(4);
    }

    pub fn ld_c_n8(cpu: &mut CPU){
        cpu.add_cycles(8);
    }

    pub fn rrca(cpu: &mut CPU){
        cpu.add_cycles(4);
    }

    pub fn stop(cpu: &mut CPU){
        cpu.add_cycles(4);
    }

    pub fn ld_de_n16(cpu: &mut CPU){
        cpu.add_cycles(12);
    }

    pub fn ld_de_a(cpu: &mut CPU){
        cpu.add_cycles(8);
    }

    pub fn inc_de(cpu: &mut CPU){
        cpu.add_cycles(8);
    }

    pub fn inc_d(cpu: &mut CPU){
        cpu.add_cycles(4);
    }

    pub fn dec_d(cpu: &mut CPU){
        cpu.add_cycles(4);
    }

    pub fn ld_d_n8(cpu: &mut CPU){
        cpu.add_cycles(8);
    }

    pub fn rla (cpu: &mut CPU){
        cpu.add_cycles(4);
    }

    pub fn jr_e8(cpu: &mut CPU){
        cpu.add_cycles(12);
    }

    pub fn add_hl_de(cpu: &mut CPU){
        cpu.add_cycles(8);
    }

    pub fn ld_a_de(cpu: &mut CPU){
        cpu.add_cycles(8);
    }

    pub fn dec_de(cpu: &mut CPU){
        cpu.add_cycles(8);
    }

    pub fn inc_e(cpu: &mut CPU){
        cpu.add_cycles(4);
    }

    pub fn dec_e(cpu: &mut CPU){
        cpu.add_cycles(4);
    }

    pub fn ld_e_n8(cpu: &mut CPU){
        cpu.add_cycles(8);
    }

    pub fn rra(cpu: &mut CPU){
        cpu.add_cycles(4);
    }

    pub fn jr_nz_e8(cpu: &mut CPU){}

    pub fn ld_hl_n16(cpu: &mut CPU){
        cpu.add_cycles(12);
    }

    pub fn ld_hl_plus_a(cpu: &mut CPU){
        cpu.add_cycles(8);
    }

    pub fn inc_hl(cpu: &mut CPU){
        cpu.add_cycles(8);
    }

    pub fn inc_h(cpu: &mut CPU){
        cpu.add_cycles(4);
    }

    pub fn dec_h(cpu: &mut CPU){
        cpu.add_cycles(4);
    }

    pub fn ld_h_n8(cpu: &mut CPU){
        cpu.add_cycles(8);
    }

    pub fn daa(cpu: &mut CPU){
        cpu.add_cycles(4);
    }

    pub fn jr_z_n8(cpu: &mut CPU){}

    pub fn add_hl_hl(cpu: &mut CPU){
        cpu.add_cycles(8);
    }

    pub fn ld_a_hl_plus(cpu: &mut CPU){
        cpu.add_cycles(8);
    }

    pub fn dec_hl(cpu: &mut CPU){
        cpu.add_cycles(8);
    }

    pub fn inc_l(cpu: &mut CPU){
        cpu.add_cycles(4);
    }

    pub fn dec_l(cpu: &mut CPU){
        cpu.add_cycles(4);
    }

    pub fn ld_l_n8(cpu: &mut CPU){
        cpu.add_cycles(8);
    }

    pub fn cpl(cpu: &mut CPU){
        cpu.add_cycles(4);
    }

    pub fn jr_nc_e8(cpu: &mut CPU){

    }

    pub fn ld_sp_n16(cpu: &mut CPU){
        cpu.add_cycles(12);
    }

    pub fn ld_hl_minus_a(cpu: &mut CPU){
        cpu.add_cycles(8);
    }

    pub fn inc_sp(cpu: &mut CPU){
        cpu.add_cycles(8);
    }

    pub fn inc_hl_(cpu: &mut CPU){
        cpu.add_cycles(12);
    }

    pub fn dec_hl_(cpu: &mut CPU){
        cpu.add_cycles(12);
    }

    pub fn ld_hl_n8(cpu: &mut CPU){
        cpu.add_cycles(12);
    }

    pub fn scf(cpu: &mut CPU){
        cpu.add_cycles(4);
    }

    pub fn jr_c_e8(cpu: &mut CPU){

    }

    pub fn add_hl_sp(cpu: &mut CPU){
        cpu.add_cycles(8);
    }

    pub fn ld_a_hl_minus(cpu: &mut CPU){
        cpu.add_cycles(8);
    }

    pub fn dec_sp(cpu: &mut CPU){
        cpu.add_cycles(8);
    }

    pub fn inc_a(cpu: &mut CPU){
        cpu.add_cycles(4);
    }

    pub fn dec_a(cpu: &mut CPU){
        cpu.add_cycles(4);
    }

    pub fn ld_a_n8(cpu: &mut CPU){
        cpu.add_cycles(8);
    }

    pub fn ccf(cpu: &mut CPU){
        cpu.add_cycles(4);
    }

}

