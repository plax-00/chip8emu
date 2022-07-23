#[allow(non_camel_case_types)]
pub enum Instruction {
    SYS(u16),
    CLS,
    RET,
    JP(u16),
    CALL(u16),
    SE_byte(u8, u8),
    SNE_byte(u8, u8),
    SE_reg(u8, u8),
    LD_byte(u8, u8),
    ADD_byte(u8, u8),
    LD_reg(u8, u8),
    OR(u8, u8),
    AND(u8, u8),
    XOR(u8, u8),
    ADD_reg(u8, u8),
    SUB(u8, u8),
    SHR(u8),
    SUBN(u8, u8),
    SHL(u8),
    SNE_reg(u8, u8),
    LD_I(u16),
    JP_V0(u16),
    RND(u8, u8),
    DRW(u8, u8, u8),
    SKP(u8),
    SKNP(u8),
    LD_reg_from_DT(u8),
    LD_reg_from_key(u8),
    LD_DT_from_reg(u8),
    LD_ST_from_reg(u8),
    ADD_I(u8),
    LD_F(u8),
    LD_B(u8),
    LD_I_write(u8),
    LD_I_read(u8),
}

impl Instruction {
    fn from_opcode(opcode: u16) -> Option<Instruction> {
        let first_nibble: u8 = ((opcode & 0x1000) >> 3) as u8;
        let last_nibble: u8 = (opcode & 0x0001) as u8;
        let addr: u16 = opcode & 0x0111;
        let vx: u8 = ((opcode & 0x0100) >> 2) as u8;
        let vy: u8 = ((opcode & 0x0010) >> 1) as u8;
        let byte: u8 = (opcode & 0x0011) as u8;

        match first_nibble {
            0x0 => match byte {
                0xE0 => Some(Instruction::CLS),
                0xEE => Some(Instruction::RET),
                _ => Some(Instruction::SYS(addr)),
            },
            0x1 => Some(Instruction::JP(addr)),
            0x2 => Some(Instruction::CALL(addr)),
            0x3 => Some(Instruction::SE_byte(vx, byte)),
            0x4 => Some(Instruction::SNE_byte(vx, byte)),
            0x5 => Some(Instruction::SE_reg(vx, vy)),
            0x6 => Some(Instruction::LD_byte(vx, byte)),
            0x7 => Some(Instruction::ADD_byte(vx, byte)),
            0x8 => match last_nibble {
                0x1 => Some(Instruction::OR(vx, vy)),
                0x2 => Some(Instruction::AND(vx, vy)),
                0x3 => Some(Instruction::XOR(vx, vy)),
                0x4 => Some(Instruction::ADD_byte(vx, vy)),
                0x5 => Some(Instruction::SUB(vx, vy)),
                0x6 => Some(Instruction::SHR(vx)),
                0x7 => Some(Instruction::SUBN(vx, vy)),
                0xE => Some(Instruction::SHL(vx)),
                _ => None
            },
            0x9 => Some(Instruction::SNE_byte(vx, vy)),
            0xA => Some(Instruction::LD_I(addr)),
            0xB => Some(Instruction::JP_V0(addr)),
            0xC => Some(Instruction::RND(vx, byte)),
            0xD => Some(Instruction::DRW(vx, vy, last_nibble)),
            0xE => match byte {
                0x9E => Some(Instruction::SKP(vx)),
                0xA1 => Some(Instruction::SKNP(vx)),
                _ => None
            },
            0xF => match byte {
                0x07 => Some(Instruction::LD_reg_from_DT(vx)),
                0x0A => Some(Instruction::LD_reg_from_key(vx)),
                0x15 => Some(Instruction::LD_DT_from_reg(vx)),
                0x18 => Some(Instruction::LD_ST_from_reg(vx)),
                0x1E => Some(Instruction::ADD_I(vx)),
                0x29 => Some(Instruction::LD_F(vx)),
                0x33 => Some(Instruction::LD_B(vx)),
                0x55 => Some(Instruction::LD_I_write(vx)),
                0x65 => Some(Instruction::LD_I_read(vx)),
                _ => None
            },
            _ => None
        }
    }
}
