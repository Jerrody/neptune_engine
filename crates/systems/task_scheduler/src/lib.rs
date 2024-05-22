mod task;

use rayon::prelude::*;
use std::collections::HashSet;
use std::sync::Arc;
use task::*;

pub struct TaskScheduler {
    tasks: Vec<ScheduledTask>,
}

impl TaskScheduler {
    pub fn new() -> Self {
        TaskScheduler { tasks: Vec::new() }
    }

    pub fn add_task(&mut self, task: Arc<dyn Task>, order: usize) {
        let existing_orders: HashSet<usize> = self.tasks.iter().map(|t| t.order).collect();
        let mut adjusted_order = order;
        while existing_orders.contains(&adjusted_order) {
            adjusted_order += 1;
        }

        self.tasks.push(ScheduledTask::new(task, adjusted_order));
    }

    pub fn run(&mut self) {
        self.tasks.par_drain(..).for_each(|scheduled_task| {
            scheduled_task.task.execute();
        });
    }
}
