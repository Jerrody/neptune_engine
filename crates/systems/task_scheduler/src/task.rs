use std::sync::Arc;

pub trait Task: Send + Sync {
    fn execute(&self);
    fn order(&self) -> usize;
    fn dependencies(&self) -> Vec<usize> {
        Default::default()
    }
}

pub struct ScheduledTask {
    pub task: Arc<dyn Task>,
    pub order: usize,
}

impl ScheduledTask {
    #[inline(always)]
    pub fn new(task: Arc<dyn Task>, order: usize) -> Self {
        Self { task, order }
    }
}

impl PartialEq for ScheduledTask {
    #[inline(always)]
    fn eq(&self, other: &Self) -> bool {
        self.order == other.order
    }
}

impl Eq for ScheduledTask {}

impl PartialOrd for ScheduledTask {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.order.cmp(&other.order))
    }
}

impl Ord for ScheduledTask {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.order.cmp(&other.order)
    }
}
