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

/// Trait defining a scheduler`
pub trait Scheduler {
    /// Adds a task to the scheduler
    fn add_task(&mut self, task: Task);
    /// Removes a task from the scheduler
    fn remove_task(&mut self, task: Task);
    /// Updates all scheduled tasks
    fn tick(&mut self);
}

/// Long term scheduler
pub struct LongTermScheduler {
    /// The job pool
    job_pool: Arc<Mutex<Vec<Task>>>,
    /// The ready queue
    ready_queue: Arc<Mutex<Vec<Task>>>,
    /// The blocked queue
    blocked_queue: Arc<Mutex<Vec<Task>>>,
    /// The finished queue
    finished_queue: Arc<Mutex<Vec<Task>>>,
}

/// Short term scheduler
pub struct ShortTermScheduler {
    /// The ready queue
    ready_queue: Arc<Mutex<Vec<Task>>>,
    /// The running task
    running_task: Arc<Mutex<Task>>,
}
