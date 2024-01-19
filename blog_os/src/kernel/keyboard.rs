use alloc::vec::Vec;

pub struct Keyboard {
    text_buffer: Vec<char>,
    caps_lock: bool,
    shift: bool,
}

pub trait KeyboardHandler {
    fn new() -> Self;
    fn on_key(&mut self, key: char);
    fn revel_text(&self) -> Vec<char>;
    fn flush(&mut self);
    fn set_caps_lock(&mut self, state: bool);
    fn set_shift(&mut self, state: bool);
}

impl KeyboardHandler for Keyboard {
    fn on_key(&mut self, key: char) {
        // ! TODO: Backspace doesn't work
        if key == '\u{8}' {
            self.text_buffer.pop();
        } else {
            self.text_buffer.push(key);
        }
    }

    fn new() -> Self {
        Keyboard {
            text_buffer: Vec::new(),
            caps_lock: false,
            shift: false,
        }
    }

    fn revel_text(&self) -> Vec<char> {
        self.text_buffer.clone()
    }

    fn flush(&mut self) {
        self.text_buffer.clear();
    }

    fn set_caps_lock(&mut self, state: bool) {
        self.caps_lock = state;
    }

    fn set_shift(&mut self, state: bool) {
        self.shift = state;
    }
}
