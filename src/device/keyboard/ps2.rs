use crate::device::keyboard::{Key, Key::*, KeyEvent, Modifier, ModifierState};

const EXTENDED_KEY_CODE: u8 = 0xE0;
const _KEY_RELEASE_CODE: u8 = 0xF0;

#[derive(Debug)]
enum ScancodeState {
    Start,
    Extended,
}

#[derive(Debug)]
enum Error {
    Unknown,
}

#[derive(Debug)]
pub struct Ps2Keyboard {
    modifier_state: ModifierState,
    scancode_state: ScancodeState,
}

impl Ps2Keyboard {
    pub const fn new() -> Ps2Keyboard {
        Ps2Keyboard {
            modifier_state: ModifierState {
                alt: (false, false),
                caps_lock: false,
                control: (false, false),
                num_lock: false,
                scroll_lock: false,
                shift: (false, false),
            },
            scancode_state: ScancodeState::Start,
        }
    }

    pub fn add_byte(&mut self, byte: u8) -> Option<KeyEvent> {
        match self.decode(byte) {
            Ok(key_event) => key_event,
            Err(_) => None,
        }
    }

    fn decode(&mut self, code: u8) -> Result<Option<KeyEvent>, Error> {
        match self.scancode_state {
            ScancodeState::Start => match code {
                EXTENDED_KEY_CODE => {
                    self.scancode_state = ScancodeState::Extended;
                    Ok(None)
                }
                0x80..=0xFF => {
                    let key = match_scancode(code - 0x80)?;
                    Ok(Some(KeyEvent::Released(key)))
                }
                _ => {
                    let key = match_scancode(code)?;
                    Ok(Some(KeyEvent::Pressed(key)))
                }
            },
            ScancodeState::Extended => {
                self.scancode_state = ScancodeState::Start;
                match code {
                    0x80..=0xFF => {
                        let key = match_extended_scancode(code - 0x80)?;
                        Ok(Some(KeyEvent::Released(key)))
                    }
                    _ => {
                        let key = match_extended_scancode(code)?;
                        Ok(Some(KeyEvent::Pressed(key)))
                    }
                }
            }
        }
    }

    pub fn process_keyevent(&mut self, key_event: KeyEvent) -> Option<Key> {
        let mut result = None;
        match key_event {
            KeyEvent::Pressed(key) => match key {
                Meta(Modifier::AltLeft) => self.modifier_state.alt.0 = true,
                Meta(Modifier::AltRight) => self.modifier_state.alt.1 = true,
                Meta(Modifier::CapsLock) => {
                    self.modifier_state.caps_lock = !self.modifier_state.caps_lock
                }
                Meta(Modifier::ControlLeft) => self.modifier_state.control.0 = true,
                Meta(Modifier::ControlRight) => self.modifier_state.control.1 = true,
                Meta(Modifier::NumLock) => {
                    self.modifier_state.num_lock = !self.modifier_state.num_lock
                }
                Meta(Modifier::ScrollLock) => {
                    self.modifier_state.scroll_lock = !self.modifier_state.scroll_lock
                }
                Meta(Modifier::ShiftLeft) => self.modifier_state.shift.0 = true,
                Meta(Modifier::ShiftRight) => self.modifier_state.shift.1 = true,
                Ascii(ascii) => {
                    result = Some(super::layout::us_std::map_key(ascii, self.modifier_state))
                }
            },
            KeyEvent::Released(key) => match key {
                Meta(Modifier::AltLeft) => self.modifier_state.alt.0 = false,
                Meta(Modifier::AltRight) => self.modifier_state.alt.1 = false,
                Meta(Modifier::CapsLock) => (),
                Meta(Modifier::ControlLeft) => self.modifier_state.control.0 = false,
                Meta(Modifier::ControlRight) => self.modifier_state.control.1 = false,
                Meta(Modifier::NumLock) => (),
                Meta(Modifier::ScrollLock) => (),
                Meta(Modifier::ShiftLeft) => self.modifier_state.shift.0 = false,
                Meta(Modifier::ShiftRight) => self.modifier_state.shift.1 = false,
                Ascii(_) => (),
            },
        };
        return result;
    }
}

fn match_scancode(scancode: u8) -> Result<Key, Error> {
    let idx = scancode as usize;
    match scancode {
        // ASCII Keys by keyboard row
        0x02..=0x0D => Ok(Ascii(b"1234567890-="[idx - 0x02])),
        0x10..=0x1B => Ok(Ascii(b"qwertyuiop[]"[idx - 0x10])),
        0x1E..=0x28 => Ok(Ascii(b"asdfghjkl;'"[idx - 0x1E])),
        0x2C..=0x35 => Ok(Ascii(b"zxcvbnm,./"[idx - 0x2C])),
        0x29 => Ok(Ascii(b'`')),
        0x2B => Ok(Ascii(b'\\')),

        // Non-modifiable ASCII keys
        0x01 => Ok(Ascii(0x1B)),  // escape
        0x0E => Ok(Ascii(0x8)),   // backspace
        0x0F => Ok(Ascii(b'\t')), // tab
        0x1C => Ok(Ascii(b'\n')), // newline
        0x39 => Ok(Ascii(b' ')),  // space

        // Meta keys
        0x1D => Ok(Meta(Modifier::ControlLeft)),
        0x2A => Ok(Meta(Modifier::ShiftLeft)),
        0x36 => Ok(Meta(Modifier::ShiftRight)),
        0x38 => Ok(Meta(Modifier::AltLeft)),
        0x3A => Ok(Meta(Modifier::CapsLock)),
        0x45 => Ok(Meta(Modifier::NumLock)),
        0x46 => Ok(Meta(Modifier::ScrollLock)),

        0xAA => Ok(Meta(Modifier::ShiftLeft)),
        0xB6 => Ok(Meta(Modifier::ShiftRight)),
        0x9D => Ok(Meta(Modifier::ControlLeft)),
        0xB8 => Ok(Meta(Modifier::AltLeft)),

        _ => Err(Error::Unknown),
    }
}

fn match_extended_scancode(scancode: u8) -> Result<Key, Error> {
    match scancode {
        0x1D => Ok(Meta(Modifier::ControlRight)),
        0x38 => Ok(Meta(Modifier::AltRight)),
        0x90..=0xED => match_extended_scancode(scancode - 0x80),
        _ => Err(Error::Unknown),
    }
}
