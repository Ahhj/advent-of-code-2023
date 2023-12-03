pub fn get_first_char(line: &str) -> char {
    return line.chars().nth(0).unwrap_or(' ');
}

pub fn get_last_char(line: &str) -> char {
    return line.chars().rev().nth(0).unwrap_or(' ');
}
