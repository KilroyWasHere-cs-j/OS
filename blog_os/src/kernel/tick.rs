#![allow(dead_code)]

use alloc::string::String;

use super::{
    display::print_s,
    keyboard::{KeyboardHandler, KEYBOARD},
    scheduler::{Scheduler, Task},
};

fn keyboard_task() {
    let keys = KEYBOARD.lock().revel_text(); // TODO make this it's own task
                                             // clear the buffer
    KEYBOARD.lock().flush();
    // writer.clear_line();
    // only print if there are keys to print
    if !keys.is_empty() {
        print_s(keys.iter().collect::<String>());
    }
}

pub fn tick() {
    let keyboard_task = Task {
        id: 0,
        sticky: true,
        priority: super::scheduler::Priority::HIGH,
        state: super::scheduler::State::READY,
        fn_ptr: keyboard_task,
    };

    let cmd_task = Task {
        id: 1,
        sticky: true,
        priority: super::scheduler::Priority::HIGH,
        state: super::scheduler::State::READY,
        fn_ptr: super::tasks::cmd_task,
    };

    //super::scheduler::add_task(cmd_task);
    super::scheduler::add_task(keyboard_task);

    super::scheduler::LongTermScheduler::new().schedule();
    // super::scheduler::ShortTermScheduler::new().schedule();

    //println(count_tasks().to_string().as_str());
}
