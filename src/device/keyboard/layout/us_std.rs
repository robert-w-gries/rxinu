use alloc::Vec;

pub fn map_to_upper(lower: char) -> Vec<char> {
    if lower.is_alphabetic() {
        lower.to_uppercase().collect()
    } else {
        let upper = match lower {
            '`' => '~',
            '1' => '!',
            '2' => '@',
            '3' => '#',
            '4' => '$',
            '5' => '%',
            '6' => '^',
            '7' => '&',
            '8' => '*',
            '9' => '(',
            '0' => ')',
            '-' => '_',
            '=' => '+',
            '[' => '{',
            ']' => '}',
            '\\' => '|',
            ';' => ':',
            '\'' => '"',
            ',' => '<',
            '.' => '>',
            '/' => '?',
            _ => 0x0 as char
        };

        vec![upper]
    }
}
