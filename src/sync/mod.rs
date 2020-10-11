pub mod irq;
//pub mod semaphore;

pub use self::irq::{IrqGuard, IrqLock, IrqSpinLock};
//pub use self::semaphore::Semaphore;
