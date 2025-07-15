use std::ptr::write_volatile;
use crate::constants::flags::{C_FLAG, H_FLAG, N_FLAG, Z_FLAG};
use crate::cpu::cpu::CPU;
use crate::memory_bus::memory_bus::MemoryBus;
use crate::utils::byte_utils::{format_u16, get_carry_inc_16b, get_half_carry_inc, get_half_carry_inc_16b, get_lsb_u16, get_lsb_u8, get_msb_u16, get_msb_u8};

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
        let bc: u16 = cpu.get_registers().get_bc();
        let r: u16 = bc.wrapping_sub(1);
        cpu.get_registers().set_bc(r);
        cpu.update_pc_and_cycles(cpu.get_pc() + 1, 8);
    }

    pub fn inc_c(cpu: &mut CPU){
        let c: u8 = cpu.get_registers().get_c();
        let r: u8 = c.wrapping_add(1);
        let h: bool = (c & 0x0F) + 1 > 0xF;
        let _c: bool = cpu.get_registers().get_f_mut().get_flag(C_FLAG);
        cpu.get_registers().get_f_mut().set_flags(_c,false, h,r == 0);
        cpu.get_registers().set_c(r);
        cpu.update_pc_and_cycles(cpu.get_pc() + 1, 4);
    }

    pub fn dec_c(cpu: &mut CPU){
        let c: u8 = cpu.get_registers().get_c();
        let r: u8 = c.wrapping_sub(1);
        let h: bool = (c & 0x0F) == 0x00;
        let _c: bool = cpu.get_registers().get_f_mut().get_flag(C_FLAG);
        cpu.get_registers().get_f_mut().set_flags(_c,true, h,r == 0);
        cpu.get_registers().set_c(r);
        cpu.update_pc_and_cycles(cpu.get_pc() + 1, 4);
    }

    pub fn ld_c_n8(cpu: &mut CPU, memory_bus: &mut MemoryBus){
        let value: u8 = memory_bus.read(cpu.get_pc() + 1);
        cpu.get_registers().set_c(value);
        cpu.update_pc_and_cycles(cpu.get_pc() + 2, 8);
    }

    pub fn rrca(cpu: &mut CPU){
        let a: u8 = cpu.get_registers().get_a();
        let lsb: u8 = get_lsb_u8(a);
        let r: u8 = (a >> 1) | (lsb << 7);
        cpu.get_registers().set_a(r);
        cpu.get_registers().get_f_mut().set_flags(lsb == 1,false,false,false);
        cpu.update_pc_and_cycles(cpu.get_pc() + 1, 4);
    }

    pub fn stop(cpu: &mut CPU){
        cpu.set_running(false);
        cpu.update_pc_and_cycles(cpu.get_pc() + 2, 4);
    }

    pub fn ld_de_n16(cpu: &mut CPU, memory_bus: &mut MemoryBus){
        let low_byte = memory_bus.read(cpu.get_pc() + 1);
        let high_byte = memory_bus.read(cpu.get_pc() + 2);
        cpu.get_registers().set_d(high_byte);
        cpu.get_registers().set_e(low_byte);
        cpu.update_pc_and_cycles(cpu.get_pc() + 3, 12);
    }

    pub fn ld_de_a(cpu: &mut CPU, memory_bus:&mut MemoryBus){
        let a: u8 = cpu.get_registers().get_a();
        let address: u16 = cpu.get_registers().get_de();
        memory_bus.write(address, a);
        cpu.update_pc_and_cycles(cpu.get_pc() + 1, 8);
    }

    pub fn inc_de(cpu: &mut CPU){
        let de: u16 = cpu.get_registers().get_de();
        let tmp: u16 = de.wrapping_add(1);
        cpu.get_registers().set_d(get_msb_u16(tmp));
        cpu.get_registers().set_e(get_lsb_u16(tmp));
        cpu.update_pc_and_cycles(cpu.get_pc() + 1, 8);
    }

    pub fn inc_d(cpu: &mut CPU){
        let d: u8 = cpu.get_registers().get_d();
        let r: u8 = d.wrapping_add(1);
        let h: bool = (d & 0x0F) + 1 > 0xF;
        let _c: bool = cpu.get_registers().get_f_mut().get_flag(C_FLAG);
        cpu.get_registers().get_f_mut().set_flags(_c,false, h,r == 0);
        cpu.get_registers().set_d(r);
        cpu.update_pc_and_cycles(cpu.get_pc() + 1, 4);
    }

    pub fn dec_d(cpu: &mut CPU){
        let d: u8 = cpu.get_registers().get_d();
        let r: u8 = d.wrapping_sub(1);
        let h: bool = (d & 0x0F) == 0x00;
        let _c: bool = cpu.get_registers().get_f_mut().get_flag(C_FLAG);
        cpu.get_registers().get_f_mut().set_flags(_c,true, h,r == 0);
        cpu.get_registers().set_d(r);
        cpu.update_pc_and_cycles(cpu.get_pc() + 1, 4);
    }

    pub fn ld_d_n8(cpu: &mut CPU, memory_bus: &mut MemoryBus){
        let value: u8 = memory_bus.read(cpu.get_pc() + 1);
        cpu.get_registers().set_d(value);
        cpu.update_pc_and_cycles(cpu.get_pc() + 2, 8);
    }

    pub fn rla (cpu: &mut CPU){
        let a: u8 = cpu.get_registers().get_a();
        let c: u8 = if cpu.get_registers().get_f_mut().get_flag(C_FLAG) { 1 }  else { 0 } ;
        let msb: u8 = get_msb_u8(cpu.get_registers().get_a());
        let r: u8 = (a << 1) | c; //a << 1 rotate to left
        cpu.get_registers().set_a(r);
        let new_carry: bool = msb == 1;
        cpu.get_registers().get_f_mut().set_flags(new_carry,false, false,false);
        cpu.update_pc_and_cycles(cpu.get_pc() + 1, 4);
    }

    pub fn jr_e8(cpu: &mut CPU, memory_bus: &mut MemoryBus){
        let pc: i16 = cpu.get_pc() as i16;
        let offset_addr: u16 = cpu.get_pc().wrapping_add(1);
        let offset: i8 = memory_bus.read(offset_addr) as i8;
        let new_pc: u16 = pc.wrapping_add(2).wrapping_add(offset as i16) as u16;
        cpu.update_pc_and_cycles(new_pc, 12);
    }

    pub fn add_hl_de(cpu: &mut CPU){
        let de: u16 = cpu.get_registers().get_de();
        let hl: u16 = cpu.get_registers().get_hl();
        let r: u16 = de.wrapping_add(hl);
        cpu.get_registers().set_hl(r);

        let h: bool = ((hl & 0x0FFF) + (de & 0x0FFF)) > 0x0FFF;
        let c: bool = (hl as u32 + de as u32) > 0xFFFF;

        let z: bool = cpu.get_registers().get_f_mut().get_flag(Z_FLAG);
        cpu.get_registers().get_f_mut().set_flags(c,false,h,z);
        cpu.update_pc_and_cycles(cpu.get_pc() + 1, 8);
    }

    pub fn ld_a_de(cpu: &mut CPU, memory_bus: &mut MemoryBus){
        let de: u16 = cpu.get_registers().get_de();
        let value: u8 = memory_bus.read(de);
        cpu.get_registers().set_a(value);
        cpu.update_pc_and_cycles(cpu.get_pc() + 1, 8);
    }

    pub fn dec_de(cpu: &mut CPU){
        let de: u16 = cpu.get_registers().get_de();
        let r: u16 = de.wrapping_sub(1);
        cpu.get_registers().set_de(r);
        cpu.update_pc_and_cycles(cpu.get_pc() + 1, 8);
    }

    pub fn inc_e(cpu: &mut CPU){
        let e: u8 = cpu.get_registers().get_e();
        let r: u8 = e.wrapping_add(1);
        let h: bool = (e & 0x0F) + 1 > 0xF;
        let _c: bool = cpu.get_registers().get_f_mut().get_flag(C_FLAG);
        cpu.get_registers().get_f_mut().set_flags(_c,false, h,r == 0);
        cpu.get_registers().set_e(r);
        cpu.update_pc_and_cycles(cpu.get_pc() + 1, 4);
    }

    pub fn dec_e(cpu: &mut CPU){
        let e: u8 = cpu.get_registers().get_e();
        let r: u8 = e.wrapping_sub(1);
        let h: bool = (e & 0x0F) == 0x00;
        let _c: bool = cpu.get_registers().get_f_mut().get_flag(C_FLAG);
        cpu.get_registers().get_f_mut().set_flags(_c,true, h,r == 0);
        cpu.get_registers().set_e(r);
        cpu.update_pc_and_cycles(cpu.get_pc() + 1, 4);
    }

    pub fn ld_e_n8(cpu: &mut CPU, memory_bus: &mut MemoryBus){
        let value: u8 = memory_bus.read(cpu.get_pc() + 1);
        cpu.get_registers().set_e(value);
        cpu.update_pc_and_cycles(cpu.get_pc() + 2, 8);
    }

    pub fn rra(cpu: &mut CPU){
        let a: u8 = cpu.get_registers().get_a();
        let lsb: u8 = get_lsb_u8(a);
        let old_carry: u8 = if cpu.get_registers().get_f_mut().get_flag(C_FLAG) { 1 }  else { 0 } ;
        let new_carry: bool = lsb == 1;
        let r: u8 = (a >> 1) | (old_carry << 7);
        cpu.get_registers().set_a(r);
        cpu.get_registers().get_f_mut().set_flags(new_carry,false, false,false);
        cpu.update_pc_and_cycles(cpu.get_pc() + 1, 4);
    }

    pub fn jr_nz_e8(cpu: &mut CPU, memory_bus: &mut MemoryBus){
        let z_flag = cpu.get_registers().get_f().get_flag(Z_FLAG);
        if(z_flag){
            cpu.update_pc_and_cycles(cpu.get_pc().wrapping_add(2), 8);
        } else {
            let pc: i16 = cpu.get_pc() as i16;
            let offset_addr: u16 = cpu.get_pc().wrapping_add(1);
            let offset: i8 = memory_bus.read(offset_addr) as i8;
            let new_pc: u16 = pc.wrapping_add(offset as i16) as u16;
            cpu.update_pc_and_cycles(new_pc, 12);
        }
    }

    pub fn ld_hl_n16(cpu: &mut CPU, memory_bus: &mut MemoryBus){
        let low_byte = memory_bus.read(cpu.get_pc() + 1);
        let high_byte = memory_bus.read(cpu.get_pc() + 2);
        cpu.get_registers().set_hl(format_u16(high_byte, low_byte));
        cpu.update_pc_and_cycles(cpu.get_pc() + 3, 12);
    }

    pub fn ld_hl_plus_a(cpu: &mut CPU, memory_bus: &mut MemoryBus){
        let a: u8 = cpu.get_registers().get_a();
        let hl: u16 = cpu.get_registers().get_hl();
        memory_bus.write(hl, a);
        let new_hl = hl.wrapping_add(1);
        cpu.get_registers().set_hl(new_hl);
        cpu.update_pc_and_cycles(cpu.get_pc() + 1, 8);
    }

    pub fn inc_hl(cpu: &mut CPU){
        let hl: u16 = cpu.get_registers().get_hl();
        let r: u16 = hl.wrapping_add(1);
        cpu.get_registers().set_hl(r);
        cpu.update_pc_and_cycles(cpu.get_pc() + 1, 8);
    }

    pub fn inc_h(cpu: &mut CPU){
        let h: u8 = cpu.get_registers().get_h();
        let r: u8 = h.wrapping_add(1);
        let half_carry: bool = (h & 0x0F) + 1 > 0xF;
        let carry: bool = cpu.get_registers().get_f_mut().get_flag(C_FLAG);
        cpu.get_registers().set_h(r);
        cpu.get_registers().get_f_mut().set_flags(carry,false, half_carry,r == 0);
        cpu.update_pc_and_cycles(cpu.get_pc() + 1, 4);
    }

    pub fn dec_h(cpu: &mut CPU){
        let h: u8 = cpu.get_registers().get_e();
        let r: u8 = h.wrapping_sub(1);
        let half_carry: bool = (h & 0x0F) == 0x00;
        let carry: bool = cpu.get_registers().get_f_mut().get_flag(C_FLAG);
        cpu.get_registers().get_f_mut().set_flags(carry,true, half_carry,r == 0);
        cpu.get_registers().set_h(r);
        cpu.update_pc_and_cycles(cpu.get_pc() + 1, 4);
    }

    pub fn ld_h_n8(cpu: &mut CPU, memory_bus: &mut MemoryBus){
        let n8: u16 = cpu.get_pc() + 1;
            let value: u8 = memory_bus.read(n8);
        cpu.get_registers().set_h(value);
        cpu.update_pc_and_cycles(cpu.get_pc() + 2, 8);
    }

    pub fn daa(cpu: &mut CPU){
        let a: u8 = cpu.get_registers().get_a();
        let n: bool = cpu.get_registers().get_f_mut().get_flag(N_FLAG);
        let half_carry = cpu.get_registers().get_f().get_flag(H_FLAG);
        let mut carry = cpu.get_registers().get_f().get_flag(C_FLAG);
        let mut r: u8 = a;
        if(!n) {
            if carry || a > 0x99 {
                r = r.wrapping_add(0x60);
                carry = true;
            }
            if half_carry || (a & 0x0F) > 0x09 {
                r = r.wrapping_add(0x06);
            }
        } else {
            if carry {
                r = r.wrapping_sub(0x60);
            }
            if half_carry {
                r = r.wrapping_sub(0x06);
            }
        }
        cpu.get_registers().set_a(r);
        cpu.get_registers().get_f_mut().set_flags(carry, n,false, r == 0);
        cpu.update_pc_and_cycles(cpu.get_pc() + 1, 4);
    }

    pub fn jr_z_n8(cpu: &mut CPU, memory_bus: &mut MemoryBus){
        let pc: u16 = cpu.get_pc();
        let offset_addr: u16 = cpu.get_pc().wrapping_add(1);
        let offset: i8 = memory_bus.read(offset_addr) as i8;
        let z_flag = cpu.get_registers().get_f().get_flag(Z_FLAG);
        if z_flag{
            let new_pc: u16 = (pc as i16).wrapping_add(2).wrapping_add(offset as i16) as u16;
            cpu.update_pc_and_cycles(new_pc, 12);
        } else {
            cpu.update_pc_and_cycles(pc.wrapping_add(2) as u16, 8);
        }
    }

    pub fn add_hl_hl(cpu: &mut CPU){
        let hl: u16 = cpu.get_registers().get_hl();
        let r: u16 = hl.wrapping_add(hl);
        cpu.get_registers().set_hl(r);
        
        let h: bool =   ((hl & 0x0FFF) + (hl & 0x0FFF)) > 0x0FFF;
        let c: bool = (hl as u32 + hl as u32) > 0xFFFF;
        let z: bool = cpu.get_registers().get_f_mut().get_flag(Z_FLAG);
        
        cpu.get_registers().get_f_mut().set_flags(c,false,h,z);
        cpu.update_pc_and_cycles(cpu.get_pc().wrapping_add(1), 8);
    }

    pub fn ld_a_hl_plus(cpu: &mut CPU, memory_bus: &mut MemoryBus){
        let hl: u16 = cpu.get_registers().get_hl();
        let value: u8 = memory_bus.read(hl);
        cpu.get_registers().set_a(value);
        let new_hl = hl.wrapping_add(1);
        cpu.get_registers().set_hl(new_hl);
        cpu.update_pc_and_cycles(cpu.get_pc() + 1, 8);
    }

    pub fn dec_hl(cpu: &mut CPU){
        let hl: u16 = cpu.get_registers().get_hl();
        let r: u16 = hl.wrapping_sub(1);
        cpu.get_registers().set_hl(r);
        cpu.update_pc_and_cycles(cpu.get_pc() + 1, 8);
    }

    pub fn inc_l(cpu: &mut CPU){
        let l: u8 = cpu.get_registers().get_l();
        let r: u8 = l.wrapping_add(1);
        let h: bool = get_half_carry_inc(l);
        let _c: bool = cpu.get_registers().get_f_mut().get_flag(C_FLAG);
        cpu.get_registers().get_f_mut().set_flags(_c,false, h,r == 0);
        cpu.get_registers().set_l(r);
        cpu.update_pc_and_cycles(cpu.get_pc() + 1, 4);
    }

    pub fn dec_l(cpu: &mut CPU){
        let l: u8 = cpu.get_registers().get_l();
        let r: u8 = l.wrapping_sub(1);
        let h: bool = (l & 0x0F) == 0x00;
        let _c: bool = cpu.get_registers().get_f_mut().get_flag(C_FLAG);
        cpu.get_registers().get_f_mut().set_flags(_c,true, h,r == 0);
        cpu.get_registers().set_l(r);
        cpu.update_pc_and_cycles(cpu.get_pc() + 1, 4);
    }

    pub fn ld_l_n8(cpu: &mut CPU, memory_bus: &mut MemoryBus){
        let offset_addr: u16 = cpu.get_pc().wrapping_add(1);
        let offset: u8 = memory_bus.read(offset_addr) as u8;
        cpu.get_registers().set_l(offset);
        cpu.update_pc_and_cycles(cpu.get_pc() + 2, 8);
    }

    pub fn cpl(cpu: &mut CPU){
        let a: u8 = cpu.get_registers().get_a();
        let r: u8 = !a;
        cpu.get_registers().set_a(r);
        let c: bool = cpu.get_registers().get_f_mut().get_flag(C_FLAG);
        let z: bool = cpu.get_registers().get_f_mut().get_flag(Z_FLAG);
        cpu.get_registers().get_f_mut().set_flags(c,true,true,z);
        cpu.update_pc_and_cycles(cpu.get_pc() + 1, 4);
    }

    pub fn jr_nc_e8(cpu: &mut CPU, memory_bus: &mut MemoryBus){
        let offset_addr: u16 = cpu.get_pc().wrapping_add(1);
        let offset: i8 = memory_bus.read(offset_addr) as i8;
        let c: bool = cpu.get_registers().get_f_mut().get_flag(C_FLAG);
        if !c {
            let new_pc: u16 = (cpu.get_pc() as i16).wrapping_add(2).wrapping_add(offset as i16) as u16;
            cpu.update_pc_and_cycles(new_pc, 12);
        } else {
            cpu.update_pc_and_cycles(cpu.get_pc().wrapping_add(2), 8);
        }
    }

    pub fn ld_sp_n16(cpu: &mut CPU, memory_bus: &mut MemoryBus){
        let low_byte = memory_bus.read(  cpu.get_pc().wrapping_add(1));
        let high_byte = memory_bus.read( cpu.get_pc().wrapping_add(2));
        let hl: u16 = format_u16(high_byte,low_byte);
        cpu.set_sp(hl);
        cpu.update_pc_and_cycles(cpu.get_pc().wrapping_add(3), 12);
    }

    pub fn ld_hl_minus_a(cpu: &mut CPU, memory_bus: &mut MemoryBus){
        let a: u8 = cpu.get_registers().get_a();
        let hl: u16 = cpu.get_registers().get_hl();
        memory_bus.write(hl, a);
        let new_hl = hl.wrapping_sub(1);
        cpu.get_registers().set_hl(new_hl);
        cpu.update_pc_and_cycles(cpu.get_pc().wrapping_add(1), 8);
    }

    pub fn inc_sp(cpu: &mut CPU){
        let sp: u16 = cpu.get_sp();
        let r: u16 = sp.wrapping_add(1);
        cpu.set_sp(r);
        cpu.update_pc_and_cycles(cpu.get_pc().wrapping_add(1), 8);
    }

    pub fn inc_hl_(cpu: &mut CPU, memory_bus: &mut MemoryBus){
        let address: u16 = cpu.get_registers().get_hl();
        let register: u8 = memory_bus.read(address);
        let r: u8 = register.wrapping_add(1);
        memory_bus.write(address, r);
        let c: bool = cpu.get_registers().get_f_mut().get_flag(C_FLAG);
        let h: bool = get_half_carry_inc(register);
        cpu.get_registers().get_f_mut().set_flags(c,false, h,r == 0);
        cpu.update_pc_and_cycles(cpu.get_pc().wrapping_add(1), 12);
    }

    pub fn dec_hl_(cpu: &mut CPU, memory_bus: &mut MemoryBus){
        let address: u16 = cpu.get_registers().get_hl();
        let register: u8 = memory_bus.read(address);
        let r: u8 = register.wrapping_sub(1);
        memory_bus.write(address, r);
        let c: bool = cpu.get_registers().get_f_mut().get_flag(C_FLAG);
        // Half-carry occurs if low-nibble underflows when subtracting 1
        let h: bool = (register & 0x0F) == 0x00;
        cpu.get_registers().get_f_mut().set_flags(c,true, h,r == 0);
        cpu.update_pc_and_cycles(cpu.get_pc().wrapping_add(1), 12);
    }

    pub fn ld_hl_n8(cpu: &mut CPU, memory_bus: &mut MemoryBus){
        let offset_address: u16 = cpu.get_pc().wrapping_add(1);
        let register: u8 = memory_bus.read(offset_address);
        let hl: u16 = cpu.get_registers().get_hl();
        memory_bus.write(hl, register);
        cpu.update_pc_and_cycles(cpu.get_pc().wrapping_add(2), 12);
    }

    pub fn scf(cpu: &mut CPU){
        let z: bool = cpu.get_registers().get_f_mut().get_flag(Z_FLAG);
        cpu.get_registers().get_f_mut().set_flags(true,false,false,z);
        cpu.update_pc_and_cycles(cpu.get_pc().wrapping_add(1), 4);
    }

    pub fn jr_c_e8(cpu: &mut CPU, memory_bus: &mut MemoryBus){
        let offset_addr: u16 = cpu.get_pc().wrapping_add(1);
        let c: bool = cpu.get_registers().get_f_mut().get_flag(C_FLAG);
        if c {
            let offset: i8 = memory_bus.read(offset_addr) as i8;
            let new_pc: u16 = (cpu.get_pc() as i16).wrapping_add(2).wrapping_add(offset as i16) as u16;
            cpu.update_pc_and_cycles(new_pc, 12);
        } else {
            cpu.update_pc_and_cycles(cpu.get_pc().wrapping_add(2), 8);
        }
    }

    pub fn add_hl_sp(cpu: &mut CPU){
        let hl: u16 = cpu.get_registers().get_hl();
        let sp: u16 = cpu.get_sp();
        let r: u16 = hl.wrapping_add(sp);
        cpu.get_registers().set_hl(r);
        let z: bool = cpu.get_registers().get_f_mut().get_flag(Z_FLAG);
        let h: bool = get_half_carry_inc_16b(hl, sp);
        let c: bool = get_carry_inc_16b(hl, sp);
        cpu.get_registers().get_f_mut().set_flags(c,false,h,z);
        cpu.update_pc_and_cycles(cpu.get_pc().wrapping_add(1), 8);
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

