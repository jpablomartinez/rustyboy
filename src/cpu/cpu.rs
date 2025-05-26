use super::register::Register;

pub struct CPU {
    pc: u16,
    sp: u16,
    registers: Register
}

impl CPU {
    pub fn new () -> Self {
        CPU {
            pc: 0x00,
            sp: 0x00,
            registers: Register::new()
        }
    }

    pub fn decode(opcode: u8) {
        let x: u8 = (opcode & 0b11000000) >> 6; //instruction
        let y: u8 = (opcode & 0b00111000) >> 3; //op1
        let z: u8 = opcode & 0b00000111; //op2

        dbg!(x,y,z);

        /*match(x,y,z){
            

        }*/

    }

}