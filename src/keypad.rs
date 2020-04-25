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

#[cfg(test)]
mod tests {
    use super::Keypad;

    #[test]
    fn test_is_pressed() {
        let mut keypad = Keypad::new();
        let key = 0xF;
        keypad.keys[key as usize] = true;

        assert!(keypad.is_pressed(key));
        assert!(!keypad.is_pressed(0));
    }

    #[test]
    fn test_key_down() {
        let mut keypad = Keypad::new();
        let key = 0xF;
        keypad.key_down(key);

        assert!(keypad.keys[key as usize]);
    }

    #[test]
    fn test_key_up() {
        let mut keypad = Keypad::new();
        let key = 0xF;
        keypad.keys[key as usize] = true;
        keypad.key_up(key);

        assert!(!keypad.keys[key as usize]);
    }
}