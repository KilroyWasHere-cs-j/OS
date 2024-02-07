use lazy_static::lazy_static;
use pc_keyboard::{layouts, DecodedKey, HandleControl, Keyboard, ScancodeSet1};
use x86_64::{
    instructions::port::Port,
    structures::idt::{InterruptDescriptorTable, InterruptStackFrame},
};

use alloc::string::String;

#[path = "./System69/mod.rs"]
mod System69;

#[path = "./kernel/mod.rs"]
mod kernel;

use pic8259::ChainedPics;

use crate::interrupts::kernel::display;
use kernel::keyboard::KeyboardHandler;

use self::{kernel::{display::{print, println}, scheduler::{self, Task, TaskPriority, TaskState, JOBPOOL}}, System69::cmdprmpt};
use crate::interrupts::kernel::keyboard::KEYBOARD;

// use crate::{print, println};

pub const PIC_1_OFFSET: u8 = 32;
pub const PIC_2_OFFSET: u8 = PIC_1_OFFSET + 8;

pub static PICS: spin::Mutex<ChainedPics> =
    spin::Mutex::new(unsafe { ChainedPics::new(PIC_1_OFFSET, PIC_2_OFFSET) });

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

fn keyboard_task() {
    let keys = KEYBOARD.lock().revel_text();
    // clear the buffer
    KEYBOARD.lock().flush();
    // writer.clear_line();
    // only print if there are keys to print
    if !keys.is_empty() {
        display::print_s(keys.iter().collect::<String>());
    }
}


pub fn process_prmt(){
    // // Need a better way to do this
    // let prompt = KEYBOARD.lock().revel_text();
    // let s: String = prompt.into_iter().collect();
    // match s.as_str(){
    //     "help" => {
    //         println("This is the help menu");
    //         println("help - displays this menu");
    //         println("clear - clears the screen");
    //         println("exit - exits the command prompt");
    //     },
    //     "clear" => {
    //         for _ in 0..100{
    //             println("");
    //         }
    //     },
    //     "exit" => {
    //         println("Exiting command prompt");
    //         return;
    //     },
    //     _ => {
    //         // Do nothing
    //         // As doing something would cause to many issues
    //         print("Unknown command: ");
    //         print(&s);
    //     }
    // }

    print("Hello")
}


// Interrupt handlers

/// Interrupt handler for the timer
extern "x86-interrupt" fn timer_interrupt_handler(_stack_frame: InterruptStackFrame) {
    // create a new task for the keyboard
    let keyboard_task = Task {
        id: 0,
        state: TaskState::Ready,
        priority: TaskPriority::High,
        fn_ptr: keyboard_task,
    };

    let cmdprmpt_task = Task {
        id: 1,
        state: TaskState::Ready,
        priority: TaskPriority::High,
        fn_ptr: cmdprmpt::process_prmt,
    };

    // add the task to the job pool
    JOBPOOL.lock().add_task(keyboard_task);
    JOBPOOL.lock().add_task(cmdprmpt_task);
    // call tick so the schedulers can do their updating
    scheduler::tick();

    // notify system that the interrupt has been handled and it's okay to unlock
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
                DecodedKey::RawKey(_key) => (),
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

extern "x86-interrupt" fn divide_error_handler(_stack_frame: InterruptStackFrame) {
    // println!("EXCEPTION: DIVIDE ERROR\n{:#?}", stack_frame);
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

extern "x86-interrupt" fn breakpoint_handler(_stack_frame: InterruptStackFrame) {
    // ! Don't uncomment this shit it will break the OS's boot
    // Check tests for possible cause :)
    //println!("EXCEPTION: BREAKPOINT\n{:#?}", stack_frame);
}

#[test_case]
fn test_breakpoint_exception() {
    // invoke a breakpoint exception
    x86_64::instructions::interrupts::int3();
}
