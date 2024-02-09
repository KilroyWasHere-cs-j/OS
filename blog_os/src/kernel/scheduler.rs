use alloc::{string::ToString, vec::Vec};
use spin::Mutex;

use super::display::{print, println};

static mut TASK_POOL: Vec<Task> = Vec::new();
static mut TASK_QUEUE: Vec<Task> = Vec::new();

#[derive(Clone)]
pub enum Priority {
    LOW,
    MED,
    HIGH
}

#[derive(Clone)]
pub enum State{
    READY,
    RUNNING,
    BLOCKED,
    CLEARED
}

#[derive(Clone)]
pub struct Task{
    pub id: usize,
    pub sticky: bool,
    pub priority: Priority,
    pub state: State,
    pub fn_ptr: fn(),
}

pub struct LongTermScheduler{
    pub holding_queue: Vec<Task>,
}

pub struct ShortTermScheduler{
    pub run_queue: Vec<Task>,
    pub holding_queue: Vec<Task>,
    pub current_task: Task,
}

pub trait Scheduler{
    fn new() -> Self;
    fn schedule(&mut self);
}

impl Scheduler for LongTermScheduler{
    fn new() -> LongTermScheduler{
        LongTermScheduler{
            holding_queue: Vec::new(),
        }
    }

    fn schedule(&mut self){
        for task in unsafe{TASK_POOL.iter()}{
            unsafe{
                TASK_QUEUE.push(task.clone());
            }
        }
    }
}

impl Scheduler for ShortTermScheduler{
    fn new() -> ShortTermScheduler{
        ShortTermScheduler{
            run_queue: Vec::new(),
            current_task: Task{
                id: 0,
                sticky: false,
                priority: Priority::LOW,
                state: State::READY,
                fn_ptr: ||{},
            },
            holding_queue: Vec::new(),
        }
    }

    fn schedule(&mut self){
        for task in unsafe{TASK_QUEUE.iter()}{
            let fn_ptr = task.fn_ptr;
            fn_ptr();
        }
    }
}

// struct Executor{
//     current_task: Task,
// }

// impl Executor{
//     pub fn new() -> Executor{
//         Executor{
//             current_task: Task{
//                 id: 0,
//                 sticky: false,
//                 priority: Priority::LOW,
//                 state: State::READY,
//                 fn_ptr: ||{},
//             }
//         }
//     }

//     pub fn run(&mut self){
//         loop{
//             self.long_term_scheduler.schedule();
//             self.short_term_scheduler.schedule();
//         }
//     }
// }

pub fn add_task(task: Task){
    unsafe{
        TASK_POOL.push(task);
    }
}

pub fn remove_task(task: Task){
    unsafe{
        TASK_POOL.retain(|x| x.id != task.id);
    }
}