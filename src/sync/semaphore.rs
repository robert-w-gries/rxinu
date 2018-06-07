use alloc::VecDeque;
use core::sync::atomic::{AtomicUsize, Ordering};
use syscall::error::Error;
use task::{global_sched, ProcessId, Scheduling, State};

pub struct Semaphore {
    count: AtomicUsize,
    wait_queue: VecDeque<ProcessId>,
}

impl Semaphore {
    pub fn new(count: usize) -> Semaphore {
        Semaphore {
            count: AtomicUsize::new(count),
            wait_queue: VecDeque::new(),
        }
    }

    pub fn count(&self) -> usize {
        self.count.load(Ordering::SeqCst)
    }

    pub fn signal(&mut self) -> Result<(), Error> {
        self.signaln(1)
    }

    pub fn signaln(&mut self, count: usize) -> Result<(), Error> {
        let mut should_resched = false;

        match self.count() {
            0 => {
                self.count.fetch_add(1, Ordering::SeqCst);

                let pid = self
                    .wait_queue
                    .pop_front()
                    .expect("signal() - No processes waiting on semaphore");
                global_sched().ready(pid)?;

                should_resched = true;
            }
            _ => {
                self.count.fetch_add(1, Ordering::SeqCst);
            }
        }

        for _ in 1..count {
            self.count.fetch_add(1, Ordering::SeqCst);
        }

        if should_resched {
            unsafe {
                global_sched().resched()?;
            }
        }

        Ok(())
    }

    pub fn wait(&mut self) -> Result<(), Error> {
        match self.count() {
            0 => {
                let curr_pid = global_sched().get_pid();
                global_sched().modify_process(curr_pid, |proc_ref| {
                    proc_ref.write().state = State::Wait;
                })?;

                self.wait_queue.push_back(curr_pid);

                unsafe {
                    global_sched().resched()?;
                }

                self.count.fetch_sub(1, Ordering::SeqCst);
            }
            _ => {
                self.count.fetch_sub(1, Ordering::SeqCst);
            }
        }

        Ok(())
    }
}
