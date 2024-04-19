use alloc::string::String;

use super::{
    display::{self, println},
    keyboard::{KeyboardHandler, KEYBOARD},
};

pub fn cmd_task() {
    let input = KEYBOARD.lock().revel_text().iter().collect::<String>();
    let command = input.split(" ").next().unwrap_or("");
    if !command.is_empty() {
        match command {
            "cl" => display::reset_screen(),
            _ => (),
        }
    }
}

pub fn dummy_task() {
    println("Dummy");
}
