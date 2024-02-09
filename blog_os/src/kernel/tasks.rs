use alloc::{string::String, vec::Vec};

use super::{display::{print, print_s, println}, keyboard::{KeyboardHandler, KEYBOARD}};

pub fn cmd_task(){
    let input = KEYBOARD.lock().revel_text().iter().collect::<String>();
    let mut input = input.split_whitespace();
    let command = input.next().unwrap();
    let args = input.collect::<Vec<&str>>();
    match command {
        "echo" => {
            println(&args.join(" "));
        }
        "prco" => {
            println("prco is a recursive acronym for 'prco recursively calls itself'");
        }
        "help" => {
            println("prco - a simple shell");
            println("Commands:");
            println("echo - print a string");
            println("prco - print a recursive acronym");
            println("help - print this help message");
        }
        _ => {
            // Do nothing
        }
    }
}