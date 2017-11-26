enum Error {
    TryAgain(str),
}

pub type Result<T> = result::Result<T, Error>;
