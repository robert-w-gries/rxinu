#[derive(Debug)]
pub enum Error {
    BadPid,
    InvalidOperation,
    ResourceNotAvailable,
    TryAgain,
}
