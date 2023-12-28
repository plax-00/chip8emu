use crate::{
	memory::Memory,
	chip8::PROGRAM_START,
	instruction::Instruction
};


#[derive(Debug)]
#[allow(dead_code)]
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
	pub fn new() -> Self {
		Self { vx: [0; 16], i: 0, dt: 0, st: 0, pc: PROGRAM_START, sp: 0, stack: [0; 16] }
	}

	fn read_register(&self, register: u8) -> u8 {
		if register >= 15 {
			panic!("Register {} does not exist", register);
		}

		self.vx[register as usize]
	}

	fn write_register(&mut self, register: u8, data: u8) {
		if register >= 15 {
			panic!("Register {} does not exist", register);
		}

		self.vx[register as usize] = data;
	}

	pub fn step(&mut self, memory: &mut Memory) {
		let high_byte = memory.read_byte(self.pc) as u16;
		let low_byte = memory.read_byte(self.pc + 1) as u16;
		let opcode = (high_byte << 8) & low_byte;
		let instruction = match Instruction::from_opcode(opcode) {
			Some(i) => i,
			None => return
		};

		self.execute(memory, instruction);
	}

	fn execute(&mut self, memory: &mut Memory, instruction: Instruction) {
		use Instruction as Ins;

		match instruction {
			Ins::SYS(_) => unimplemented!(),
			Ins::CLS => todo!(),
			Ins::RET => {
				if self.sp == 0 {
					panic!("Stack underflow");
				}
				self.pc = self.stack[self.sp as usize];
				self.sp -= 1;
			},
			Ins::JP(addr) => self.pc = addr,
			Ins::CALL(addr) => {
				if self.sp == 15 {
					panic!("Stack overflow");
				}
				self.sp += 1;
				self.stack[self.sp as usize] = self.pc;
				self.pc = addr;
			},
			Ins::SE_byte(vx, byte) => if vx == byte { self.pc += 2 },
			Ins::SNE_byte(vx, byte) => if vx != byte { self.pc += 2 },
			Ins::SE_reg(vx, vy) => if vx == vy { self.pc += 2 },
			Ins::LD_byte(vx, byte) => self.write_register(vx, byte),
			Ins::ADD_byte(vx, byte) => {
				let data = self.read_register(vx) + byte;
				self.write_register(vx, data);
			},
			Ins::LD_reg(vx, vy) => {
				let data = self.read_register(vy);
				self.write_register(vx, data);
			},
			Ins::OR(vx, vy) => {
				let data = self.read_register(vx) | self.read_register(vy);
				self.write_register(vx, data);
			},
			Ins::AND(vx, vy) => {
				let data = self.read_register(vx) & self.read_register(vy);
				self.write_register(vx, data);
			},
			Ins::XOR(vx, vy) => {
				let data = self.read_register(vx) ^ self.read_register(vy);
				self.write_register(vx, data);
			},
			Ins::ADD_reg(vx, vy) => {
				let (data, overflow) = self.read_register(vx).overflowing_add(self.read_register(vy));
				self.write_register(vx, data);
				self.write_register(0xF, if overflow { 1 } else { 0 });
			},
			Ins::SUB(vx, vy) => {
				let (data, overflow) = self.read_register(vx).overflowing_sub(self.read_register(vy));
				self.write_register(vx, data);
				self.write_register(0xF, if overflow { 0 } else { 1 });
			},
			Ins::SHR(vx) => {
				let first_bit = (vx & 0x80) >> 7;
				self.write_register(0xF, if first_bit == 1 { 1 } else { 0 });
				self.write_register(vx, self.read_register(vx) >> 1);
			},
			Ins::SUBN(vx, vy) => {
				let (data, overflow) = self.read_register(vy).overflowing_sub(self.read_register(vx));
				self.write_register(vx, data);
				self.write_register(0xF, if overflow { 0 } else { 1 });
			},
			Ins::SHL(vx) => {
				let first_bit = (vx & 0x80) >> 7;
				self.write_register(0xF, if first_bit == 1 { 1 } else { 0 });
				self.write_register(vx, self.read_register(vx) << 1);
			},
			Ins::SNE_reg(vx, vy) => if self.read_register(vx) != self.read_register(vy) { self.pc += 2 },
			Ins::LD_I(addr) => self.i = addr,
			Ins::JP_V0(addr) => self.pc = addr + self.read_register(0x0) as u16,
			Ins::RND(vx, byte) => {
				let data = rand::random::<u8>() & byte;
				self.write_register(vx, data);
			},
			Ins::DRW(vx, vy, byte) => todo!(),
			Ins::SKP(vx) => todo!(),
			Ins::SKNP(vx) => todo!(),
			Ins::LD_reg_from_DT(vx) => self.write_register(vx, self.dt),
			Ins::LD_reg_from_key(vx) => todo!(),
			Ins::LD_DT_from_reg(vx) => self.dt = self.read_register(vx),
			Ins::LD_ST_from_reg(vx) => self.st = self.read_register(vx),
			Ins::ADD_I(vx) => self.i += self.read_register(vx) as u16,
			Ins::LD_F(vx) => todo!(),
			Ins::LD_B(vx) => {
				let hundreds = (vx - (vx % 100)) / 100;
				let tens = ((vx - hundreds * 100) - (vx % 10)) / 10;
				let ones = vx - hundreds  * 100 - tens * 10;
				memory.write_byte(self.i, hundreds);
				memory.write_byte(self.i + 1, tens);
				memory.write_byte(self.i + 2, ones);
			},
			Ins::LD_I_write(vx) => {
				let mut addr = self.i;
				for reg in 0..=vx {
					memory.write_byte(addr, self.read_register(reg));
					addr += 1;
				}
			},
			Ins::LD_I_read(vx) => {
				let mut addr = self.i;
				for reg in 0..=vx {
					self.write_register(reg, memory.read_byte(addr));
					addr += 1;
				}
			}
		}
	}
}
