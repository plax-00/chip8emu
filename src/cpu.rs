use crate::{chip8::PROGRAM_START, display::Display, instruction::Instruction, memory::Memory};

#[derive(Debug)]
#[allow(dead_code)]
pub struct CPU {
    v: [u8; 16],
    i: u16,
    dt: u8,
    st: u8,
    pub pc: u16,
    sp: u8,
    stack: [u16; 16],
}

impl CPU {
    pub fn new() -> Self {
        Self {
            v: [0; 16],
            i: 0,
            dt: 0,
            st: 0,
            pc: PROGRAM_START,
            sp: 0,
            stack: [0; 16],
        }
    }

    fn read_reg(&self, register: u8) -> u8 {
        if register >= 16 {
            panic!("Register {} does not exist", register);
        }

        self.v[register as usize]
    }

    fn write_reg(&mut self, register: u8, data: u8) {
        if register >= 16 {
            panic!("Register {} does not exist", register);
        }

        self.v[register as usize] = data;
    }

    pub fn decrement_timers(&mut self) {
        if self.dt > 0 {
            self.dt -= 1;
        }
        if self.st > 0 {
            self.st -= 1;
        }
    }

    pub fn execute(
        &mut self,
        instruction: &Instruction,
        display: &mut Display,
        memory: &mut Memory,
        keys: &[bool; 16],
    ) {
        use Instruction as Ins;

        match *instruction {
            Ins::SYS(_) => (),
            Ins::CLS => display.clear(),
            Ins::RET => {
                if self.sp <= 0 {
                    panic!("Stack underflow");
                }
                self.pc = self.stack[self.sp as usize];
                self.sp -= 1;
            }
            Ins::JP(addr) => self.pc = addr,
            Ins::CALL(addr) => {
                if self.sp >= 15 {
                    panic!("Stack overflow");
                }
                self.sp += 1;
                self.stack[self.sp as usize] = self.pc;
                self.pc = addr;
            }
            Ins::SE_byte(vx, byte) => {
                if self.read_reg(vx) == byte {
                    self.pc += 2
                }
            }
            Ins::SNE_byte(vx, byte) => {
                if self.read_reg(vx) != byte {
                    self.pc += 2
                }
            }
            Ins::SE_reg(vx, vy) => {
                if self.read_reg(vx) == self.read_reg(vy) {
                    self.pc += 2
                }
            }
            Ins::LD_byte(vx, byte) => self.write_reg(vx, byte),
            Ins::ADD_byte(vx, byte) => {
                let data = self.read_reg(vx).wrapping_add(byte);
                self.write_reg(vx, data);
            }
            Ins::LD_reg(vx, vy) => {
                let data = self.read_reg(vy);
                self.write_reg(vx, data);
            }
            Ins::OR(vx, vy) => {
                let data = self.read_reg(vx) | self.read_reg(vy);
                self.write_reg(vx, data);
            }
            Ins::AND(vx, vy) => {
                let data = self.read_reg(vx) & self.read_reg(vy);
                self.write_reg(vx, data);
            }
            Ins::XOR(vx, vy) => {
                let data = self.read_reg(vx) ^ self.read_reg(vy);
                self.write_reg(vx, data);
            }
            Ins::ADD_reg(vx, vy) => {
                let (data, overflow) = self.read_reg(vx).overflowing_add(self.read_reg(vy));
                self.write_reg(vx, data);
                self.write_reg(0xF, if overflow { 1 } else { 0 });
            }
            Ins::SUB(vx, vy) => {
                let (data, overflow) = self.read_reg(vx).overflowing_sub(self.read_reg(vy));
                self.write_reg(vx, data);
                self.write_reg(0xF, if overflow { 0 } else { 1 });
            }
            Ins::SHR(vx) => {
                let least_sig_bit = self.read_reg(vx) & 0x1;
                self.write_reg(0xF, least_sig_bit);
                self.write_reg(vx, self.read_reg(vx) >> 1);
            }
            Ins::SUBN(vx, vy) => {
                let (data, overflow) = self.read_reg(vy).overflowing_sub(self.read_reg(vx));
                self.write_reg(vx, data);
                self.write_reg(0xF, if overflow { 0 } else { 1 });
            }
            Ins::SHL(vx) => {
                let most_sig_bit = (self.read_reg(vx) & 0x80) >> 7;
                self.write_reg(0xF, most_sig_bit);
                self.write_reg(vx, self.read_reg(vx) << 1);
            }
            Ins::SNE_reg(vx, vy) => {
                if self.read_reg(vx) != self.read_reg(vy) {
                    self.pc += 2
                }
            }
            Ins::LD_I(addr) => self.i = addr,
            Ins::JP_V0(addr) => self.pc = addr + self.read_reg(0x0) as u16,
            Ins::RND(vx, byte) => {
                let data = rand::random::<u8>() & byte;
                self.write_reg(vx, data);
            }
            Ins::DRW(vx, vy, byte) => {
                let start_x = self.read_reg(vx);
                let start_y = self.read_reg(vy);
                let sprite = memory.get_sprite(self.i, byte);
                let collision = display.draw(start_x, start_y, sprite);
                self.write_reg(0xF, collision);
            }
            Ins::SKP(vx) => {
                if keys[self.read_reg(vx) as usize] {
                    self.pc += 2;
                }
            }
            Ins::SKNP(vx) => {
                if !keys[self.read_reg(vx) as usize] {
                    self.pc += 2;
                }
            }
            Ins::LD_reg_from_DT(vx) => self.write_reg(vx, self.dt),
            Ins::LD_reg_from_key(vx) => {
                for (i, key) in keys.iter().enumerate() {
                    if *key {
                        self.write_reg(vx, i as u8);
                        return;
                    }
                }

                self.pc -= 2;
            }
            Ins::LD_DT_from_reg(vx) => self.dt = self.read_reg(vx),
            Ins::LD_ST_from_reg(vx) => self.st = self.read_reg(vx),
            Ins::ADD_I(vx) => self.i += self.read_reg(vx) as u16,
            Ins::LD_F(vx) => self.i = memory.get_font_sprite_addr(self.read_reg(vx)),
            Ins::LD_B(vx) => {
                let value = self.read_reg(vx);
                let hundreds = (value - (value % 100)) / 100;
                let tens = ((value - hundreds * 100) - (value % 10)) / 10;
                let ones = value - hundreds * 100 - tens * 10;
                memory.write_byte(self.i, hundreds);
                memory.write_byte(self.i + 1, tens);
                memory.write_byte(self.i + 2, ones);
            }
            Ins::LD_I_write(vx) => {
                let mut addr = self.i;
                for reg in 0..=vx {
                    memory.write_byte(addr, self.read_reg(reg));
                    addr += 1;
                }
            }
            Ins::LD_I_read(vx) => {
                let mut addr = self.i;
                for reg in 0..=vx {
                    self.write_reg(reg, memory.read_byte(addr));
                    addr += 1;
                }
            }
        }
    }
}
