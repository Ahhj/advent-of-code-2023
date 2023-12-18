use itertools::Itertools;
use num_integer::binomial;
use regex::bytes::Regex;
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
    let mut solution: u64 = 0;
    for line in input_data_raw.trim().lines() {
        let (row_raw, groups_raw) = line.split_whitespace().collect_tuple().unwrap();
        let row = row_raw.as_bytes().to_vec();
        let groups: Vec<u64> = groups_raw
            .split(",")
            .map(|x| x.parse().unwrap())
            .collect_vec();

        let mut new_row = row.clone();
        let groups_re = generate_groups_regex(&groups);

        for (index, &char) in row.iter().enumerate() {
            if char != b'?' {
                continue;
            } else {
                new_row[index] = b'#';
                let could_be_hash = groups_re.is_match(&new_row);

                new_row[index] = b'.';
                let could_be_dot = groups_re.is_match(&new_row);

                if could_be_hash && could_be_dot {
                    new_row[index] = b'?';
                    continue;
                } else if could_be_dot {
                    // Can only be dot
                    new_row[index] = b'.';
                } else if could_be_hash {
                    // Can only be hash
                    new_row[index] = b'#';
                }
            }
        }

        let cleaned_row = String::from_utf8_lossy(&new_row);

        let mut filtered_groups: Vec<&u64> = vec![];

        if !new_row.contains(&b'?') {
            println!("All sites populated");
        } else {
            let max_length = row.len();
            for (index, group) in groups.iter().enumerate() {
                let groups_left = &groups[0..index];
                let n_groups_left: usize = groups_left.iter().len();
                let min_left: usize = (groups_left.iter().sum::<u64>() as usize) + n_groups_left;

                let groups_right = &groups[index + 1..groups.len()];
                let n_groups_right: usize = groups_right.iter().len();
                let min_right: usize = (groups_right.iter().sum::<u64>() as usize) + n_groups_right;
                //{{{min_left}},{{max_length}}}

                let range_left = format!("{min_left},{max_length}");
                let range_right = format!("{min_right},{max_length}");
                let group_re = format!(
                    "^[.?#]{{{range_left}}}[.]+([#]{{{group}}})[.]+[.?#]{{{range_right}}}$"
                );
                Regex::new(&group_re).expect("generated regex should be valid");

                let is_match = Regex::new(&group_re).expect("fine").is_match(&new_row);

                if !is_match {
                    filtered_groups.push(&group);
                }
            }
        }

        if filtered_groups.len() == 0 {
            // No spaces left to allocate
            solution += 1;
            continue;
        } else {
            // k_buckets = #{groups after filtering} + 1
            // n_items = #{? after cleaning} - {sum groups after filtering} + #{missing '.' to insert}
            let num_filtered_groups = filtered_groups.len() as u64;
            let sum_filtered_groups = filtered_groups.iter().fold(0, |acc, &x| acc + *x);
            let num_questions = new_row
                .iter()
                .filter(|&b| b == &b'?')
                .fold(0, |acc, _| acc + 1);

            // Need number of periods that need to be fixed that arent yet
            // The total number should be ({number of original groups} - 1) since there needs to be a period between each
            // The number allocated right now is the number of period blocks that are not on the ends
            let period_re = Regex::new("^*[?#]+([.]+)[?#]+*").expect("Valid regex required");
            let num_periods: u64 = period_re
                .captures_iter(cleaned_row.as_bytes())
                .fold(0, |acc, _| acc + 1);

            for cap in period_re.captures_iter(cleaned_row.as_bytes()) {
                let (_, x): (&[u8], [&[u8]; 1]) = cap.extract();
            }
            let num_missing_periods: u64 = (groups.len() as u64) - 1 - num_periods;

            let n_items = num_questions - sum_filtered_groups - num_missing_periods;

            // Bug is here - problem occurs when items are split up which constrains how big each bucket is
            let k_buckets = num_filtered_groups + 1;
            let result = binomial(n_items + k_buckets - 1, n_items);
            println!("{cleaned_row:?} {filtered_groups:?} {n_items} {k_buckets} {result}");
        }
    }

    solution
}

fn get_solution_part2(input_data_raw: &str) -> u64 {
    0
}

/// Creates a Regex that can check if a row of springs matches the given group numbers.
///
/// E.g. for the group numbers `[3,2,1]` this will generate the regex
/// `^[.?]*[#?]{3}[.?]+[#?]{2}[.?]+[#?]{1}[.?]*$`, which can check if any row of springs consisting
/// of  `#`, `.` and `?`s can match those group numbers.
fn generate_groups_regex(groups: &[u64]) -> Regex {
    let groups_re = groups.iter().map(|n| format!("[#?]{{{n}}}")).join("[.?]+");
    let full_re = format!("^[.?]*{groups_re}[.?]*$");
    Regex::new(&full_re).expect("generated regex should be valid")
}
