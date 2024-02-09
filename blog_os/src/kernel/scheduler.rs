use alloc::{string::ToString, vec::Vec};
use spin::Mutex;

use super::display::{print, println};


pub enum Priority {
    LOW,
    MED,
    HIGH
}

pub enum State{
    READY,
    RUNNING,
    BLOCKED,
    CLEARED
}


pub struct Task{
    pub id: usize,
    pub sticky: bool,
    pub priority: Priority,
    pub state: State,
    pub fn_ptr: fn(),
}
