use std::array;

pub const PIXEL_WIDTH: usize = 64;
pub const PIXEL_HEIGHT: usize = 32;

pub const WIDTH: f32 = PIXEL_WIDTH as f32;
pub const HEIGHT: f32 = PIXEL_HEIGHT as f32;

pub const FONT_START_ADDRESS: u16 = 0x100;
pub const FONT_BYTES_PER_CHARACTER: usize = 5;

pub const FONT: [[u8; FONT_BYTES_PER_CHARACTER]; 16] = [
    [0xF0, 0x90, 0x90, 0x90, 0xF0],
    [0x20, 0x60, 0x20, 0x20, 0x70],
    [0xF0, 0x10, 0xF0, 0x80, 0xF0],
    [0xF0, 0x10, 0xF0, 0x10, 0xF0],
    [0x90, 0x90, 0xF0, 0x10, 0x10],
    [0xF0, 0x80, 0xF0, 0x10, 0xF0],
    [0xF0, 0x80, 0xF0, 0x90, 0xF0],
    [0xF0, 0x10, 0x20, 0x40, 0x40],
    [0xF0, 0x90, 0xF0, 0x90, 0xF0],
    [0xF0, 0x90, 0xF0, 0x10, 0xF0],
    [0xF0, 0x90, 0xF0, 0x90, 0x90],
    [0xE0, 0x90, 0xE0, 0x90, 0xE0],
    [0xF0, 0x80, 0x80, 0x80, 0xF0],
    [0xE0, 0x90, 0x90, 0x90, 0xE0],
    [0xF0, 0x80, 0xF0, 0x80, 0xF0],
    [0xF0, 0x80, 0xF0, 0x80, 0x80],
];

pub struct Display {
    pixels: [[bool; PIXEL_WIDTH]; PIXEL_HEIGHT],
}

impl Display {
    pub fn new() -> Self {
        Self {
            pixels: [[false; PIXEL_WIDTH]; PIXEL_HEIGHT],
        }
    }

    pub fn reset(&mut self) {
        *self = Self::new();
    }
    
    pub fn clear(&mut self) {
        self.pixels.fill([false; PIXEL_WIDTH]);
    }

    pub fn pixels(&self) -> [[bool; PIXEL_WIDTH]; PIXEL_HEIGHT] {
        self.pixels
    }
    
    pub fn draw_sprite(&mut self, x: usize, y: usize, sprite: &[u8]) -> bool {
        let mut updated = false;
        let start_x = x % PIXEL_WIDTH;
        let start_y = y % PIXEL_HEIGHT;

        for (row, sprite_pixel_row) in sprite.iter().enumerate() {
            let current_y = start_y + row;

            if current_y >= PIXEL_HEIGHT {
                return updated;
            }

            match self.pixels.get_mut(current_y) {
                Some(pixel_row) => {
                    for (index, bit) in Self::byte_to_bits(*sprite_pixel_row).iter().enumerate() {
                        let current_x = start_x + index;

                        if current_x >= PIXEL_WIDTH {
                            continue;
                        }

                        match pixel_row.get_mut(current_x) {
                            Some(pixel) => {
                                if *bit {
                                    *pixel = !*pixel;
                                    updated = true;
                                }
                            }
                            None => continue
                        }
                    }
                }
                None => break
            }
        }

        updated
    }

    fn byte_to_bits(byte: u8) -> [bool; 8] {
       array::from_fn(|offset| {
           byte & (0x80 >> offset) != 0
       })
    }
    
}
