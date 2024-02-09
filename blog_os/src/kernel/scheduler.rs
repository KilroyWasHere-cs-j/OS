/*
A basic operating system schedular

Author: Gabriel Tower
Written: 2/08/2024

Kilroy Was Here
*/

use alloc::vec::Vec;

static mut TASK_POOL: Vec<Task> = Vec::new();
static mut TASK_QUEUE: Vec<Task> = Vec::new();

/// A task priority
/// - LOW: A low priority task
/// - MED: A medium priority task
/// - HIGH: A high priority task
#[derive(Clone)]
pub enum Priority {
    LOW,
    MED,
    HIGH
}

/// A task state
/// - READY: A task is ready to be executed
/// - RUNNING: A task is currently being executed
/// - BLOCKED: A task is blocked from being executed
/// - CLEARED: A task has been cleared from the task pool
#[derive(Clone)]
pub enum State{
    READY,
    RUNNING,
    BLOCKED,
    CLEARED
}


/// A task
/// - id: A unique identifier for the task
/// - sticky: A boolean value that determines if the task will be removed from the task pool after execution
/// - priority: The priority of the task
/// - state: The state of the task
/// - fn_ptr: A function pointer to the task
#[derive(Clone)]
pub struct Task{
    pub id: usize,
    pub sticky: bool,
    pub priority: Priority,
    pub state: State,
    pub fn_ptr: fn(),
}


/// A long term scheduler
/// - holding_queue: A queue of tasks that are waiting to be executed
pub struct LongTermScheduler{
    pub holding_queue: Vec<Task>,
}

pub struct ShortTermScheduler{
    pub run_queue: Vec<Task>,
    pub holding_queue: Vec<Task>,
    pub current_task: Task,
}

pub trait Scheduler{
    /// Create a new scheduler
    /// - returns: A new scheduler
    fn new() -> Self;
    /// Schedule tasks
    /// - returns: None
    fn schedule(&mut self);
    /// Flush the scheduler
    /// - returns: None
    fn flush(&mut self);
    /// Prune the scheduler
    /// Removes non sticking a CLEARED tasks from the task pool
    /// - returns: None
    fn prune(&mut self);
    /// Sort the TASK_POOL
    /// Sorts tasks in the TASK_POOL by priority
    /// - returns: None
    fn sort(&mut self);
}

impl Scheduler for LongTermScheduler{
    fn new() -> LongTermScheduler{
        LongTermScheduler{
            holding_queue: Vec::new(),
        }
    }

    fn schedule(&mut self){
        for task in unsafe{TASK_POOL.iter()}{
            execute_task(task.clone());
        }
    }

    fn flush(&mut self) {
        unsafe{
            TASK_POOL.clear();
        }
    }

    fn sort(&mut self) {
        unsafe {
            for task in TASK_POOL.iter(){
                let task_pro = task.priority.clone();
                match task_pro{
                    Priority::LOW => {
                        self.holding_queue.push(task.clone());
                    }
                    Priority::MED => {
                        self.holding_queue.push(task.clone());
                    }
                    Priority::HIGH => {
                        TASK_POOL[0] = task.clone();
                    }
                }
            }
        }
    }

    fn prune(&mut self) {
        todo!()
    }
}

/// Execute a task
/// - task: A task to be executed
/// - returns: None
fn execute_task(task: Task){
    let fn_ptr = task.fn_ptr;
    fn_ptr();
}

/// Add a task to the task pool
/// - task: A task to be added to the task pool
/// - returns: None
/// # Safety
/// This function is unsafe because it accesses a mutable static variable
/// TASK_POOL
pub fn add_task(task: Task){
    unsafe{
        TASK_POOL.push(task);
    }
}

/// Remove a task from the task pool
/// - task: A task to be removed from the task pool
/// - returns: None
/// # Safety
/// This function is unsafe because it accesses a mutable static variable
/// TASK_POOL
pub fn remove_task(task: Task){
    unsafe{
        TASK_POOL.retain(|x| x.id != task.id);
    }
}