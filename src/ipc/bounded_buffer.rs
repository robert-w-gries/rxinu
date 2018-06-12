use alloc::VecDeque;
use arch::interrupts;
use sync::Semaphore;
use syscall::error::Error;

#[derive(Debug)]
pub struct BoundedBuffer<T> {
    data: VecDeque<T>,
    data_count: Semaphore,
    empty_count: Semaphore,
    mutex: Semaphore,
}

impl<T> BoundedBuffer<T> {
    pub fn new(size: usize) -> BoundedBuffer<T> {
        BoundedBuffer {
            data: VecDeque::with_capacity(size),
            data_count: Semaphore::new(0),
            empty_count: Semaphore::new(size),
            mutex: Semaphore::new(1),
        }
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn pop(&mut self) -> Result<T, Error> {
        interrupts::disable_then_execute(|| {
            self.mutex.wait()?;

            self.data_count.wait()?;
            let result = self.data.pop_front().ok_or_else(|| Error::ResourceNotAvailable);
            self.empty_count.signal()?;

            self.mutex.signal()?;

            result
        })
    }

    pub fn push(&mut self, item: T) -> Result<(), Error> {
        interrupts::disable_then_execute(|| {
            self.mutex.wait()?;

            self.empty_count.wait()?;
            self.data.push_back(item);
            self.data_count.signal()?;

            self.mutex.signal()?;

            Ok(())
        })
    }
}
