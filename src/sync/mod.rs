pub mod irq;
pub mod semaphore;

pub use self::irq::{IrqLock, IrqSpinLock};
pub use self::semaphore::Semaphore;
