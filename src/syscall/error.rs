use alloc::String;
use core::result;

pub enum Error {
    TryAgain(String),
}
