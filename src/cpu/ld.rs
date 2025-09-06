use crate::cpu::cpu::CPU;
use crate::cpu::register::Register;
use crate::memory_bus::memory_bus::MemoryBus;
use crate::utils::byte_utils::{format_u16, get_lsb_u16, get_msb_u16};


pub struct LD;

impl LD {

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

    pub fn ld_b_n8(cpu: &mut CPU, memory_bus: &mut MemoryBus){
        let value: u8 = memory_bus.read(cpu.get_pc() + 1);
        cpu.get_registers().set_b(value);
        cpu.update_pc_and_cycles(cpu.get_pc() + 2, 8);
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

    pub fn ld_a_bc(cpu: &mut CPU, memory_bus: &mut MemoryBus){
        let bc: u16 = cpu.get_registers().get_bc();
        let value: u8 = memory_bus.read(bc);
        cpu.get_registers().set_a(value);
        cpu.update_pc_and_cycles(cpu.get_pc() + 1, 8);
    }

    pub fn ld_c_n8(cpu: &mut CPU, memory_bus: &mut MemoryBus){
        let value: u8 = memory_bus.read(cpu.get_pc() + 1);
        cpu.get_registers().set_c(value);
        cpu.update_pc_and_cycles(cpu.get_pc() + 2, 8);
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

    pub fn ld_d_n8(cpu: &mut CPU, memory_bus: &mut MemoryBus){
        let value: u8 = memory_bus.read(cpu.get_pc() + 1);
        cpu.get_registers().set_d(value);
        cpu.update_pc_and_cycles(cpu.get_pc() + 2, 8);
    }

    pub fn ld_a_de(cpu: &mut CPU, memory_bus: &mut MemoryBus){
        let de: u16 = cpu.get_registers().get_de();
        let value: u8 = memory_bus.read(de);
        cpu.get_registers().set_a(value);
        cpu.update_pc_and_cycles(cpu.get_pc() + 1, 8);
    }

    pub fn ld_e_n8(cpu: &mut CPU, memory_bus: &mut MemoryBus){
        let value: u8 = memory_bus.read(cpu.get_pc() + 1);
        cpu.get_registers().set_e(value);
        cpu.update_pc_and_cycles(cpu.get_pc() + 2, 8);
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

    pub fn ld_h_n8(cpu: &mut CPU, memory_bus: &mut MemoryBus){
        let n8: u16 = cpu.get_pc() + 1;
        let value: u8 = memory_bus.read(n8);
        cpu.get_registers().set_h(value);
        cpu.update_pc_and_cycles(cpu.get_pc() + 2, 8);
    }

    pub fn ld_a_hl_plus(cpu: &mut CPU, memory_bus: &mut MemoryBus){
        let hl: u16 = cpu.get_registers().get_hl();
        let value: u8 = memory_bus.read(hl);
        cpu.get_registers().set_a(value);
        let new_hl = hl.wrapping_add(1);
        cpu.get_registers().set_hl(new_hl);
        cpu.update_pc_and_cycles(cpu.get_pc() + 1, 8);
    }

    pub fn ld_l_n8(cpu: &mut CPU, memory_bus: &mut MemoryBus){
        let offset_addr: u16 = cpu.get_pc().wrapping_add(1);
        let offset: u8 = memory_bus.read(offset_addr) as u8;
        cpu.get_registers().set_l(offset);
        cpu.update_pc_and_cycles(cpu.get_pc() + 2, 8);
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

    pub fn ld_hl_n8(cpu: &mut CPU, memory_bus: &mut MemoryBus){
        let offset_address: u16 = cpu.get_pc().wrapping_add(1);
        let register: u8 = memory_bus.read(offset_address);
        let hl: u16 = cpu.get_registers().get_hl();
        memory_bus.write(hl, register);
        cpu.update_pc_and_cycles(cpu.get_pc().wrapping_add(2), 12);
    }

    pub fn ld_a_hl_minus(cpu: &mut CPU, memory_bus: &mut MemoryBus){
        let hl_address: u16 = cpu.get_registers().get_hl();
        let value: u8 = memory_bus.read(hl_address);
        cpu.get_registers().set_a(value);
        let new_hl = hl_address.wrapping_sub(1);
        cpu.get_registers().set_hl(new_hl);
        cpu.update_pc_and_cycles(cpu.get_pc().wrapping_add(1), 8);
    }

    pub fn ld_a_n8(cpu: &mut CPU, memory_bus: &mut MemoryBus){
        let addr: u16 = cpu.get_pc().wrapping_add(1);
        let value: u8 = memory_bus.read(addr);
        cpu.get_registers().set_a(value);
        cpu.update_pc_and_cycles(cpu.get_pc().wrapping_add(2), 8);
    }

    pub fn ld_r8_r8(cpu: &mut CPU, setter: fn(&mut Register, u8), getter: fn(&Register) -> u8){
        let value = {
            let registers = cpu.get_registers();
            getter(registers)
        };
        let registers = cpu.get_registers();
        setter(registers, value);
        cpu.update_pc_and_cycles(cpu.get_pc() + 1, 4);
    }
}