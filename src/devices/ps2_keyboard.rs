use alloc::Vec;
use alloc::string::{String, ToString};
use devices::{keyboard, ps2_controller_8042};
use spin::Mutex;

#[derive(Debug)]
struct KeyPair {
    left: bool,
    right: bool,
}

impl KeyPair {
    const fn new() -> Self {
        KeyPair {
            left: false,
            right: false,
        }
    }

    fn is_pressed(&self) -> bool {
        self.left || self.right
    }
}

pub enum Modifier {
    AltLeft(bool),
    AltRight(bool),
    CapsLock,
    ControlLeft(bool),
    ControlRight(bool),
    NumLock,
    ScrollLock,
    ShiftLeft(bool),
    ShiftRight(bool),
}

/// All of our supported keyboard modifiers.
#[derive(Debug)]
struct ModifierState {
    alt: KeyPair,
    caps_lock: bool,
    control: KeyPair,
    num_lock: bool,
    scroll_lock: bool,
    shift: KeyPair,
}

impl ModifierState {
    const fn new() -> Self {
        ModifierState {
            alt: KeyPair::new(),
            caps_lock: false,
            control: KeyPair::new(),
            num_lock: false,
            scroll_lock: false,
            shift: KeyPair::new(),
        }
    }

    fn is_uppercase(&self) -> bool {
        self.shift.is_pressed() ^ self.caps_lock
    }

    /// Apply all of our modifiers to an ASCII character, and return a new
    /// ASCII character.
    fn apply_to(&self, ascii: char) -> String {
        if self.is_uppercase() {
            ascii.to_uppercase().collect()
        } else {
            ascii.to_string()
        }
    }

    fn update(&mut self, m: Modifier) {
        use self::Modifier::*;

        match m {
            AltLeft(state) => self.alt.left = state,
            AltRight(state) => self.alt.right = state,
            CapsLock => self.caps_lock = !self.caps_lock,
            ControlLeft(state) => self.control.left = state,
            ControlRight(state) => self.control.right = state,
            NumLock => self.num_lock = !self.num_lock,
            ScrollLock => self.scroll_lock = !self.scroll_lock,
            ShiftLeft(state) => self.shift.left = state,
            ShiftRight(state) => self.shift.right = state,
        }
    }
}

pub enum KeyEvent {
    Pressed(Key),
    Released(Key),
}

pub enum Key {
    Ascii(u8),
    Meta(Modifier),
    LowerAscii(u8),
}

static STATE: Mutex<ModifierState> = Mutex::new(ModifierState::new());

/// Get all bytes from keyboard and translate to key
pub fn parse_key(scancode: u8) {
    let byte_sequence: u64 = retrieve_bytes(scancode);
    if let Some(key) = keyboard::get_key(byte_sequence) {
        match key {
            Key::Ascii(k) => print_char(k as char),
            Key::Meta(modifier) => STATE.lock().update(modifier),
            Key::LowerAscii(byte) => print_str(STATE.lock().apply_to(byte as char)),
        }
    }
}

/// Keep reading bytes until sequence is finished and combine bytes into an integer
fn retrieve_bytes(scancode: u8) -> u64 {
    let mut byte_sequence: Vec<u8> = vec![scancode];

    // if byte is start of sequence, start reading bytes until end of sequence
    // TODO: Design system that reads more than two bytes
    if scancode == 0xE0 || scancode == 0xE1 {
        let check_byte: u8 = ps2_controller_8042::key_read();
        if let Some(byte) = keyboard::is_special_key(check_byte) {
            byte_sequence.push(byte);
        }
    }

    byte_sequence
        .iter()
        .rev()
        .fold(0, |acc, &b| (acc << 1) + b as u64)
}

fn print_str(s: String) {
    kprint!("{}", s);
}

fn print_char(byte: char) {
    match byte {
        '\n' | ' ' | '\t' => kprint!("{}", byte),
        _ => (),
    }
}
