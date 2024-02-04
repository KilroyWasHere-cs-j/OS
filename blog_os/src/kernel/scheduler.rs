use alloc::{sync::Arc, vec::Vec};

/// Struct to store the job pool
pub struct JobPool {
    pub jobs: Vec<Arc<Task>>,
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
}

impl JobPool {
    pub fn new() -> Self {
        JobPool { jobs: Vec::new() }
    }

    pub fn add_job(&mut self, job: Arc<Task>) {
        self.jobs.push(job);
    }

    pub fn remove_job(&mut self, job: Arc<Task>) {
        todo!();
    }
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
