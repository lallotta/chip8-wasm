use crate::display::{Display, FONT_SET};
use crate::keypad::Keypad;

pub struct Cpu {
    pub memory: [u8; 4096],
    pub v: [u8; 16],
    pub i: u16,
    // program counter
    pub pc: u16,
    pub stack: [u16; 16],
    // stack pointer
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

    fn execute_opcode(&mut self, opcode: u16) {
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
            _ => panic!("unknown opcode: {:#x}", opcode)
        }
    }

    fn update_timers(&mut self) {
        if self.dt > 0 {
            self.dt -= 1;
        }
        if self.st > 0 {
            self.st -= 1;
        }
    }

    pub fn emulate_cycle(&mut self) {
        let opcode = self.read_opcode();
        self.execute_opcode(opcode);
        self.update_timers();
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

    pub fn unset_draw_flag(&mut self) {
        self.draw_flag = false;
    }

    pub fn draw_pending(&self) -> bool {
        self.draw_flag
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
        self.v[x] = self.v[x].wrapping_add(kk);
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
        self.v[0xF] = self.v[x] & 1;
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
        self.v[x] = (js_sys::Math::random() * 256f64) as u8 & kk;
    }

    fn op_dxyn(&mut self, x: usize, y: usize, n: usize) {
        let col = self.v[x] as usize;
        let row = self.v[y] as usize;
        let sprite = &self.memory[self.i as usize..self.i as usize + n];

        let collision = self.display.draw_sprite(col, row, sprite);
        self.v[0xF] = if collision { 1 } else { 0 };

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
        for (i, pressed) in (0u8..).zip(&self.keypad.keys) {
            if *pressed {
                self.v[x] = i;
                return;
            }
        }
        self.pc -= 2;
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
        let start = self.i as usize;
        self.memory[start..=start+x].copy_from_slice(&self.v[0..=x]);
    }

    fn op_fx65(&mut self, x: usize) {
        let start = self.i as usize;
        self.v[0..=x].copy_from_slice(&self.memory[start..=start+x]);
    }
}

#[cfg(test)]
mod tests {
    use super::Cpu;

    #[test]
    fn test_op_00e0() {
        let mut cpu = Cpu::new();
        cpu.display.gfx[1] = 1;
        cpu.display.gfx[2] = 1;

        cpu.execute_opcode(0x00E0);
        assert_eq!(cpu.display.gfx[1], 0);
        assert_eq!(cpu.display.gfx[2], 0);
        assert!(cpu.draw_flag);
    }

    #[test]
    fn test_op_00ee() {
        let mut cpu = Cpu::new();
        let addr = 0x220;
        cpu.stack[0] = addr;
        cpu.sp = 1;
        cpu.pc = 0x300;

        cpu.execute_opcode(0x00EE);
        assert_eq!(cpu.pc, addr);
        assert_eq!(cpu.sp, 0);
    }

    #[test]
    fn test_op_1nnn() {
        let mut cpu = Cpu::new();
        cpu.pc = 0x220;

        cpu.execute_opcode(0x1210);
        assert_eq!(cpu.pc, 0x210);
    }

    #[test]
    fn test_op_2nnn() {
        let mut cpu = Cpu::new();
        cpu.pc = 0x300;

        cpu.execute_opcode(0x21AF);
        assert_eq!(cpu.stack[0], 0x302);
        assert_eq!(cpu.sp, 1);
        assert_eq!(cpu.pc, 0x1AF);
    }

    #[test]
    fn test_op_3xkk() {
        let mut cpu = Cpu::new();
        let pc = 0x400;
        cpu.v[3] = 5;
        cpu.pc = pc;

        cpu.execute_opcode(0x3306);
        assert_eq!(cpu.pc, pc + 2);
        
        cpu.execute_opcode(0x3305);
        assert_eq!(cpu.pc, pc + 6);
    }

    #[test]
    fn test_op_4xkk() {
       let mut cpu = Cpu::new();
        let pc = 0x400;
        cpu.v[3] = 5;
        cpu.pc = pc;

        cpu.execute_opcode(0x4305);
        assert_eq!(cpu.pc, pc + 2);
        
        cpu.execute_opcode(0x4306);
        assert_eq!(cpu.pc, pc + 6);
    }

    #[test]
    fn test_op_5xy0() {
        let mut cpu = Cpu::new();
        let pc = 0x2AF;
        cpu.v[2] = 30;
        cpu.v[3] = 40;
        cpu.pc = pc;
        
        cpu.execute_opcode(0x5230);
        assert_eq!(cpu.pc, pc + 2);

        cpu.v[2] = 40;

        cpu.execute_opcode(0x5230);
        assert_eq!(cpu.pc, pc + 6);
    }

    #[test]
    fn test_op_6xkk() {
        let mut cpu = Cpu::new();
        cpu.v[4] = 10;

        cpu.execute_opcode(0x6455);
        assert_eq!(cpu.v[4], 0x55);   
    }
}
