use crate::kprintln;
use conquer_once::spin::OnceCell;
use core::{pin::Pin, task::{Poll, Context}};
use crossbeam_queue::ArrayQueue;
use futures_util::{
    stream::{Stream, StreamExt},
    task::AtomicWaker,
};

pub mod uart_16550;

static SERIAL_QUEUE: OnceCell<ArrayQueue<u8>> = OnceCell::uninit();
static WAKER: AtomicWaker = AtomicWaker::new();

pub struct SerialStream {
    _private: (),
}

impl SerialStream {
    pub fn new() -> Self {
        SERIAL_QUEUE.try_init_once(|| ArrayQueue::new(1024))
            .expect("SerialStream::new should only be called once");
            SerialStream { _private: () }
    }
}

impl Stream for SerialStream {
    type Item = u8;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context) -> Poll<Option<u8>> {
        let queue = SERIAL_QUEUE.try_get().expect("not initialized");
        if let Ok(byte) = queue.pop() {
            return Poll::Ready(Some(byte));
        }

        WAKER.register(&cx.waker());
        match queue.pop() {
            Ok(byte) => {
                WAKER.take();
                Poll::Ready(Some(byte))
            }
            Err(crossbeam_queue::PopError) => Poll::Pending,
        }
    }
}

pub fn add_byte(byte: u8) {
    if let Ok(queue) = SERIAL_QUEUE.try_get() {
        if let Err(_) = queue.push(byte) {
            kprintln!("WARNING: serial queue full; dropping keyboard input");
        } else {
            WAKER.wake();
        }
    } else {
        kprintln!("WARNING: serial queue uninitialized");
    }
}

pub async fn print_serial() {
    let mut bytes = SerialStream::new();

    while let Some(byte) = bytes.next().await {
        serial_print!("{}", byte as char);
    }
}