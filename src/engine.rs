use task_scheduler::TaskScheduler;

pub struct Engine {
    task_scheduler: TaskScheduler,
}

impl Engine {
    pub fn new() -> Self {
        let task_scheduler = TaskScheduler::new();

        Self {
            task_scheduler
        }
    }
}
