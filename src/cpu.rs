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
    stack: [u16; 16],
}

impl CPU {
    pub fn new() -> CPU {
        CPU { vx: [0; 16], i: 0, dt: 0, st: 0, pc: PROGRAM_START, sp: 0, stack: [0; 16] }
    }

    pub fn read_register(&self, register: u8) -> u8 {
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
        use Instruction::*;

        match instruction {
            SYS(_) => unimplemented!(),
            CLS => todo!(),
            RET => {
                self.pc = self.stack[self.sp as usize];
                self.sp -= 1;
            },
            JP(addr) => self.pc = addr,
            CALL(addr) => {
                self.sp += 1;
                self.stack[self.sp as usize] = self.pc;
                self.pc = addr;
            },
            SE_byte(vx, byte) => if vx == byte { self.pc += 2 },
            SNE_byte(vx, byte) => if vx != byte { self.pc += 2 },
            SE_reg(vx, vy) => if vx == vy { self.pc += 2 },
            LD_byte(vx, byte) => self.write_register(vx, byte),
            ADD_byte(vx, byte) => {
                let data = self.read_register(vx) + byte;
                self.write_register(vx, data);
            },
            LD_reg(vx, vy) => {
                let data = self.read_register(vy);
                self.write_register(vx, data);
            },
            OR(vx, vy) => {
                let data = self.read_register(vx) | self.read_register(vy);
                self.write_register(vx, data);
            },
            AND(vx, vy) => {
                let data = self.read_register(vx) & self.read_register(vy);
                self.write_register(vx, data);
            },
            XOR(vx, vy) => {
                let data = self.read_register(vx) ^ self.read_register(vy);
                self.write_register(vx, data);
            },
            ADD_reg(vx, vy) => {
                let (data, overflow) = self.read_register(vx).overflowing_add(self.read_register(vy));
                self.write_register(vx, data);
                self.write_register(0xF, if overflow { 1 } else { 0 });
            },
            SUB(vx, vy) => {
                let (data, overflow) = self.read_register(vx).overflowing_sub(self.read_register(vy));
                self.write_register(vx, data);
                self.write_register(0xF, if overflow { 0 } else { 1 });
            },
            SHR(vx) => {
                let first_bit = (vx & 0x80) >> 7;
                self.write_register(0xF, if first_bit == 1 { 1 } else { 0 });
                self.write_register(vx, self.read_register(vx) >> 1);
            },
            SUBN(vx, vy) => {
                let (data, overflow) = self.read_register(vy).overflowing_sub(self.read_register(vx));
                self.write_register(vx, data);
                self.write_register(0xF, if overflow { 0 } else { 1 });
            },
            SHL(vx) => {
                let first_bit = (vx & 0x80) >> 7;
                self.write_register(0xF, if first_bit == 1 { 1 } else { 0 });
                self.write_register(vx, self.read_register(vx) << 1);
            },
            SNE_reg(vx, vy) => if self.read_register(vx) != self.read_register(vy) { self.pc += 2 },
            LD_I(addr) => self.i = addr,
            JP_V0(addr) => self.pc = addr + self.read_register(0x0) as u16,
            RND(vx, byte) => {
                let data = rand::random::<u8>() & byte;
                self.write_register(vx, data);
            },
            DRW(vx, vy, byte) => todo!(),
            SKP(vx) => todo!(),
            SKNP(vx) => todo!(),
            LD_reg_from_DT(vx) => self.write_register(vx, self.dt),
            LD_reg_from_key(vx) => todo!(),
            LD_DT_from_reg(vx) => self.dt = self.read_register(vx),
            LD_ST_from_reg(vx) => self.st = self.read_register(vx),
            ADD_I(vx) => self.i += self.read_register(vx) as u16,
            LD_F(vx) => todo!(),
            LD_B(vx) => todo!(),
            LD_I_write(vx) => {
                let mut addr = self.i;
                for reg in 0..=vx {
                    memory.write_byte(addr, self.read_register(reg));
                    addr += 1;
                }
            },
            LD_I_read(vx) => {
                let mut addr = self.i;
                for reg in 0..=vx {
                    self.write_register(reg, memory.read_byte(addr));
                    addr += 1;
                }
            }
        }
    }
}
