#[allow(non_camel_case_types)]
#[derive(Debug)]
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
    pub fn from_opcode(opcode: u16) -> Option<Self> {
        let first_nibble: u8 = ((opcode & 0xF000) >> 12) as u8;
        let last_nibble: u8 = (opcode & 0x000F) as u8;
        let addr: u16 = opcode & 0x0FFF;
        let vx: u8 = ((opcode & 0x0F00) >> 8) as u8;
        let vy: u8 = ((opcode & 0x00F0) >> 4) as u8;
        let byte: u8 = (opcode & 0x00FF) as u8;

        match first_nibble {
            0x0 => match byte {
                0xE0 => Some(Self::CLS),
                0xEE => Some(Self::RET),
                _ => Some(Self::SYS(addr)),
            },
            0x1 => Some(Self::JP(addr)),
            0x2 => Some(Self::CALL(addr)),
            0x3 => Some(Self::SE_byte(vx, byte)),
            0x4 => Some(Self::SNE_byte(vx, byte)),
            0x5 => Some(Self::SE_reg(vx, vy)),
            0x6 => Some(Self::LD_byte(vx, byte)),
            0x7 => Some(Self::ADD_byte(vx, byte)),
            0x8 => match last_nibble {
                0x0 => Some(Self::LD_reg(vx, vy)),
                0x1 => Some(Self::OR(vx, vy)),
                0x2 => Some(Self::AND(vx, vy)),
                0x3 => Some(Self::XOR(vx, vy)),
                0x4 => Some(Self::ADD_reg(vx, vy)),
                0x5 => Some(Self::SUB(vx, vy)),
                0x6 => Some(Self::SHR(vx)),
                0x7 => Some(Self::SUBN(vx, vy)),
                0xE => Some(Self::SHL(vx)),
                _ => None,
            },
            0x9 => Some(Self::SNE_reg(vx, vy)),
            0xA => Some(Self::LD_I(addr)),
            0xB => Some(Self::JP_V0(addr)),
            0xC => Some(Self::RND(vx, byte)),
            0xD => Some(Self::DRW(vx, vy, last_nibble)),
            0xE => match byte {
                0x9E => Some(Self::SKP(vx)),
                0xA1 => Some(Self::SKNP(vx)),
                _ => None,
            },
            0xF => match byte {
                0x07 => Some(Self::LD_reg_from_DT(vx)),
                0x0A => Some(Self::LD_reg_from_key(vx)),
                0x15 => Some(Self::LD_DT_from_reg(vx)),
                0x18 => Some(Self::LD_ST_from_reg(vx)),
                0x1E => Some(Self::ADD_I(vx)),
                0x29 => Some(Self::LD_F(vx)),
                0x33 => Some(Self::LD_B(vx)),
                0x55 => Some(Self::LD_I_write(vx)),
                0x65 => Some(Self::LD_I_read(vx)),
                _ => None,
            },
            _ => None,
        }
    }
}
