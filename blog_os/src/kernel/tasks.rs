use alloc::string::String;

use crate::kernel;

use super::{
    display::print,
    display::println,
    keyboard::{KeyboardHandler, KEYBOARD},
};

pub fn cmd_task() {
    let input = KEYBOARD.lock().revel_text().iter().collect::<String>();
    let command = input.split(" ").next().unwrap_or("");
    if command == "echo" {
        print("e");
    }
    if command == "c" {
        kernel::display::reset_screen();
    }
}

pub fn dummy_task() {
    println("Dummy");
}
