use crate::chip8::PROGRAM_START;

#[derive(Debug)]
pub struct Memory {
    bytes: [u8; 4096],
}

impl Memory {
    pub fn new() -> Self {
        let mut memory = Self { bytes: [0; 4096] };

        // Initialize the first 80 bytes (0x00 to 0x4F) with font sprite data
        let font_sprites: [u8; 80] = [
            0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
            0x20, 0x60, 0x20, 0x20, 0x70, // 1
            0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
            0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
            0x90, 0x90, 0xF0, 0x10, 0x10, // 4
            0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
            0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
            0xF0, 0x10, 0x20, 0x40, 0x40, // 7
            0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
            0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
            0xF0, 0x90, 0xF0, 0x90, 0x90, // A
            0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
            0xF0, 0x80, 0x80, 0x80, 0xF0, // C
            0xE0, 0x90, 0x90, 0x90, 0xE0, // D
            0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
            0xF0, 0x80, 0xF0, 0x80, 0x80, // F
        ];
        for byte in 0..80 {
            memory.write_byte(byte, font_sprites[byte as usize])
        }

        memory
    }

    pub fn read_byte(&self, addr: u16) -> u8 {
        self.bytes[addr as usize]
    }

    pub fn write_byte(&mut self, addr: u16, data: u8) {
        self.bytes[addr as usize] = data;
    }

    pub fn get_font_sprite_addr(&self, sprite: u8) -> u16 {
        sprite as u16 * 5
    }

    pub fn print_mem(&self) -> String {
        format!("{:?}", self.bytes)
    }

    pub fn get_sprite(&self, addr: u16, length: u8) -> &[u8] {
        let start = addr as usize;
        let end = (addr + length as u16) as usize;
        &self.bytes[start..end]
    }

    pub fn load_rom(&mut self, rom: &Vec<u8>) {
        for i in 0..rom.len() {
            let addr = PROGRAM_START + i as u16;
            let data = rom[i];
            self.write_byte(addr, data);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Memory;

    #[test]
    fn read_write_test() {
        let mut memory = Memory::new();
        memory.write_byte(0x200, 200);
        assert_eq!(memory.read_byte(0x200), 200);
    }

    #[test]
    fn load_rom_test() {
        let mut memory = Memory::new();
        let mut rom: Vec<u8> = Vec::new();
        rom.push(18);
        memory.load_rom(&rom);
        assert_eq!(memory.read_byte(0x200), 18);
    }

    #[test]
    fn get_sprite_test() {
        let mut memory = Memory::new();
        memory.write_byte(0x200, 200);
        memory.write_byte(0x201, 201);
        memory.write_byte(0x202, 202);
        memory.write_byte(0x203, 203);
        let sprite = memory.get_sprite(0x200, 4);
        assert_eq!(sprite, &[200, 201, 202, 203]);
    }
}
