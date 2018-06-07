use alloc::VecDeque;
use arch::interrupts;
use core::cell::UnsafeCell;
use core::ops::{Deref, DerefMut, Drop};
use core::sync::atomic::{AtomicBool, Ordering, ATOMIC_BOOL_INIT};
use syscall::error::Error;
use task::{global_sched, ProcessId, Scheduling, State};

pub struct IrqLock<T: ?Sized> {
    data: UnsafeCell<T>,
}

pub struct IrqGuard<'a, T: ?Sized + 'a> {
    data: &'a mut T,
    was_enabled: bool,
}

unsafe impl<T: ?Sized + Send> Sync for IrqLock<T> {}
unsafe impl<T: ?Sized + Send> Send for IrqLock<T> {}

impl<T> IrqLock<T> {
    pub const fn new(data: T) -> IrqLock<T> {
        IrqLock {
            data: UnsafeCell::new(data),
        }
    }

    pub fn lock(&self) -> IrqGuard<T> {
        let was_enabled = interrupts::enabled();
        if was_enabled {
            unsafe {
                interrupts::disable();
            }
        }

        IrqGuard {
            data: unsafe { &mut *self.data.get() },
            was_enabled,
        }
    }

    pub fn lock_map<F, U>(&self, f: F) -> IrqGuard<U>
    where
        F: FnOnce(&mut T) -> &mut U,
    {
        let was_enabled = interrupts::enabled();
        if was_enabled {
            unsafe {
                interrupts::disable();
            }
        }

        let data = f(unsafe { &mut *self.data.get() });

        IrqGuard { data, was_enabled }
    }
}

impl<T: ?Sized + Default> Default for IrqLock<T> {
    fn default() -> IrqLock<T> {
        IrqLock::new(Default::default())
    }
}

impl<'a, T: ?Sized> IrqGuard<'a, T> {
    /// Drops self
    pub fn release(self) {}
}

impl<'a, T: ?Sized> Deref for IrqGuard<'a, T> {
    type Target = T;
    fn deref(&self) -> &T {
        &*self.data
    }
}

impl<'a, T: ?Sized> DerefMut for IrqGuard<'a, T> {
    fn deref_mut(&mut self) -> &mut T {
        &mut *self.data
    }
}

impl<'a, T: ?Sized> Drop for IrqGuard<'a, T> {
    fn drop(&mut self) {
        if self.was_enabled {
            unsafe {
                interrupts::enable();
            }
        }
    }
}

#[derive(Debug)]
pub struct IrqSpinLock<T: ?Sized> {
    lock: AtomicBool,
    data: UnsafeCell<T>,
}

pub struct IrqSpinGuard<'a, T: ?Sized + 'a> {
    lock: &'a AtomicBool,
    was_enabled: bool,
    data: &'a mut T,
}

unsafe impl<T: ?Sized + Send> Sync for IrqSpinLock<T> {}
unsafe impl<T: ?Sized + Send> Send for IrqSpinLock<T> {}

impl<T> IrqSpinLock<T> {
    pub const fn new(data: T) -> IrqSpinLock<T> {
        IrqSpinLock {
            lock: ATOMIC_BOOL_INIT,
            data: UnsafeCell::new(data),
        }
    }

    fn obtain_lock(&self) {
        while self.lock.compare_and_swap(false, true, Ordering::Acquire) != false {
            while self.lock.load(Ordering::Relaxed) {
                interrupts::pause();
            }
        }
    }

    pub fn lock(&self) -> IrqSpinGuard<T> {
        self.obtain_lock();

        let was_enabled = interrupts::enabled();
        if was_enabled {
            unsafe {
                interrupts::disable();
            }
        }

        IrqSpinGuard {
            lock: &self.lock,
            was_enabled,
            data: unsafe { &mut *self.data.get() },
        }
    }

    pub fn try_lock(&self) -> Option<IrqSpinGuard<T>> {
        if self.lock.compare_and_swap(false, true, Ordering::Acquire) == false {
            let was_enabled = interrupts::enabled();
            if was_enabled {
                unsafe {
                    interrupts::disable();
                }
            }
            Some(IrqSpinGuard {
                lock: &self.lock,
                was_enabled,
                data: unsafe { &mut *self.data.get() },
            })
        } else {
            None
        }
    }
}

impl<T: ?Sized + Default> Default for IrqSpinLock<T> {
    fn default() -> IrqSpinLock<T> {
        IrqSpinLock::new(Default::default())
    }
}

impl<'a, T: ?Sized> IrqSpinGuard<'a, T> {
    /// Release the spinlock
    pub fn release(self) {}
}

impl<'a, T: ?Sized> Deref for IrqSpinGuard<'a, T> {
    type Target = T;
    fn deref(&self) -> &T {
        &*self.data
    }
}

impl<'a, T: ?Sized> DerefMut for IrqSpinGuard<'a, T> {
    fn deref_mut(&mut self) -> &mut T {
        &mut *self.data
    }
}

impl<'a, T: ?Sized> Drop for IrqSpinGuard<'a, T> {
    fn drop(&mut self) {
        self.lock.store(false, Ordering::Release);
        if self.was_enabled {
            unsafe {
                interrupts::enable();
            }
        }
    }
}

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
                    .expect("signal() - No threads waiting on semaphore");
                global_sched().ready(pid)?;

                // Important; Lock must be droppped before resched()
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
