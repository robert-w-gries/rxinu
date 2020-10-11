use super::{Error, Scheduler, Task, TaskId, TaskWaker};
use alloc::{collections::BTreeMap, sync::Arc};
use core::task::{Context, Poll, Waker};
use crossbeam_queue::ArrayQueue;

pub struct CooperativeExecutor {
    tasks: BTreeMap<TaskId, Task>,
    task_queue: Arc<ArrayQueue<TaskId>>,
    waker_cache: BTreeMap<TaskId, Waker>,
}

impl CooperativeExecutor {
    pub fn new() -> Self {
        CooperativeExecutor {
            tasks: BTreeMap::new(),
            task_queue: Arc::new(ArrayQueue::new(1024)),
            waker_cache: BTreeMap::new(),
        }
    }

    pub fn run_ready_tasks(&mut self) {
        // destructure `self` to avoid borrow checker errors
        let Self {
            tasks,
            task_queue,
            waker_cache,
        } = self;

        while let Ok(task_id) = task_queue.pop() {
            let task = match tasks.get_mut(&task_id) {
                Some(task) => task,
                None => continue,
            };
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
        use x86_64::instructions::interrupts::{self, enable_interrupts_and_hlt};

        interrupts::disable();
        if self.task_queue.is_empty() {
            enable_interrupts_and_hlt();
        } else {
            interrupts::enable();
        }
    }
}

impl Scheduler for CooperativeExecutor {
    fn run(&mut self) -> ! {
        loop {
            self.run_ready_tasks();
            self.sleep_if_idle();
        }
    }

    fn spawn(&mut self, task: Task) -> Result<(), Error> {
        let task_id = task.id;
        if self.tasks.insert(task.id, task).is_some() {
            return Err(Error::DuplicateId);
        }
        self.task_queue.push(task_id).map_err(|_| Error::TaskQueueFull)
    }

    fn kill(&mut self, task_id: TaskId) -> Result<(), Error> {
        self.tasks.remove(&task_id).ok_or(Error::UnknownId)?;
        Ok(())
    }
}