#[derive(Debug)]
pub struct Memory {
    bytes: [u8; 4096]
}

impl Memory {
    pub fn new() -> Memory {
        Memory { bytes: [0; 4096] }
    }
    pub fn read_byte(&self, addr: u16) -> u8 {
        self.bytes[addr as usize]
    }
    pub fn write_byte(&mut self, addr: u16, data: u8) {
        self.bytes[addr as usize] = data;
    }
    pub fn print_mem(&self) {
        println!("{:?}", self.bytes);
    }
}

