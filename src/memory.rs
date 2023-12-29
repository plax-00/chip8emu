#[derive(Debug)]
pub struct Memory {
	bytes: [u8; 4096]
}

impl Memory {
	pub fn new() -> Memory {
		let mut memory = Memory { bytes: [0; 4096] };

		// Initialize the first 80 bytes (0x00 to 0x4F) with font sprite data
		let font_sprites: [u8; 80] = [
			0xF0, 0x90, 0x90, 0x90, 0xF0,	// 0
			0x20, 0x60, 0x20, 0x20, 0x70,	// 1
			0xF0, 0x10, 0xF0, 0x80, 0xF0,	// 2
			0xF0, 0x10, 0xF0, 0x10, 0xF0,	// 3
			0x90 ,0x90 ,0xF0 ,0x10 ,0x10,	// 4
			0xF0 ,0x80 ,0xF0 ,0x10 ,0xF0,	// 5
			0xF0, 0x80, 0xF0, 0x90, 0xF0,	// 6
			0xF0, 0x10, 0x20, 0x40, 0x40,	// 7
			0xF0, 0x90, 0xF0, 0x90, 0xF0,	// 8
			0xF0, 0x90, 0xF0, 0x10, 0xF0,	// 9
			0xF0, 0x90, 0xF0, 0x90, 0x90,	// A
			0xE0, 0x90, 0xE0, 0x90, 0xE0,	// B
			0xF0, 0x80, 0x80, 0x80, 0xF0,	// C
			0xE0, 0x90, 0x90, 0x90, 0xE0,	// D
			0xF0, 0x80, 0xF0, 0x80, 0xF0,	// E
			0xF0, 0x80, 0xF0, 0x80, 0x80,	// F
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

	pub fn print_mem(&self) {
		println!("{:?}", self.bytes);
	}
}

