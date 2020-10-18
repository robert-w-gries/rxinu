use super::{Error, Scheduler, TaskWaker};
use crate::arch::interrupts;
use crate::task::{Priority, PriorityTask, TaskFuture, TaskId};
use alloc::{collections::BTreeMap, sync::Arc};
use core::task::{Context, Poll, Waker};
use crossbeam_queue::ArrayQueue;

pub struct PriorityScheduler {
    tasks: BTreeMap<TaskId, PriorityTask>,
    high_queue: Arc<ArrayQueue<TaskId>>,
    medium_queue: Arc<ArrayQueue<TaskId>>,
    low_queue: Arc<ArrayQueue<TaskId>>,
    waker_cache: BTreeMap<TaskId, Waker>,
}

impl PriorityScheduler {
    pub fn new() -> Self {
        PriorityScheduler {
            tasks: BTreeMap::new(),
            high_queue: Arc::new(ArrayQueue::new(1024)),
            medium_queue: Arc::new(ArrayQueue::new(1024)),
            low_queue: Arc::new(ArrayQueue::new(1024)),
            waker_cache: BTreeMap::new(),
        }
    }

    pub fn run_ready_tasks(&mut self) {
        while !self.is_idle() {
            if let Ok(task_id) = self.high_queue.pop() {
                self.execute_priority_task(task_id, self.high_queue.clone());
            } else if let Ok(task_id) = self.medium_queue.pop() {
                self.execute_priority_task(task_id, self.medium_queue.clone());
            } else if let Ok(task_id) = self.low_queue.pop() {
                self.execute_priority_task(task_id, self.low_queue.clone());
            }
        }
    }

    fn execute_priority_task(&mut self, task_id: TaskId, task_queue: Arc<ArrayQueue<TaskId>>) {
        let Self {
            tasks, waker_cache, ..
        } = self;

        if let Some(task) = tasks.get_mut(&task_id) {
            let waker = waker_cache
                .entry(task_id)
                .or_insert_with(|| TaskWaker::new(task_id, task_queue.clone()));
            let mut context = Context::from_waker(waker);
            match task.poll(&mut context) {
                Poll::Ready(()) => {
                    // task done -> remove it and its cached waker
                    tasks.remove(&task_id);
                    waker_cache.remove(&task_id);
                }
                Poll::Pending => {}
            }
        }
    }

    fn sleep_if_idle(&self) {
        interrupts::disable();
        if self.is_idle() {
            interrupts::enable_and_hlt();
        } else {
            interrupts::enable();
        }
    }

    fn is_idle(&self) -> bool {
        return self.high_queue.is_empty()
            && self.medium_queue.is_empty()
            && self.low_queue.is_empty();
    }
}

impl Scheduler<PriorityTask> for PriorityScheduler {
    fn run(&mut self) -> ! {
        loop {
            self.run_ready_tasks();
            self.sleep_if_idle();
        }
    }

    fn spawn(&mut self, task: PriorityTask) -> Result<(), Error> {
        let task_id = task.id();
        let priority = task.priority();
        if self.tasks.insert(task_id, task).is_some() {
            return Err(Error::DuplicateId);
        }
        match priority {
            Priority::High => self
                .high_queue
                .push(task_id)
                .map_err(|_| Error::TaskQueueFull),
            Priority::Medium => self
                .medium_queue
                .push(task_id)
                .map_err(|_| Error::TaskQueueFull),
            Priority::Low => self
                .low_queue
                .push(task_id)
                .map_err(|_| Error::TaskQueueFull),
        }
    }

    fn kill(&mut self, task_id: TaskId) -> Result<(), Error> {
        self.tasks.remove(&task_id).ok_or(Error::UnknownId)?;
        Ok(())
    }
}
