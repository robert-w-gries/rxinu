use alloc::VecDeque;
use sync::IrqLock;
use syscall::error::Error;
use task::{global_sched, ProcessId, Scheduling, State};

pub struct Semaphore {
    inner: IrqLock<SemaphoreInner>,
}

struct SemaphoreInner {
    count: usize,
    wait_queue: VecDeque<ProcessId>,
}

impl Semaphore {
    pub fn new(count: usize) -> Semaphore {
        Semaphore {
            inner: IrqLock::new(SemaphoreInner {
                count: count,
                wait_queue: VecDeque::new(),
            }),
        }
    }

    pub fn signal(&self) -> Result<(), Error> {
        let mut inner = self.inner.lock();

        match inner.count {
            0 => {
                let pid = inner
                    .wait_queue
                    .pop_front()
                    .expect("signal() - No processes waiting on semaphore");
                global_sched().ready(pid)?;

                // Safety: Lock must be droppped before resched()
                drop(inner);
                unsafe {
                    global_sched().resched()?;
                }
            }
            _ => inner.count += 1,
        }

        Ok(())
    }

    pub fn wait(&self) -> Result<(), Error> {
        let mut inner = self.inner.lock();

        match inner.count {
            0 => {
                let curr_pid = global_sched().get_pid();
                global_sched().modify_process(curr_pid, |proc_ref| {
                    proc_ref.write().state = State::Wait;
                })?;

                inner.wait_queue.push_back(curr_pid);

                // Safety: Lock must be dropped before resched()
                drop(inner);
                unsafe {
                    global_sched().resched()?;
                }
            }
            _ => inner.count -= 1,
        }

        Ok(())
    }
}
