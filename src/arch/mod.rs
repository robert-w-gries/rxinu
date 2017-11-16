#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
#[macro_use]
pub mod x86;

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
pub use self::x86::init;
