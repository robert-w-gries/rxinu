use crate::{kprint, kprintln};
use conquer_once::spin::OnceCell;
use core::{pin::Pin, task::{Poll, Context}};
use crossbeam_queue::ArrayQueue;
use futures_util::{
    stream::{Stream, StreamExt},
    task::AtomicWaker,
};

pub mod layout;
pub mod ps2;

static SCANCODE_QUEUE: OnceCell<ArrayQueue<u8>> = OnceCell::uninit();
static WAKER: AtomicWaker = AtomicWaker::new();

#[derive(Debug)]
pub enum KeyEvent {
    Pressed(Key),
    Released(Key),
}

#[derive(Debug)]
pub enum Key {
    Ascii(u8),
    Meta(Modifier),
}

#[derive(Debug)]
pub enum Modifier {
    AltLeft,
    AltRight,
    CapsLock,
    ControlLeft,
    ControlRight,
    NumLock,
    ScrollLock,
    ShiftLeft,
    ShiftRight,
}

#[derive(Clone, Copy, Debug)]
pub struct ModifierState {
    alt: (bool, bool),
    caps_lock: bool,
    control: (bool, bool),
    num_lock: bool,
    scroll_lock: bool,
    shift: (bool, bool),
}

impl ModifierState {
    pub fn is_uppercase(&self) -> bool {
        (self.shift.0 | self.shift.1) ^ self.caps_lock
    }
}

pub struct ScancodeStream {
    _private: (),
}

impl ScancodeStream {
    pub fn new() -> Self {
        SCANCODE_QUEUE.try_init_once(|| ArrayQueue::new(1024))
            .expect("ScancodeStream::new should only be called once");
        ScancodeStream { _private: () }
    }
}

impl Stream for ScancodeStream {
    type Item = u8;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context) -> Poll<Option<u8>> {
        let queue = SCANCODE_QUEUE.try_get().expect("not initialized");
        if let Ok(scancode) = queue.pop() {
            return Poll::Ready(Some(scancode));
        }

        WAKER.register(&cx.waker());
        match queue.pop() {
            Ok(scancode) => {
                WAKER.take();
                Poll::Ready(Some(scancode))
            }
            Err(crossbeam_queue::PopError) => Poll::Pending,
        }
    }
}

pub fn add_scancode(scancode: u8) {
    if let Ok(queue) = SCANCODE_QUEUE.try_get() {
        if let Err(_) = queue.push(scancode) {
            kprintln!("WARNING: scancode queue full; dropping keyboard input");
        } else {
            WAKER.wake();
        }
    } else {
        kprintln!("WARNING: scancode queue uninitialized");
    }
}

pub async fn print_keypresses() {
    let mut scancodes = ScancodeStream::new();
    let mut keyboard = ps2::Ps2Keyboard::new();

    while let Some(scancode) = scancodes.next().await {
        if let Some(key_event) = keyboard.add_byte(scancode) {
            if let Some(Key::Ascii(key)) = keyboard.process_keyevent(key_event) {
                kprint!("{}", key as char);
            }
        }
    }
}