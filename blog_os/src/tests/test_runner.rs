use crate::{print, println, display::vga_buffer::{self, BUFFER_HEIGHT}};

/// This module contains the test runner, which executes all tests and prints their results.

/// Quick test to check if the test runner works.
fn print_test(){
    print!("Running print test ... ");
    assert_eq!(1, 1);
    println!("[ok]");
}

pub fn vga_buffer_quick_test(){
    print!("Running VGA buffer test ...");
    vga_buffer::test_print();
    println!("[ok]");
}

pub fn buffer_size_check(){
    print!("Running VGA buffer size check ...");
    if BUFFER_HEIGHT != 25 && BUFFER_HEIGHT != 50{
        panic!("VGA buffer dimensions are not correct");
    }
    println!("[ok]");
}

pub fn buffer_cursor_postion_check(){
    print!("Running VGA buffer cursor position check ...");
    if vga_buffer::cursor_position_test(10) == false{
        panic!("VGA buffer cursor cannot be moved");
    }
    println!("[ok]");
}

pub fn buffer_advance_column_check(){
    print!("Running VGA buffer advance column check ...");
    if vga_buffer::advance_column_test(10) == false{
        panic!("VGA buffer cursor cannot be moved");
    }
    println!("[ok]");
}

/// Executes all tests.
pub fn run_all_tests(){
    print_test();
    vga_buffer_quick_test();
    buffer_size_check();
    buffer_cursor_postion_check();
    buffer_advance_column_check();
    print!("All VGA buffer tests passed!");
    vga_buffer::advance_column_test(5);
}