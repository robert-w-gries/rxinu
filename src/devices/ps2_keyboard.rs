use alloc::string::{String, ToString};
use devices::keyboard::{self, ScanCodeSet};
use spin::Mutex;

const DEFAULT_SCANCODE_SET: ScanCodeSet = ScanCodeSet::Set2;

#[derive(Debug)]
struct KeyPair {
    left: bool,
    right: bool,
}

impl KeyPair {
    const fn new() -> Self {
        KeyPair { left: false, right: false }
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
        if 'a' <= ascii && ascii <= 'z' {
            if self.is_uppercase() {
                return ascii.to_uppercase().collect();
            }
        }
        ascii.to_string()
    }

    fn update(&mut self, m: Modifier) {
        match m {
            Modifier::AltLeft(state) => self.alt.left = state,
            Modifier::AltRight(state) => self.alt.right = state,
            Modifier::CapsLock => self.caps_lock = !self.caps_lock,
            Modifier::ControlLeft(state) => self.control.left = state,
            Modifier::ControlRight(state) => self.control.right = state,
            Modifier::NumLock => self.num_lock = !self.num_lock,
            Modifier::ScrollLock => self.scroll_lock = !self.scroll_lock,
            Modifier::ShiftLeft(state) => self.shift.left = state,
            Modifier::ShiftRight(state) => self.shift.right = state,
            _ => return,
        }
    }
}

enum KeyEvent {
    Pressed,
    SolidState,
    Released,
}

pub enum KeyType {
    Ascii(char),
    KeyState(Modifier),
}

struct Keypress {
    /// Select a scan code set. Scan code set 2 is the default for most cases
    scancode_set: ScanCodeSet,

    /// Modifiers to ascii keys
    modifiers: ModifierState,

    /// Indicates whether key is pressed or released
    event: KeyEvent,
}

/// Our global keyboard state, protected by a mutex.
static KEYPRESS: Mutex<Keypress> = Mutex::new(Keypress{
    scancode_set: ScanCodeSet::Set2,
    modifiers: ModifierState::new(),
    event: KeyEvent::SolidState,
});

/// Try to read a single input character
pub fn parse_key(scancode: u8) {
    if let Some(key) = keyboard::get_scancode_key(scancode, DEFAULT_SCANCODE_SET) {
        match key {
            KeyType::KeyState(modifier) => KEYPRESS.lock().modifiers.update(modifier),
            KeyType::Ascii(letter) => print_key(letter),
        }
    }
}

fn print_key(character: char) {
    // The `as char` converts our ASCII data to Unicode, which is
    // correct as long as we're only using 7-bit ASCII.
    let final_char = KEYPRESS.lock().modifiers.apply_to(character);
    match final_char.as_ref() {
        "\n" => println!(""),
        character => print!("{}", final_char),
    }
}
