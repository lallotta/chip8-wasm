pub mod cpu;
mod display;
mod keypad;

use crate::cpu::Cpu;
use crate::display::Display;
use crate::keypad::Keypad;
use wasm_bindgen::prelude::*;

static mut CPU: Cpu = Cpu {
    memory: [0; 4096],
    v: [0; 16],
    i: 0,
    pc: 0,
    stack: [0; 16],
    sp: 0,
    display: Display {
        gfx: [0; 2048]
    },
    keypad: Keypad {
        keys: [false; 16]
    },
    draw_flag: false,
    dt: 0,
    st: 0,
};

#[wasm_bindgen]
pub fn emulate_cycle() {
    unsafe {
        CPU.emulate_cycle();
    }
}

#[wasm_bindgen]
pub fn reset() {
    unsafe {
        CPU.reset();
    }
}

#[wasm_bindgen]
pub fn get_memory() -> *const u8 {
    unsafe {
        CPU.memory.as_ptr()
    }
}

#[wasm_bindgen]
pub fn get_display() -> *const u8 {
    unsafe {
        CPU.display.gfx.as_ptr()
    }
}

#[wasm_bindgen]
pub fn draw_pending() -> bool {
    unsafe {
        CPU.draw_pending()
    }
}

#[wasm_bindgen]
pub fn unset_draw_flag() {
    unsafe {
        CPU.unset_draw_flag();
    }
}

#[wasm_bindgen]
pub fn key_down(key: u8) {
    unsafe {
        CPU.keypad.key_down(key);
    }
}

#[wasm_bindgen]
pub fn key_up(key: u8) {
    unsafe {
        CPU.keypad.key_up(key);
    }
}