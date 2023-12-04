use hashbrown::{HashMap, HashSet};
use itertools::Itertools;
use textwrap::dedent;

use crate::utils::get_input_for_day;

use super::helpers::{print_solutions, Example, Solution};

pub fn print_solutions_day4() {
    let day: u32 = 4;
    let example = Example {
        input_data: dedent(
            "
            Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
            Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
            Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
            Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
            Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
            Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11
        ",
        ),
        expected_part1: 13,
        expected_part2: 30,
    };
    let solution = Solution {
        input_data: get_input_for_day(day),
        get_solution_part1: get_solution_day4_part1,
        get_solution_part2: get_solution_day4_part2,
    };
    print_solutions(day, example, solution);
}

fn get_solution_day4_part1(input_data_raw: &str) -> u32 {
    let mut solution: u32 = 0;

    for line in input_data_raw.lines() {
        let winning_numbers = get_winning_numbers(line);
        if winning_numbers.len() > 0 {
            solution += (2 as u32).pow((winning_numbers.len() - 1) as u32);
        }
    }
    return solution;
}

fn get_solution_day4_part2(input_data_raw: &str) -> u32 {
    let mut n_cards: HashMap<usize, u32> = HashMap::new();

    for (idx, line) in input_data_raw.lines().enumerate() {
        if line.is_empty() {
            continue;
        }

        if !n_cards.contains_key(&idx) {
            n_cards.insert(idx, 1);
        }

        // Get indexes to copy.
        let winning_numbers = get_winning_numbers(line);
        let copy_range_start = idx + 1;
        let copy_range_end = copy_range_start + winning_numbers.len();

        // Number of copies of this card (gives number of additional copies of ones below this)
        let n_copies = *n_cards.get(&idx).unwrap_or(&1);

        // Increment the number of copies for cards under this one
        for copy_idx in copy_range_start..copy_range_end {
            let old_val: &u32 = n_cards.get(&copy_idx).unwrap_or(&1);
            n_cards.insert(copy_idx, old_val + n_copies);
        }
    }

    let solution = n_cards.values().sum();

    return solution;
}

fn get_winning_numbers(line: &str) -> HashSet<u32> {
    let (_, numbers) = line.split(":").collect_tuple().unwrap_or(("", ""));

    // Extract numbers
    let (my_numbers, their_numbers) = numbers.split("|").collect_tuple().unwrap_or(("", ""));

    let my_numbers_numeric: HashSet<u32> = my_numbers
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    let their_numbers_numeric: HashSet<u32> = their_numbers
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    // Winning numbers given by overlaps
    let winning_numbers: HashSet<u32> = my_numbers_numeric
        .intersection(&their_numbers_numeric)
        .map(|x| x.to_owned())
        .collect();

    return winning_numbers;
}
