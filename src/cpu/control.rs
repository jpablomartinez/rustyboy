use crate::cpu::cpu::CPU;

pub struct Control;

impl Control {
    pub fn nop (cpu: &mut CPU){
        cpu.add_cycles(4);
    }
}

