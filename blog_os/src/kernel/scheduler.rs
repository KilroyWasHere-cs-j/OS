use alloc::{sync::Arc, vec::Vec};
use lazy_static::lazy_static;
use spin::Mutex;

lazy_static! {
    pub static ref JOBPOOL: Arc<Mutex<JobPool>> = Arc::new(Mutex::new(JobPool::new()));
}

lazy_static! {
    static ref TASK_QUEUE: Mutex<Vec<Arc<Task>>> = Mutex::new(Vec::new());
}

lazy_static! {
    static ref LONGTERMSCHEDULER: Mutex<LongTermScheduler> = Mutex::new(LongTermScheduler::new());
}

lazy_static! {
    static ref SHORTTERMSCHEDULER: Mutex<ShortTermScheduler> =
        Mutex::new(ShortTermScheduler::new());
}

/// Job pool
/// - jobs: Vec<Arc<Task>>: The tasks to be scheduled
pub struct JobPool {
    pub jobs: Vec<Arc<Task>>,
}

/// Task state
/// - Ready: The task is ready to be executed
/// - Running: The task is currently running
/// - Blocked: The task is blocked
/// - Terminated: The task has terminated
pub enum TaskState {
    Ready,
    Running,
    Blocked,
    Terminated,
    Cleared,
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

/// Long term scheduler
/// - tasks: The tasks to be scheduled
struct LongTermScheduler {
    pub tasks: Vec<Task>,
}

/// Short term scheduler
/// - tasks: The tasks to be scheduled
struct ShortTermScheduler {
    pub tasks: Vec<Task>,
}

/// Trait defining a scheduler
/// - new: Creates a new scheduler
/// - add_task: Adds a task to the scheduler
/// - remove_task: Removes a task from the scheduler
/// - tick: Updates all scheduled tasks
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

    pub fn flush(&mut self) {
        self.jobs.clear();
    }

    pub fn prune(&mut self) {
        todo!("Prune the job pool");
    }
}

/// This function is only called by "timer_interrupt_handler"
/// It call the schedulers to update
pub fn tick() {
    LONGTERMSCHEDULER.lock().tick();
    SHORTTERMSCHEDULER.lock().tick();
}

impl Scheduler for LongTermScheduler {
    fn new() -> Self {
        LongTermScheduler { tasks: Vec::new() }
    }

    fn add_task(&mut self, task: Task) {
        self.tasks.push(task);
    }

    fn remove_task(&mut self, task: Task) {
        self.tasks.retain(|x| x.id != task.id);
    }

    fn tick(&mut self) {
        // Add every task in JOBPOOL to JOBQUEUE
        for task in JOBPOOL.lock().jobs.iter() {
            TASK_QUEUE.lock().push(task.clone());
        }

        // Clear JOBPOOL as all jobs should be loaded by now
        JOBPOOL.lock().flush();
    }
}

impl Scheduler for ShortTermScheduler {
    fn new() -> Self {
        ShortTermScheduler { tasks: Vec::new() }
    }

    fn add_task(&mut self, task: Task) {
        self.tasks.push(task);
    }

    fn remove_task(&mut self, task: Task) {
        self.tasks.retain(|x| x.id != task.id);
    }

    fn tick(&mut self) {
        // Do some better sorting here
        for task in TASK_QUEUE.lock().iter() {
            let fn_ptr = task.fn_ptr;
            fn_ptr();
            //TODO wrap task in a mutex so that we can change its state
        }
    }
}
