use crate::display::{Display, FONT_SET, WIDTH, HEIGHT};
use crate::keypad::Keypad;
use js_sys::Math;

pub struct Cpu {
    pub memory: [u8; 4096],
    pub v: [u8; 16],
    pub i: u16,
    pub pc: u16,
    pub stack: [u16; 16],
    pub sp: u8,
    pub display: Display,
    pub keypad: Keypad,
    pub draw_flag: bool,
    // delay timer
    pub dt: u8,
    //sound timer
    pub st: u8,
}

impl Cpu {
    pub fn new() -> Cpu {
        Cpu {
            memory: [0; 4096],
            v: [0; 16],
            i: 0,
            pc: 0,
            stack: [0; 16],
            sp: 0,
            display: Display::new(),
            keypad: Keypad::new(),
            draw_flag: false,
            dt: 0,
            st: 0,
        }
    }

    fn read_opcode(&self) -> u16 {
        (self.memory[self.pc as usize] as u16) << 8 |
            self.memory[self.pc as usize + 1] as u16
    }

    pub fn emulate_cycle(&mut self) {
        let opcode = self.read_opcode();

        let nnn = opcode & 0x0FFF;
        let kk = (opcode & 0x00FF) as u8;
        let n = (opcode & 0x000F) as usize;
        let x = ((opcode & 0x0F00) >> 8) as usize;
        let y = ((opcode & 0x00F0) >> 4) as usize;

        let nibbles = (
            opcode >> 12,
            (opcode & 0x0F00) >> 8,
            (opcode & 0x00F0) >> 4,
            opcode & 0x000F
        );

        self.pc += 2;

        match nibbles {
            (0, 0, 0xE, 0) => self.op_00e0(),
            (0, 0, 0xE, 0xE) => self.op_00ee(),
            (0x1, _, _, _) => self.op_1nnn(nnn),
            (0x2, _, _, _) => self.op_2nnn(nnn),
            (0x3, _, _, _) => self.op_3xkk(x, kk),
            (0x4, _, _, _) => self.op_4xkk(x, kk),
            (0x5, _, _, 0) => self.op_5xy0(x, y),
            (0x6, _, _, _) => self.op_6xkk(x, kk),
            (0x7, _, _, _) => self.op_7xkk(x, kk),
            (0x8, _, _, 0) => self.op_8xy0(x, y),
            (0x8, _, _, 0x1) => self.op_8xy1(x, y),
            (0x8, _, _, 0x2) => self.op_8xy2(x, y),
            (0x8, _, _, 0x3) => self.op_8xy3(x, y),
            (0x8, _, _, 0x4) => self.op_8xy4(x, y),
            (0x8, _, _, 0x5) => self.op_8xy5(x, y),
            (0x8, _, _, 0x6) => self.op_8xy6(x),
            (0x8, _, _, 0x7) => self.op_8xy7(x, y),
            (0x8, _, _, 0xE) => self.op_8xye(x),
            (0x9, _, _, 0) => self.op_9xy0(x, y),
            (0xA, _, _, _) => self.op_annn(nnn),
            (0xB, _, _, _) => self.op_bnnn(nnn),
            (0xC, _, _, _) => self.op_cxkk(x, kk),
            (0xD, _, _, _) => self.op_dxyn(x, y, n),
            (0xE, _, 0x9, 0xE) => self.op_ex9e(x),
            (0xE, _, 0xA, 0x1) => self.op_exa1(x),
            (0xF, _, 0, 0x7) => self.op_fx07(x),
            (0xF, _, 0, 0xA) => self.op_fx0a(x),
            (0xF, _, 0x1, 0x5) => self.op_fx15(x),
            (0xF, _, 0x1, 0x8) => self.op_fx18(x),
            (0xF, _, 0x1, 0xE) => self.op_fx1e(x),
            (0xF, _, 0x2, 0x9) => self.op_fx29(x), 
            (0xF, _, 0x3, 0x3) => self.op_fx33(x),
            (0xF, _, 0x5, 0x5) => self.op_fx55(x),
            (0xF, _, 0x6, 0x5) => self.op_fx65(x),
            _ => {}
        }

        if self.dt > 0 {
            self.dt -= 1;
        }
        if self.st > 0 {
            self.st -= 1;
        }
    }

    pub fn reset(&mut self) {
        self.memory = [0; 4096];
        self.v = [0; 16];
        self.i = 0;
        self.pc = 0x200;
        self.stack = [0; 16];
        self.sp = 0;
        self.display.clear();
        self.draw_flag = false;

        for i in 0..FONT_SET.len() {
            self.memory[i] = FONT_SET[i];
        }
    }

    fn op_00e0(&mut self) {
        self.display.clear();
        self.draw_flag = true;
    }

    fn op_00ee(&mut self) {
        self.sp -= 1;
        self.pc = self.stack[self.sp as usize];
    }

    fn op_1nnn(&mut self, nnn: u16) {
        self.pc = nnn;
    }

    fn op_2nnn(&mut self, nnn: u16) {
        self.stack[self.sp as usize] = self.pc;
        self.sp += 1;
        self.pc = nnn;
    }

    fn op_3xkk(&mut self, x: usize, kk: u8) {
        if self.v[x] == kk {
            self.pc += 2;
        }
    }

    fn op_4xkk(&mut self, x: usize, kk: u8) {
        if self.v[x] != kk {
            self.pc += 2;
        }
    }

    fn op_5xy0(&mut self, x: usize, y: usize) {
        if self.v[x] == self.v[y] {
            self.pc += 2;
        }
    }

    fn op_6xkk(&mut self, x: usize, kk: u8) {
        self.v[x] = kk;
    }

    fn op_7xkk(&mut self, x: usize, kk: u8) {
        let (sum, _) = self.v[x].overflowing_add(kk);
        self.v[x] = sum;
    }

    fn op_8xy0(&mut self, x: usize, y: usize) {
        self.v[x] = self.v[y];
    }

    fn op_8xy1(&mut self, x: usize, y: usize) {
        self.v[x] |= self.v[y];
    }

    fn op_8xy2(&mut self, x: usize, y: usize) {
        self.v[x] &= self.v[y];
    }

    fn op_8xy3(&mut self, x: usize, y: usize) {
        self.v[x] ^= self.v[y];
    }

    fn op_8xy4(&mut self, x: usize, y: usize) {
        let (sum, overflow) = self.v[x].overflowing_add(self.v[y]);
        self.v[0xF] = if overflow { 1 } else { 0 };
        self.v[x] = sum;
    }

    fn op_8xy5(&mut self, x: usize, y: usize) {
        let (res, overflow) = self.v[x].overflowing_sub(self.v[y]);
        self.v[0xF] = if overflow { 0 } else { 1 };
        self.v[x] = res;
    }

    fn op_8xy6(&mut self, x: usize) {
        self.v[0xF] = self.v[x] & 1u8;
        self.v[x] >>= 1;
    }

    fn op_8xy7(&mut self, x: usize, y: usize) {
        let (res, overflow) = self.v[y].overflowing_sub(self.v[x]);
        self.v[0xF] = if overflow { 0 } else { 1 };
        self.v[x] = res;
    }

    fn op_8xye(&mut self, x: usize) {
        self.v[0xF] = self.v[x] & 0x80;
        self.v[x] <<= 1;
    }

    fn op_9xy0(&mut self, x: usize, y: usize) {
        if self.v[x] != self.v[y] {
            self.pc += 2;
        }
    }

    fn op_annn(&mut self, nnn: u16) {
        self.i = nnn;
    }

    fn op_bnnn(&mut self, nnn: u16) {
        self.pc = self.v[0] as u16 + nnn;
    }

    fn op_cxkk(&mut self, x: usize, kk: u8) {
        self.v[x] = (Math::random() * 256 as f64) as u8 & kk;
    }

    fn op_dxyn(&mut self, x: usize, y: usize, n: usize) {
        let xpos = self.v[x] as usize;
        let ypos = self.v[y] as usize;

        self.v[0xF] = 0;

        for i in 0..n {
            let px = self.memory[self.i as usize + i];

            for j in 0..8 {
                if px & (0x80 >> j) != 0 {
                    let xj = (xpos + j) % WIDTH;
                    let yi = (ypos + i) % HEIGHT;
                    let idx = self.display.get_index(xj, yi);

                    if self.display.gfx[idx] == 1 {
                        self.v[0xF] = 1;
                    }

                    self.display.gfx[idx] ^= 1;
                }
            }
        }

        self.draw_flag = true;
    }

    fn op_ex9e(&mut self, x: usize) {
        if self.keypad.is_pressed(self.v[x]) {
            self.pc += 2;
        }
    }

    fn op_exa1(&mut self, x: usize) {
        if !self.keypad.is_pressed(self.v[x]) {
            self.pc += 2;
        }
    }

    fn op_fx07(&mut self, x: usize) {
        self.v[x] = self.dt;
    }

    fn op_fx0a(&mut self, x: usize) {
        self.pc -= 2;
        
        for (i, state) in self.keypad.keys.iter().enumerate() {
            if *state {
                self.v[x] = i as u8;
                self.pc += 2;
            }
        }
    }

    fn op_fx15(&mut self, x: usize) {
        self.dt = self.v[x];
    }

    fn op_fx18(&mut self, x: usize) {
        self.st = self.v[x];
    }

    fn op_fx1e(&mut self, x: usize) {
        self.i += self.v[x] as u16;
    }

    fn op_fx29(&mut self, x: usize) {
        self.i = self.v[x] as u16 * 5;
    }

    fn op_fx33(&mut self, x: usize) {
        self.memory[self.i as usize] = self.v[x] / 100;
        self.memory[self.i as usize + 1] = (self.v[x] / 10) % 10;
        self.memory[self.i as usize + 2] = self.v[x] % 10;
    }

    fn op_fx55(&mut self, x: usize) {
        for i in 0..=x {
            self.memory[self.i as usize + i] = self.v[i];
        }
    }

    fn op_fx65(&mut self, x: usize) {
        for i in 0..=x {
            self.v[i] = self.memory[self.i as usize + i];
        }
    }
}