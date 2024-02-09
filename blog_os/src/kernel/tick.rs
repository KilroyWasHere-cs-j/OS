use alloc::{string::{String, ToString}, vec::Vec};

use super::{display::{print, print_s, println}, keyboard::{self, KeyboardHandler, KEYBOARD}, scheduler::Task};

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

static mut TASK_QUEUE: Vec<Task> = Vec::new();

pub fn tick(){

    let keyboard_task = Task{
        id: 0,
        sticky: true,
        priority: super::scheduler::Priority::LOW,
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

    unsafe{
        TASK_QUEUE.push(keyboard_task);
        TASK_QUEUE.push(task_test);
    }

    for task in unsafe{TASK_QUEUE.iter_mut()}{
        let task_fn = task.fn_ptr;
        task_fn();
    }

    //println(count_tasks().to_string().as_str());
}

pub fn count_tasks() -> usize{
    unsafe{
        TASK_QUEUE.len()
    }
}