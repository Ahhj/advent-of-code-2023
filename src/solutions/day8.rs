use textwrap::dedent;

use crate::helpers::{print_solutions, Example, Solution};
use crate::utils::get_input_for_day;

pub fn print_solutions_day8() {
    let day: u32 = 8;
    let example = Example {
        input_data: dedent(
            "
        ",
        ),
        expected_part1: 0,
        expected_part2: 0,
    };
    let solution = Solution {
        input_data: get_input_for_day(day),
        get_solution_part1,
        get_solution_part2,
    };
    print_solutions(day, example, solution);
}

fn get_solution_part1(input_data_raw: &str) -> u32 {
    0
}

fn get_solution_part2(input_data_raw: &str) -> u32 {
    0
}
