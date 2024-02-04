use alloc::{sync::Arc, vec::Vec};
use lazy_static::lazy_static;
use spin::Mutex;

/// Struct to store the job pool
pub struct JobPool {
    pub jobs: Vec<Arc<Task>>,
}

lazy_static! {
    pub static ref JOBPOOL: Arc<Mutex<JobPool>> = Arc::new(Mutex::new(JobPool::new()));
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
    pub id: usize,
    pub state: TaskState,
    pub priority: TaskPriority,
    pub fn_ptr: fn(),
}

/// Trait defining a scheduler`
pub trait Scheduler {
    fn new() -> Self;
    /// Adds a task to the scheduler
    fn add_task(&mut self, task: Task);
    /// Removes a task from the scheduler
    fn remove_task(&mut self, task: Task);
    /// Updates all scheduled tasks
    fn tick(&mut self);
}

impl JobPool {
    pub fn new() -> Self {
        JobPool { jobs: Vec::new() }
    }

    pub fn add_task(&mut self, task: Task) {
        self.jobs.push(Arc::new(task));
    }

    pub fn remove_task(&mut self, task: Task) {
        self.jobs.retain(|x| x.id != task.id);
    }
}
