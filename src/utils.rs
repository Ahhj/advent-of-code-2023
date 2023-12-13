use itertools::Itertools;
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

/*
Rolling difference between elements
NOTE: this can result in overflow errors if say you provide a u32 and the
difference becomes negative!
*/
pub fn diff<'a, T>(x: &'a Vec<T>) -> Vec<T>
where
    &'a T: std::ops::Sub<&'a T, Output = T>,
{
    let pairs: Vec<(&T, &T)> = x.iter().zip(x.iter().skip(1)).collect_vec();
    let x_diff: Vec<T> = pairs.iter().map(|(curr, next)| *curr - *next).collect_vec();
    x_diff
}
