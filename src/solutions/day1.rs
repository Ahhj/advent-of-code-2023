use crate::utils::{get_first_char, get_input_for_day, get_last_char};
use regex::Regex;
use textwrap::dedent;

pub fn print_solutions_day1() {
    /*
    Part 1
    Run with example data
    */
    let example_data_raw_part1: String = dedent(
        "
        1abc2
        pqr3stu8vwx
        a1b2c3d4e5f
        treb7uchet
    ",
    );
    let example_expected_part1: u32 = 142;
    let example_solution_part1: u32 = get_solution_day1_part1(&example_data_raw_part1);
    assert_eq!(example_solution_part1, example_expected_part1);

    // Run with real data
    let input_data_raw: String = get_input_for_day(1);
    let solution_part1: u32 = get_solution_day1_part1(&input_data_raw);
    println!("Day 1, part 1: {}", solution_part1);

    // Part 2
    // Run with example data
    let example_data_raw_part2: String = dedent(
        "
        two1nine
        eightwothree
        abcone2threexyz
        xtwone3four
        4nineeightseven2
        zoneight234
        7pqrstsixteen
    ",
    );
    let example_expected_part2: u32 = 281;
    let example_solution_part2: u32 = get_solution_day1_part2(&example_data_raw_part2);
    assert_eq!(example_expected_part2, example_solution_part2);

    let solution_part2: u32 = get_solution_day1_part2(&input_data_raw);
    println!("Day 1, part 2: {}", solution_part2);
}

fn get_solution_day1_part2(input_data: &str) -> u32 {
    let prepped_data: String = replace_spelled_digits(&input_data);
    let solution: u32 = get_solution_day1_part1(&prepped_data);
    return solution;
}

fn get_solution_day1_part1(input_data: &str) -> u32 {
    let cal_vals: Vec<u32> = input_data.lines().map(get_calibration_value).collect();
    let solution: u32 = cal_vals.iter().sum();
    return solution;
}

/*
Extracts first and last numbers in a string and uses then to create a
2-digit number {first}{last} e.g. '1abc2' -> 12.
*/
fn get_calibration_value(line: &str) -> u32 {
    // Get numbers from strings
    let digit_re = Regex::new(r"[A-Za-z]+").unwrap();
    let numbers_only: String = digit_re.replace_all(&line, "").to_string();

    // Pick first and last
    let first: String = get_first_char(&numbers_only).to_string();
    let last: String = get_last_char(&numbers_only).to_string();

    let cal_val: u32 = (first + &last).parse().unwrap_or(0);
    return cal_val;
}

fn replace_spelled_digits(input_data: &str) -> String {
    let mut prepped_data: String = input_data.to_owned();
    let spelled_digits: [&str; 9] = [
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];

    for (i, val) in spelled_digits.iter().enumerate() {
        // Replace by inserting the numeric value while keeping the original
        // value either side. This ensures we don't interfere with other replacements
        // and prevents us needing to explicitly handle all edge cases!
        let val_numeric: usize = i + 1;
        let repl: &str = &format!("{val}{val_numeric}{val}");

        prepped_data = prepped_data.replace(val, repl);
    }

    return prepped_data;
}
