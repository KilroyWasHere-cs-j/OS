use alloc::{sync::Arc, vec::Vec};
use lazy_static::lazy_static;
use spin::Mutex;

use super::display::println;

lazy_static! {
    pub static ref KEYBOARD: Arc<Mutex<Keyboard>> = Arc::new(Mutex::new(Keyboard::new()));
}

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
    fn new() -> Self {
        Keyboard {
            text_buffer: Vec::new(),
            caps_lock: false,
            shift: false,
        }
    }

    fn on_key(&mut self, key: char) {
        // ! TODO: Backspace doesn't work
        // println!("{:?}", key);
        if self.caps_lock {
            self.text_buffer.push(key.to_uppercase().next().unwrap());
        } else {
            self.text_buffer.push(key);
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
