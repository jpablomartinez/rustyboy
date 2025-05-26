mod gameboy;
mod memory_bus;
mod cpu;

use gameboy::Gameboy;

fn main() {
    let mut gameboy: Gameboy = Gameboy::new();
    gameboy.start("cpu_instrs/cpu_instrs.gb");    
    for i in 0..0x4000 {
        print!("{:02X} ", gameboy.rom.bank1[i]);
        if (i + 1) % 16 == 0 {
            println!();
        }
    }
}

