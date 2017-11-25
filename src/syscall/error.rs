enum Error {
    Again(str),
}

pub type Result<T> = result::Result<T, Error>;
