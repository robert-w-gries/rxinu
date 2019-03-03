use alloc::collections::VecDeque;
use core::sync::atomic::{AtomicUsize, Ordering};

use crate::arch::interrupts;
use crate::syscall::error::Error;
use crate::task::{global_sched, ProcessId, Scheduling, State};

#[derive(Debug)]
pub struct Semaphore {
    count: AtomicUsize,
    wait_queue: VecDeque<ProcessId>,
    waiting: bool,
}

impl Semaphore {
    pub fn new(count: usize) -> Semaphore {
        Semaphore {
            count: AtomicUsize::new(count),
            wait_queue: VecDeque::new(),
            waiting: false,
        }
    }

    pub fn count(&self) -> usize {
        self.count.load(Ordering::SeqCst)
    }

    pub fn signal(&mut self) -> Result<(), Error> {
        self.signaln(1)
    }

    pub fn signaln(&mut self, count: usize) -> Result<(), Error> {
        interrupts::disable_then_execute(|| {
            let mut should_resched = false;

            for _ in 0..count {
                if self.waiting {
                    let pid = self
                        .wait_queue
                        .pop_front()
                        .expect("signal() - No processes waiting on semaphore");
                    global_sched().ready(pid)?;

                    self.waiting = !self.wait_queue.is_empty();
                    should_resched = true;
                } else {
                    self.count.fetch_add(1, Ordering::SeqCst);
                }
            }

            if should_resched {
                unsafe {
                    global_sched().resched()?;
                }
            }

            Ok(())
        })
    }

    pub fn wait(&mut self) -> Result<(), Error> {
        match self.count() {
            0 => interrupts::disable_then_execute(|| {
                let curr_pid = global_sched().get_pid();
                global_sched().modify_process(curr_pid, |proc_ref| {
                    proc_ref.write().state = State::Wait;
                })?;

                self.wait_queue.push_back(curr_pid);
                self.waiting = true;

                unsafe {
                    global_sched().resched()?;
                }

                Ok(())
            }),
            _ => {
                self.count.fetch_sub(1, Ordering::SeqCst);
                Ok(())
            }
        }
    }
}
