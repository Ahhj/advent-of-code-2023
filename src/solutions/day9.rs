use itertools::Itertools;
use textwrap::dedent;

use crate::helpers::{Example, Solution};
use crate::utils::{diff, get_input_for_day};

pub fn print_solutions_day9() {
    let day: u32 = 9;
    let example = Example {
        input_data: dedent(
            "
            0 3 6 9 12 15
            1 3 6 10 15 21
            10 13 16 21 30 45
        ",
        ),
        expected_part1: 114,
        expected_part2: 2,
    };
    let solution = Solution {
        input_data: get_input_for_day(day),
        get_solution_part1,
        get_solution_part2,
    };
    solution.print_solutions(day, example);
}

fn get_solution_part1(input_data_raw: &str) -> u64 {
    get_solution(input_data_raw, false)
}

fn get_solution_part2(input_data_raw: &str) -> u64 {
    get_solution(input_data_raw, true)
}

fn get_solution(input_data_raw: &str, reverse: bool) -> u64 {
    let mut result = 0;
    for line in input_data_raw.trim().lines() {
        let values_iter = line.split_whitespace().map(|x| x.parse().unwrap());
        let values: Vec<i64>;
        if reverse {
            values = values_iter.collect();
        } else {
            values = values_iter.rev().collect();
        }
        let next_val: i64 = get_next_value(&values);
        result += next_val
    }
    result as u64
}

fn get_next_value(values: &Vec<i64>) -> i64 {
    // Differences at each level
    let max_depth = 1000;
    let mut vals_diff: Vec<Vec<i64>> = vec![diff(&values)];
    for depth in 1..max_depth {
        let prev_diff = vals_diff.get(depth - 1).unwrap();
        match prev_diff.iter().all_equal_value().ok() {
            Some(&0) => break,
            _ => {
                vals_diff.insert(depth, diff(prev_diff));
            }
        }
    }

    // Add to value from lowest level to upper level.
    let mut next_val: i64 = 0;
    for level_diffs in vals_diff.iter().rev() {
        next_val += level_diffs.first().unwrap_or(&0);
    }
    next_val += values.first().unwrap_or(&0);

    return next_val;
}
