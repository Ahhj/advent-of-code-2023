use itertools::Itertools;
use num_integer::binomial;
use textwrap::dedent;

use crate::helpers::{Example, Solution};
use crate::utils::get_input_for_day;

pub fn print_solutions_day12() {
    let day: u32 = 12;
    let example = Example {
        input_data: dedent(
            "
            ???.### 1,1,3
            .??..??...?##. 1,1,3
            ?#?#?#?#?#?#?#? 1,3,1,6
            ????.#...#... 4,1,1
            ????.######..#####. 1,6,5
            ?###???????? 3,2,1
        ",
        ),
        expected_part1: 0,
        expected_part2: 0, // Not actual example because different params used.
    };
    let solution = Solution {
        input_data: get_input_for_day(day),
        get_solution_part1,
        get_solution_part2,
    };
    solution.print_solutions(day, example);
}

fn get_solution_part1(input_data_raw: &str) -> u64 {
    0
}

fn get_solution_part2(input_data_raw: &str) -> u64 {
    0
}
