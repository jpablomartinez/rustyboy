use std::fs::File;
use std::io::{self, Read};

pub struct ROM {
    pub bank0: [u8; 0x4000],
    pub bank1: [u8; 0x4000],
}

impl ROM {
    pub fn new() -> Self {
        ROM {
            bank0: [0; 0x4000],
            bank1: [0; 0x4000]
        }
    }
    pub fn read(&mut self, path: &str) -> io::Result<()> {
        let mut file = File::open(path)?;
        let mut buffer = [0u8; 0x8000]; // 32 KB buffer
        let bytes_read = file.read(&mut buffer)?;
        let split = 0x4000;
        self.bank0.copy_from_slice(&buffer[..split]);
        if bytes_read > split {
            let end = std::cmp::min(bytes_read, 0x8000);
            self.bank1[..(end - split)].copy_from_slice(&buffer[split..end]);
        }
        Ok(())        
    }
}

//C3 20 C2 D6 05 30 FC 1F 30 00 CE 01 D0 C8 00 C9