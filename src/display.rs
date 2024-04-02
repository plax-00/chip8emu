#[derive(Debug)]
pub struct Display {
    width: u32,
    height: u32,
    pub pixels: Vec<bool>,
}

impl Display {
    pub fn new() -> Self {
        let width: u32 = 64;
        let height: u32 = 32;
        let pixels = (0..width * height).map(|_| false).collect();

        Self {
            width,
            height,
            pixels,
        }
    }

    pub fn get_ptr(&self) -> *const bool {
        self.pixels.as_ptr()
    }

    pub fn clear(&mut self) {
        self.pixels = (0..self.width * self.height).map(|_| false).collect();
        for pixel in &mut self.pixels {
            *pixel = false;
        }
    }

    fn get_index(&self, x: u8, y: u8) -> usize {
        let width = self.width;
        let height = self.height;
        let adjusted_x = x as u32 % width;
        let adjusted_y = y as u32 % height;
        (adjusted_x + adjusted_y * width) as usize
    }

    pub fn draw(&mut self, start_x: u8, start_y: u8, sprite: &[u8]) -> u8 {
        let mut collision = false;

        for i in 0..sprite.len() {
            let byte = byte_as_bool_array(sprite[i]);
            let y = start_y + i as u8;

            for j in 0..8 as usize {
                let x = start_x + j as u8;
                let pixel_idx = self.get_index(x, y);
                let new_pixel = self.pixels[pixel_idx] ^ byte[j];
                if self.pixels[pixel_idx] && !new_pixel {
                    collision = true;
                }
                self.pixels[pixel_idx] = new_pixel;
            }
        }

        if collision {
            1
        } else {
            0
        }
    }
}

fn byte_as_bool_array(byte: u8) -> [bool; 8] {
    let mut bool_array = [false; 8];
    for i in 0..8 {
        bool_array[i] = (byte & (0x80 >> i)) >> (8 - i - 1) == 1;
    }
    bool_array
}

#[cfg(test)]
mod tests {
    use crate::display::byte_as_bool_array;

    #[test]
    fn byte_as_bool_array_test() {
        assert_eq!(
            byte_as_bool_array(0b10110110),
            [true, false, true, true, false, true, true, false]
        );
    }
}
