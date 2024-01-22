use alloc::{sync::Arc, vec::Vec};
use lazy_static::lazy_static;
use spin::Mutex;

lazy_static! {
    /// Job pool
    pub static ref JOB_POOL: Arc<Mutex<Vec<Task>>> = Mutex::new(Vec::new()).into();
}
/// Task state
/// - Running: The task is currently running
/// - Ready: The task is ready to run
/// - Blocked: The task is blocked and waiting for an event to unblock
/// - Finished: The task has finished and is waiting to be cleaned up
pub enum TaskState {
    Running,
    Ready,
    Blocked,
    Finished,
}

/// Task priority
/// - High: The task has high priority
/// - Medium: The task has medium priority
/// - Low: The task has low priority
pub enum TaskPriority {
    High,
    Medium,
    Low,
}

/// Task
/// - id: The task id
/// - state: The task state
/// - priority: The task priority
pub struct Task {
    id: usize,
    state: TaskState,
    priority: TaskPriority,
}

pub struct LongTermScheduler {}

pub struct ShortTermScheduler {}

/// Updates all scheduled tasks
pub fn tick() {}
