use crate::device::keyboard::{Key, ModifierState};

pub fn map_key(ascii: u8, modifier_state: ModifierState) -> Key {
    if modifier_state.is_uppercase() {
        map_upper(ascii)
    } else {
        Key::Ascii(ascii)
    }
}

fn map_upper(ascii: u8) -> Key {
    Key::Ascii(match ascii {
        0x61..=0x7A => ascii - 0x20,
        b'`' => b'~',
        b'1' => b'!',
        b'2' => b'@',
        b'3' => b'#',
        b'4' => b'$',
        b'5' => b'%',
        b'6' => b'^',
        b'7' => b'&',
        b'8' => b'*',
        b'9' => b'(',
        b'0' => b')',
        b'-' => b'_',
        b'=' => b'+',
        b'[' => b'{',
        b']' => b'}',
        b'\\' => b'|',
        b';' => b':',
        b'\'' => b'"',
        b',' => b'<',
        b'.' => b'>',
        b'/' => b'?',
        _ => ascii,
    })
}
