pub struct Display {
    pub gfx: [u8; 2048],
}

const WIDTH: usize = 64;
#[allow(dead_code)]
const HEIGHT: usize = 32;

impl Display {
    pub fn new() -> Display {
        Display {
            gfx: [0; 2048]
        }
    }

    fn get_index(&self, row: usize, column: usize) -> usize {
        row * WIDTH + column
    }

    pub fn draw_sprite(&mut self, x: usize, y: usize, sprite: &[u8]) -> bool {
        let mut collision = false;
        for i in 0..sprite.len() {
            for j in 0..8 {
                let row = y + i;
                let column = x + j;
                if sprite[i] & (0x80 >> j) != 0 && row <= HEIGHT && column <= WIDTH {
                    let idx = self.get_index(row, column);
                    
                    if self.gfx[idx] == 1 {
                        collision = true;
                    }

                    self.gfx[idx] ^= 1;
                    // if idx < self.gfx.len() {
                    // }
                }
            }
        }
        collision
    }

    pub fn clear(&mut self) {
        for i in 0..self.gfx.len() {    
            self.gfx[i] = 0;
        }
    }
}

pub const FONT_SET: [u8; 80] = [
    0xF0, 0x90, 0x90, 0x90, 0xF0,
    0x20, 0x60, 0x20, 0x20, 0x70,
    0xF0, 0x10, 0xF0, 0x80, 0xF0,
    0xF0, 0x10, 0xF0, 0x10, 0xF0,
    0x90, 0x90, 0xF0, 0x10, 0x10,
    0xF0, 0x80, 0xF0, 0x10, 0xF0,
    0xF0, 0x80, 0xF0, 0x90, 0xF0,
    0xF0, 0x10, 0x20, 0x40, 0x40,
    0xF0, 0x90, 0xF0, 0x90, 0xF0,
    0xF0, 0x90, 0xF0, 0x10, 0xF0,
    0xF0, 0x90, 0xF0, 0x90, 0x90,
    0xE0, 0x90, 0xE0, 0x90, 0xE0,
    0xF0, 0x80, 0x80, 0x80, 0xF0,
    0xE0, 0x90, 0x90, 0x90, 0xE0,
    0xF0, 0x80, 0xF0, 0x80, 0xF0,
    0xF0, 0x80, 0xF0, 0x80, 0x80
];
