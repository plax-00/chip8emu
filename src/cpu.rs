use crate::{
    memory::Memory,
    chip8::PROGRAM_START,
    instruction::Instruction
};


#[derive(Debug)]
pub struct CPU {
    vx: [u8; 16],
    i: u16,
    dt: u8,
    st: u8,
    pc: u16,
    sp: u8,
}

impl CPU {
    pub fn new() -> CPU {
        CPU { vx: [0; 16], i: 0, dt: 0, st: 0, pc: PROGRAM_START, sp: 0 }
    }

    pub fn read_register(self, register: u8) -> u8 {
        if register >= 15 {
            panic!("Register {} does not exist", register);
        }

        self.vx[register as usize]
    }

    pub fn write_register(&mut self, register: u8, data: u8) {
        if register >= 15 {
            panic!("Register {} does not exist", register);
        }

        self.vx[register as usize] = data;
    }

    pub fn execute(&mut self, memory: &mut Memory, instruction: Instruction) {
        match instruction {
            _ => todo!(),
        }
    }
}
