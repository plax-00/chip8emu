use wasm_bindgen::prelude::*;

use crate::{cpu::CPU, display::Display, instruction::Instruction, memory::Memory};

pub const PROGRAM_START: u16 = 0x200;

#[derive(Debug)]
#[wasm_bindgen]
pub struct Chip8 {
    cpu: CPU,
    display: Display,
    memory: Memory,
    keys: [bool; 16],
}

#[wasm_bindgen]
impl Chip8 {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self {
            cpu: CPU::new(),
            display: Display::new(),
            memory: Memory::new(),
            keys: [false; 16],
        }
    }

    pub fn tick(&mut self) {
        let high_byte = self.memory.read_byte(self.cpu.pc) as u16;
        let low_byte = self.memory.read_byte(self.cpu.pc + 1) as u16;
        let opcode = (high_byte << 8) | low_byte;
        let instruction = match Instruction::from_opcode(opcode) {
            Some(i) => i,
            None => panic!("Invalid instruction"),
        };
        self.cpu.pc += 2;
        self.cpu.decrement_timers();
        self.cpu.execute(
            &instruction,
            &mut self.display,
            &mut self.memory,
            &self.keys,
        );
    }

    pub fn get_display(&self) -> *const bool {
        self.display.get_ptr()
    }

    pub fn update_key(&mut self, key: u8, state: bool) {
        self.keys[key as usize] = state;
    }

    pub fn reset(&mut self) {
        self.display.clear();
        let _ = std::mem::replace(&mut self.memory, Memory::new());
        let _ = std::mem::replace(&mut self.cpu, CPU::new());
    }

    pub fn load_rom(&mut self, rom: Vec<u8>) {
        self.memory.load_rom(&rom);
    }
}
