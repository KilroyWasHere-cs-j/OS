/*
This is a basic implementation of a command prompt

Author: Gabriel Tower
Written: 2/07/2024


Kilroy Was Here

*/

#[path = "../kernel/mod.rs"]
mod kernel;

use alloc::string::String;

use self::kernel::{display::{print, println}, keyboard::{KeyboardHandler, KEYBOARD}};


pub fn process_prmt(){
    // Need a better way to do this
    let prompt = KEYBOARD.lock().revel_text();
    let s: String = prompt.into_iter().collect();
    match s.as_str(){
        "help" => {
            println("This is the help menu");
            println("help - displays this menu");
            println("clear - clears the screen");
            println("exit - exits the command prompt");
        },
        "clear" => {
            for _ in 0..100{
                println("");
            }
        },
        "exit" => {
            println("Exiting command prompt");
            return;
        },
        _ => {
            // Do nothing
            // As doing something would cause to many issues
            print(&s);
        }
    }
}
