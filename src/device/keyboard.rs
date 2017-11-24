use device::ps2_keyboard::{Key, KeyEvent};
use device::ps2_keyboard::Key::*;
use device::ps2_keyboard::Modifier::*;

macro_rules! key_press {
    ($x:expr) => (Some(KeyEvent::Pressed($x)))
}

macro_rules! key_release {
    ($x:expr) => (Some(KeyEvent::Released($x)))
}

pub fn get_key(scancode: u64) -> Option<Key> {
    match get_key_event(scancode) {
        Some(KeyEvent::Pressed(key)) => Some(key),
        Some(KeyEvent::Released(key)) => Some(key),
        _ => None,
    }
}

pub fn get_key_event(scancode: u64) -> Option<KeyEvent> {
    match_scancode(scancode)
}

pub fn is_special_key(byte: u8) -> Option<u8> {
    match byte {
        0x5B | 0xDB => Some(byte), // L GUI
        0x1D | 0x9D => Some(byte), // R CTRL
        0x5C | 0xDC => Some(byte), // R GUI
        0x38 | 0xB8 => Some(byte), // R ALT
        0x5D | 0xDD => Some(byte), // APPS
        0x52 | 0xD2 => Some(byte), // INSERT
        0x47 | 0x97 => Some(byte), // HOME
        0x49 | 0xC9 => Some(byte), // PG UP
        0x53 | 0xD3 => Some(byte), // DELETE
        0x4F | 0xCF => Some(byte), // END
        0x51 | 0xD1 => Some(byte), // PG DN
        0x48 | 0xC8 => Some(byte), // U ARROW
        0x4B | 0xCB => Some(byte), // L ARROW
        0x50 | 0xD0 => Some(byte), // D ARROW
        0x4D | 0xCD => Some(byte), // R ARROW
        0x35 | 0xB5 => Some(byte), // Keypad '/'
        0x1C | 0x9C => Some(byte), // Keypad '\n'
        _ => None,
    }
}

fn match_scancode(scancode: u64) -> Option<KeyEvent> {
    let idx = scancode as usize;
    match scancode {
        // ASCII Keys by keyboard row
        0x02...0x0D => key_press!(LowerAscii(b"1234567890-="[idx - 0x02])),
        0x10...0x1B => key_press!(LowerAscii(b"qwertyuiop[]"[idx - 0x10])),
        0x1E...0x28 => key_press!(LowerAscii(b"asdfghjkl;'"[idx - 0x1E])),
        0x2C...0x35 => key_press!(LowerAscii(b"zxcvbnm,./"[idx - 0x2C])),
        0x29 => key_press!(LowerAscii((b'`'))),
        0x2B => key_press!(LowerAscii((b'\\'))),

        // Non-modifiable ASCII keys
        0x01 => key_press!(Ascii(0x1B)),  // escape
        0x0E => key_press!(Ascii(0x8)),   // backspace
        0x0F => key_press!(Ascii(b'\t')), // tab
        0x1C => key_press!(Ascii(b'\n')), // newline
        0x39 => key_press!(Ascii(b' ')),  // space

        // Meta keys
        0x1D => key_press!(Meta(ControlLeft(true))),
        0xE01D => key_press!(Meta(ControlRight(true))),
        0x2A => key_press!(Meta(ShiftLeft(true))),
        0x36 => key_press!(Meta(ShiftRight(true))),
        0x38 => key_press!(Meta(AltLeft(true))),
        0xE038 => key_press!(Meta(AltRight(false))),
        0x3A => key_press!(Meta(CapsLock)),
        0x45 => key_press!(Meta(NumLock)),
        0x46 => key_press!(Meta(ScrollLock)),

        0xAA => key_release!(Meta(ShiftLeft(false))),
        0xB6 => key_release!(Meta(ShiftRight(false))),
        0x9D => key_release!(Meta(ControlLeft(false))),
        0xE09D => key_release!(Meta(ControlRight(false))),
        0xB8 => key_release!(Meta(AltLeft(false))),
        0xE0B8 => key_release!(Meta(AltRight(false))),

        _ => None,
    }
}
