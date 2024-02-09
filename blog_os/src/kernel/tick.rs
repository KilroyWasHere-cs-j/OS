use alloc::{string::{String, ToString}, vec::Vec};

use super::{display::{print, print_s, println}, keyboard::{self, KeyboardHandler, KEYBOARD}, scheduler::{Scheduler, Task}};

fn keyboard_task() {
    let keys = KEYBOARD.lock().revel_text();
    // clear the buffer
    KEYBOARD.lock().flush();
    // writer.clear_line();
    // only print if there are keys to print
    if !keys.is_empty() {
        print_s(keys.iter().collect::<String>());
    }
}

fn task_test(){
    //print("Task Test");
}

pub fn tick(){

    let keyboard_task = Task{
        id: 0,
        sticky: true,
        priority: super::scheduler::Priority::HIGH,
        state: super::scheduler::State::READY,
        fn_ptr: keyboard_task,
    };

    let task_test = Task{
        id: 1,
        sticky: false,
        priority: super::scheduler::Priority::LOW,
        state: super::scheduler::State::READY,
        fn_ptr: task_test,
    };

    super::scheduler::add_task(keyboard_task);
    super::scheduler::add_task(task_test);

    super::scheduler::LongTermScheduler::new().schedule();
    super::scheduler::ShortTermScheduler::new().schedule();

    //println(count_tasks().to_string().as_str());
}