pub struct Keypad {
    pub keys: [bool; 16],
}

impl Keypad {
    pub fn new() -> Keypad {
        Keypad {
            keys: [false; 16]
        }
    }

    pub fn is_pressed(&self, key: u8) -> bool {
        self.keys[key as usize]
    }

    pub fn key_down(&mut self, key: u8) {
        self.keys[key as usize] = true;
    }

    pub fn key_up(&mut self, key: u8) {
        self.keys[key as usize] = false;
    }
}