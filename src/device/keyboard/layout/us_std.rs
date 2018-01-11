pub fn map_to_upper(lower: u8) -> u8 {
    if lower.is_ascii_lowercase() {
        lower.to_ascii_uppercase()
    } else {
        match lower {
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
            _ => 0x0,
        }
    }
}
