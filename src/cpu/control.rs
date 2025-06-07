use crate::cpu::cpu::CPU;

pub struct Control;

impl Control {
    pub fn nop (cpu: &mut CPU){
        cpu.add_cycles(4);
    }

    pub fn ld_bc_n16(cpu: &mut CPU){
        cpu.add_cycles(12);
    }

    pub fn ld_bc_a(cpu: &mut CPU){
        cpu.add_cycles(8);
    }

    pub fn inc_bc(cpu: &mut CPU){
        cpu.add_cycles(8);
    }

    pub fn inc_b(cpu: &mut CPU){
        cpu.add_cycles(4);
    }

    pub fn dec_b(cpu: &mut CPU){
        cpu.add_cycles(4);
    }

    pub fn ld_b_n8(cpu: &mut CPU){
        cpu.add_cycles(8);
    }

    pub fn rlca (cpu: &mut CPU){
        cpu.add_cycles(4);
    }

    pub fn ld_a16_sp(cpu: &mut CPU){
        cpu.add_cycles(20);
    }

    pub fn add_hl_bc(cpu: &mut CPU){
        cpu.add_cycles(8);
    }

    pub fn ld_a_bc(cpu: &mut CPU){
        cpu.add_cycles(8);
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

