use textwrap::dedent;

use crate::utils::get_input_for_day;

use super::helpers::{print_solutions, Example, Solution};

pub fn print_solutions_day5() {
    let day: u32 = 5;
    let example = Example {
        input_data: dedent(""),
        expected_part1: 0,
        expected_part2: 0,
    };
    let solution = Solution {
        input_data: get_input_for_day(day),
        get_solution_part1: Box::new(get_solution_day5_part1),
        get_solution_part2: Box::new(get_solution_day5_part2),
    };
    print_solutions(day, example, solution);
}

fn get_solution_day5_part1(input_data_raw: &str) -> u32 {
    return 0;
}

fn get_solution_day5_part2(input_data_raw: &str) -> u32 {
    return 0;
}
