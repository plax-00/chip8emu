use wasm_bindgen::prelude::*;
use crate::{cpu::CPU, memory::Memory};


pub const PROGRAM_START: u16 = 0x200;

#[derive(Debug)]
#[wasm_bindgen]
pub struct Chip8 {
	cpu: CPU,
	memory: Memory,
}

#[wasm_bindgen]
impl Chip8 {
	#[wasm_bindgen(constructor)]
	pub fn new() -> Self {
		Self { cpu: CPU::new(), memory: Memory::new() }
	}

	pub fn print_mem(&self) -> String {
		self.memory.print_mem()
	}
}
