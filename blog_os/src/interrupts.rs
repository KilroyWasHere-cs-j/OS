use crate::{print, println};
use alloc::string::String;
use lazy_static::lazy_static;
use pc_keyboard::{layouts, DecodedKey, HandleControl, Keyboard, ScancodeSet1};
use x86_64::{
    instructions::{interrupts, port::Port},
    structures::idt::{InterruptDescriptorTable, InterruptStackFrame},
};

#[path = "./kernel/mod.rs"]
mod kernel;

use pic8259::ChainedPics;
use spin;

use kernel::keyboard::KeyboardHandler;

use kernel::tick;

pub const PIC_1_OFFSET: u8 = 32;
pub const PIC_2_OFFSET: u8 = PIC_1_OFFSET + 8;

pub static PICS: spin::Mutex<ChainedPics> =
    spin::Mutex::new(unsafe { ChainedPics::new(PIC_1_OFFSET, PIC_2_OFFSET) });

lazy_static! {
    static ref KEYBOARD: spin::Mutex<kernel::keyboard::Keyboard> =
        spin::Mutex::new(kernel::keyboard::Keyboard::new());
}

lazy_static! {
    // create a static referenceuse to the interrupt descriptor table
    static ref IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();
        idt.breakpoint.set_handler_fn(breakpoint_handler);
        idt.divide_error.set_handler_fn(divide_error_handler);
        idt[InterruptIndex::Timer.as_usize()].set_handler_fn(timer_interrupt_handler);
        idt[InterruptIndex::Keyboard.as_usize()].set_handler_fn(keyboard_interrupt_handler);

        // return the interrupt descriptor table
        idt
    };
}

// Interrupt handlers

/// Interrupt handler for the timer
extern "x86-interrupt" fn timer_interrupt_handler(_stack_frame: InterruptStackFrame) {
    // Commented out because it's pointless and gets in the way
    //let keys = KEYBOARD.lock().revel_text();

    tick::tick();

    let keys = KEYBOARD.lock().revel_text();
    KEYBOARD.lock().clear_text();

    if !keys.is_empty() {
        print!("{}", keys.iter().collect::<String>());
    }
    // notify system that the interrupt has been handled
    unsafe {
        PICS.lock()
            .notify_end_of_interrupt(InterruptIndex::Timer.as_u8());
    }
}

extern "x86-interrupt" fn keyboard_interrupt_handler(_stack_frame: InterruptStackFrame) {
    // create a new keyboard with the US layout
    let mut keyboard = Keyboard::new(layouts::Us104Key, ScancodeSet1, HandleControl::Ignore);

    // read the scancode from the data port
    let mut port = Port::new(0x60);
    let scancode: u8 = unsafe { port.read() };

    // decode the scancode
    if let Ok(Some(key_event)) = keyboard.add_byte(scancode) {
        // print the key event
        if let Some(key) = keyboard.process_keyevent(key_event) {
            match key {
                DecodedKey::Unicode(character) => KEYBOARD.lock().on_key(character),
                DecodedKey::RawKey(key) => print!("{:?}", key),
            }
        }
    }
    //KEYBOARD.lock().on_key(scancode);
    // notify system that the interrupt has been handled
    unsafe {
        PICS.lock()
            .notify_end_of_interrupt(InterruptIndex::Keyboard.as_u8());
    }
}

extern "x86-interrupt" fn divide_error_handler(stack_frame: InterruptStackFrame) {
    println!("EXCEPTION: DIVIDE ERROR\n{:#?}", stack_frame);
}

pub fn init_idt() {
    IDT.load();
}

#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum InterruptIndex {
    Timer = PIC_1_OFFSET,
    Keyboard,
}

impl InterruptIndex {
    fn as_u8(self) -> u8 {
        self as u8
    }

    fn as_usize(self) -> usize {
        usize::from(self.as_u8())
    }
}

extern "x86-interrupt" fn breakpoint_handler(stack_frame: InterruptStackFrame) {
    // ! Don't uncomment this shit it will break the OS's boot
    // Check tests for possible cause :)
    //println!("EXCEPTION: BREAKPOINT\n{:#?}", stack_frame);
}

#[test_case]
fn test_breakpoint_exception() {
    // invoke a breakpoint exception
    x86_64::instructions::interrupts::int3();
}
