use core::cmp::PartialEq;

pub trait Io {
    type Value: Copy + PartialEq;

    fn read(&self) -> Self::Value;
    fn write(&mut self, value: Self::Value);
}

pub struct ReadOnly<I: Io> {
        inner: I
}

impl<I: Io> ReadOnly<I> {
    pub const fn new(inner: I) -> ReadOnly<I> {
        ReadOnly {
            inner: inner
        }
    }

    pub fn read(&self) -> I::Value {
        self.inner.read()
    }
}

pub struct WriteOnly<I: Io> {
        inner: I
}

impl<I: Io> WriteOnly<I> {
    pub const fn new(inner: I) -> WriteOnly<I> {
        WriteOnly {
            inner: inner
        }
    }

    pub fn write(&mut self, value: I::Value) {
        self.inner.write(value)
    }
}
