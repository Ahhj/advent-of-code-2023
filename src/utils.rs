use std::fs;

pub fn get_first_char(line: &str) -> char {
    return line.chars().nth(0).unwrap_or(' ');
}

pub fn get_last_char(line: &str) -> char {
    return line.chars().rev().nth(0).unwrap_or(' ');
}

pub fn get_input_for_day(day: u32) -> String {
    let filepath: &str = &format!("data/input/day{day}.txt");
    let input_data_raw: String = fs::read_to_string(filepath)
        .expect(&format!("Should have been able to read file {filepath}"));
    return input_data_raw;
}
