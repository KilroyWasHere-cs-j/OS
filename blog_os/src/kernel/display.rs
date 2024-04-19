use alloc::string::ToString;
/// A module for displaying text on the screen
///
/// Author: Gabriel Tower with some help from Josh Kolasa
/// Date: 2020-12-28
/// Kilroy Was Here
use alloc::{string::String, sync::Arc, vec::Vec};
use lazy_static::lazy_static;
use spin::Mutex;

/// The height of the text buffer (normally 25 lines).
const BUFFER_HEIGHT: usize = 25;
/// The width of the text buffer (normally 80 columns).
const BUFFER_WIDTH: usize = 80;
const SPACING: usize = 1; // Space between each character

lazy_static! {
    /// A static writer instance that can be used for printing to the VGA text buffer.
    pub static ref WRITER: Arc<Mutex<Writer>> = Arc::new(Mutex::new(Writer::new()));
}

/// A color code for the VGA text buffer.
pub enum ColorCode {
    White = 0xF,
    Black = 0x0,
    Blue = 0x1,
    Green = 0x2,
    Cyan = 0x3,
    Red = 0x4,
    Magenta = 0x5,
    Brown = 0x6,
    LightGray = 0x7,
    DarkGray = 0x8,
    LightBlue = 0x9,
    LightGreen = 0xA,
    LightCyan = 0xB,
    LightRed = 0xC,
    Pink = 0xD,
    Yellow = 0xE,
}

pub struct Writer {
    /// The column position of the cursor column wise
    column_position: isize,
    /// The row position of the cursor row wise
    row_position: isize,
    /// A vector of characters that are to be printed to the screen
    text_buffer: Vec<char>,
    ///Height of the screen to prevent overrunning
    window_height: isize,
}

pub trait WriterTrait {
    fn new() -> Self;
    fn print(&mut self, printable: String);
    fn new_line(&mut self);
    fn reset_screen(&mut self);
    fn clear_line(&mut self);
    fn write_byte(&mut self, byte: u8);
    fn set_column_position(&mut self, position: isize);
    fn set_row_position(&mut self, position: isize);
}

impl WriterTrait for Writer {
    /// Creates a new writer instance that can be used for printing to the VGA text buffer.
    /// # Example
    /// ```
    /// use crate::kernel::display::WriterTrait;
    /// use crate::kernel::display::Writer;
    /// let mut writer = Writer::new();
    /// ```
    // row_position is set to one to avoid a multiplication by zero error resulting in new line not working the first
    // time it's called
    fn new() -> Self {
        Writer {
            text_buffer: Vec::new(),
            column_position: 0,
            row_position: 1,
            window_height: 10,
        }
    }

    /// Writes to screen
    /// # Arguments
    /// * 'printable' - the string to be printed to the screen
    /// # Notes
    /// * This function is a safe wrapper around a VGA buffer
    /// # Example
    /// ```
    /// use crate::kernel::display::WriterTrait;
    /// use crate::kernel::display::Writer;
    /// let mut writer = Writer::new();
    /// writer.print("Hello World!".to_string());
    /// ```
    fn print(&mut self, printable: String) {
        if self.row_position <= self.window_height {
            // Convert 'printable' to char and load into text_buffer
            for i in printable.chars() {
                self.text_buffer.push(i);
            }

            for i in 0..self.text_buffer.len() {
                self.write_byte(self.text_buffer[i] as u8);
            }
        } else {
            self.reset_screen();
            self.row_position = 0;
        }
        // Although this causes a funny looking glitch it's necessary to clear the text_buffer :(
        self.text_buffer.clear();

        // Facts with Gabe:
        // 1. In 0x the first number is the background color and the second number is the foreground color
        // 2. 0x0 is black and 0xF is white it's hexadecimal :)
    }

    /// Moves the cursor to the next line
    /// # Notes
    /// * This function is a safe wrapper around a VGA buffer
    /// # Example
    /// ```
    /// use crate::kernel::display::WriterTrait;
    /// use crate::kernel::display::Writer;
    /// let mut writer = Writer::new();
    /// writr.new_line();
    /// ```
    // Thanks to Josh Kolasa for this math
    fn new_line(&mut self) {
        let hold = (BUFFER_WIDTH as isize * self.row_position) - self.column_position;
        for _i in 0..hold {
            self.write_byte(b' ');
        }
        self.column_position += 1;
    }

    fn reset_screen(&mut self) {
        let vga_buffer = 0xb8000 as *mut u8;
        for i in 0..BUFFER_WIDTH * BUFFER_HEIGHT {
            unsafe {
                *vga_buffer.offset((i * 2) as isize) = b' ';
                *vga_buffer.offset((i * 2 + 1) as isize) = 0xb;
            }
        }
        self.column_position = 0;
    }

    fn clear_line(&mut self) {
        self.column_position = self.column_position - 1;
        self.write_byte(b' ');
    }

    fn set_column_position(&mut self, position: isize) {
        self.column_position = position;
    }

    fn set_row_position(&mut self, position: isize) {
        self.row_position = position;
    }

    /// Writes a byte to the VGA buffer
    /// # Arguments
    /// * 'byte' - the byte to be written to the VGA buffer
    /// # Notes
    /// * This function is unsafe because it uses the VGA buffer
    /// # Example
    /// ```
    /// use crate::kernel::display::WriterTrait;
    /// use crate::kernel::display::Writer;
    /// let mut writer = Writer::new();
    /// writer.write_byte(b' ');
    /// ```
    fn write_byte(&mut self, byte: u8) {
        match byte {
            b'\n' => self.new_line(),
            b'\r' => self.clear_line(),
            byte => {
                let vga_buffer = 0xb8000 as *mut u8;
                unsafe {
                    *vga_buffer.offset(self.column_position * 2) = byte;
                    *vga_buffer.offset(self.column_position * 2 + 1) = 0xb;
                }
                self.column_position += 1;
            }
        }
    }
}

/// TODO: All the code below needs/should be replaced with less satanic shit
// TODO: Replace with println! and print! macros before release
pub fn print_s(v: String) {
    WRITER.lock().print(v);
}

pub fn println_s(v: String) {
    WRITER.lock().print(v);
    WRITER.lock().new_line();
}

pub fn print(v: &str) {
    WRITER.lock().print(v.to_string());
}

pub fn println(v: &str) {
    WRITER.lock().print(v.to_string());
    WRITER.lock().new_line();
}

pub fn force_new_line() {
    WRITER.lock().new_line();
}

pub fn reset_screen() {
    WRITER.lock().reset_screen();
}
