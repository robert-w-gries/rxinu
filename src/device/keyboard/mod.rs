use spin::Mutex;

macro_rules! key_press {
    ($x:expr) => {
        Some(KeyEvent::Pressed($x))
    };
}

macro_rules! key_release {
    ($x:expr) => {
        Some(KeyEvent::Released($x))
    };
}

pub mod layout;
pub mod ps2;

pub static STATE: Mutex<ModifierState> = Mutex::new(ModifierState::new());

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
pub struct ModifierState {
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

    /// Apply all of our modifiers to character and convert to String
    pub fn apply_to(&self, ascii: u8) -> u8 {
        if self.is_uppercase() {
            layout::us_std::map_to_upper(ascii)
        } else {
            ascii
        }
    }

    pub fn update(&mut self, m: Modifier) {
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
