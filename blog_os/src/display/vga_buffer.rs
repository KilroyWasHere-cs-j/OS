#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Color {
    Black = 0,
    Blue = 1,
    Green = 2,
    Cyan = 3,
    Red = 4,
    Magenta = 5,
    Brown = 6,
    LightGray = 7,
    DarkGray = 8,
    LightBlue = 9,
    LightGreen = 10,
    LightCyan = 11,
    LightRed = 12,
    Pink = 13,
    Yellow = 14,
    White = 15,
}


pub fn print(text: &[u8], color: Color){
    let vga_buffer = 0xb8000 as *mut u8;

    for (i, &byte) in text.iter().enumerate() {
        unsafe {
            *vga_buffer.offset(i as isize * 2) = byte; // Text to print
            *vga_buffer.offset(i as isize * 2 + 1) = color as u8; // Foregound of text
        }
    }
}