use crate::{cpu::CPU, memory::Memory};


pub const PROGRAM_START: u16 = 0x200;

#[derive(Debug)]
pub struct Chip8 {
    cpu: CPU,
    memory: Memory,
}

impl Chip8 {
    pub fn new() -> Self {
        Self { cpu: CPU::new(), memory: Memory::new() }
    }

    pub fn load_rom(&mut self, rom: &Vec<u8>) {
        let mut addr: u16 = PROGRAM_START;
        for byte in rom.iter() {
            self.memory.write_byte(addr, *byte);
            addr += 1;
        }
    }

    pub fn print_mem(&self) {
        self.memory.print_mem();
    }
}
