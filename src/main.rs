use std::fs;
use std::process;

mod chip8;
mod cpu;
mod instruction;
mod memory;

use crate::chip8::Chip8;


fn main() {
    let mut chip8 = Chip8::new();

    let rom = fs::read("data/space_invaders.ch8").unwrap_or_else(|_| {
        println!("Error: Unable to read ROM");
        process::exit(1);
    });

    chip8.load_rom(&rom);
    chip8.print_mem();
}

