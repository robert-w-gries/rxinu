use devices::ps2_keyboard::{KeyType, Modifier};

pub enum ScanCodeSet {
    Set1,
    Set2,
}

pub fn get_scancode_key(scancode: u8, scancode_set: ScanCodeSet) -> Option<KeyType> {
    match scancode_set {
        ScanCodeSet::Set1 => match_scancode_set1(scancode),
        ScanCodeSet::Set2 => match_scancode_set2(scancode),
        _ => None,
    }
}

fn match_scancode_set1(scancode: u8) -> Option<KeyType> {
    let idx = scancode as usize;
    let ascii = match scancode {
        // ASCII Keys
        0x01 ... 0x0E => Some(b"\x1B1234567890-=\0x02"[idx-0x01]),
        0x0F ... 0x1C => Some(b"\tqwertyuiop[]\r"[idx-0x0F]),
        0x1E ... 0x28 => Some(b"asdfghjkl;'"[idx-0x1E]),
        0x2C ... 0x35 => Some(b"zxcvbnm,./"[idx-0x2C]),
        0x39 => Some(b' '),
        _ => None,
    };

    if let Some(c) = ascii {
        return Some(KeyType::Ascii(c as char));
    }

    let modifier = match scancode {
        0x1D => Some(Modifier::ControlLeft(true)),
        0x2A => Some(Modifier::ShiftLeft(true)),
        0x36 => Some(Modifier::ShiftRight(true)),
        0x38 => Some(Modifier::AltLeft(true)),
        0x3A => Some(Modifier::CapsLock),
        0x45 => Some(Modifier::NumLock),
        0x46 => Some(Modifier::ScrollLock),
        //0x9D => Some(Modifier::ControlLeft(false)),
        //0xAA => Some(Modifier::ShiftLeft(false)),
        //0xB6 => Some(Modifier::ShiftRight(false)),
        //0xB8 => Some(Modifier::AltLeft(false)),
        _ => None
    };

    if let Some(m) = modifier {
        Some(KeyType::KeyState(m))
    } else {
        None
    }
}

fn match_scancode_set2(scancode: u8) -> Option<KeyType> {
    let ascii = match scancode {
        0x1C => Some('a'),
        0x32 => Some('b'),
        0x21 => Some('c'),
        0x23 => Some('d'),
        0x24 => Some('e'),
        0x2B => Some('f'),
        0x34 => Some('g'),
        0x33 => Some('h'),
        0x43 => Some('i'),
        0x3B => Some('j'),
        0x42 => Some('k'),
        0x4B => Some('l'),
        0x3A => Some('m'),
        0x31 => Some('n'),
        0x44 => Some('o'),
        0x4D => Some('p'),
        0x15 => Some('q'),
        0x2D => Some('r'),
        0x1B => Some('s'),
        0x2C => Some('t'),
        0x3C => Some('u'),
        0x2A => Some('v'),
        0x1D => Some('w'),
        0x22 => Some('x'),
        0x35 => Some('y'),
        0x1A => Some('z'),
        0x45 => Some('0'),
        0x16 => Some('1'),
        0x1E => Some('2'),
        0x26 => Some('3'),
        0x25 => Some('4'),
        0x2E => Some('5'),
        0x36 => Some('6'),
        0x3D => Some('7'),
        0x3E => Some('8'),
        0x46 => Some('9'),
        0x0E => Some('`'),
        0x4E => Some('-'),
        0x55 => Some('='),
        0x5D => Some('\\'),
        0x29 => Some(' '),
        0x54 => Some('['),
        0x5B => Some(']'),
        0x4C => Some(';'),
        0x52 => Some('\''),
        0x41 => Some(','),
        0x49 => Some('.'),
        0x4A => Some('/'),
        0x7C => Some('*'),   // keypad
        0x7B => Some('-'),   // keypad
        0x79 => Some('+'),   // keypad
        _ => return None,
    };

    if let Some(c) = ascii {
        return Some(KeyType::Ascii(c));
    }

    let modifier = match scancode {
        0x11 => Some(Modifier::AltLeft(true)),
        0x12 => Some(Modifier::ShiftLeft(true)),
        0x14 => Some(Modifier::ControlLeft(true)),
        0x59 => Some(Modifier::ShiftRight(true)),
        0x58 => Some(Modifier::CapsLock),
        0x7E => Some(Modifier::ScrollLock),
        0x77 => Some(Modifier::NumLock),
        //0xE0 => Some(Modifier::ControRight(false)),
        //0xB8 => Some(Modifier::AltRight(false)),
        _ => None
    };

    if let Some(m) = modifier {
        Some(KeyType::KeyState(m))
    } else {
        None
    }
}
