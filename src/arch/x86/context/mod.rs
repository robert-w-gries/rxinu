#[cfg(target_arch = "x86")]
mod bits32;
#[cfg(target_arch = "x86_64")]
mod bits64;

#[cfg(target_arch = "x86")]
pub use self::bits32::*;
#[cfg(target_arch = "x86_64")]
pub use self::bits64::*;

global_asm!(include_str!("context_switch.asm"));
