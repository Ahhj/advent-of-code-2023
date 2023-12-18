use hashbrown::HashMap;
use itertools::Itertools;
use textwrap::dedent;

use crate::helpers::{Example, Solution};
use crate::utils::get_input_for_day;

pub fn print_solutions_day11() {
    let day: u32 = 11;
    let example = Example {
        input_data: dedent(
            "
            ...#......
            .......#..
            #.........
            ..........
            ......#...
            .#........
            .........#
            ..........
            .......#..
            #...#.....
        ",
        ),
        expected_part1: 374,
        expected_part2: 82000210, // Not actual example because different params used.
    };
    let solution = Solution {
        input_data: get_input_for_day(day),
        get_solution_part1,
        get_solution_part2,
    };
    solution.print_solutions(day, example);
}

fn get_solution_part1(input_data_raw: &str) -> u64 {
    let coords = get_expanded_coords(input_data_raw, None);
    get_total_distance(&coords)
}

fn get_solution_part2(input_data_raw: &str) -> u64 {
    let coords = get_expanded_coords(input_data_raw, Some(1000000));
    get_total_distance(&coords)
}

fn get_expanded_coords(
    input_data_raw: &str,
    offset_size: Option<usize>,
) -> HashMap<(usize, usize), char> {
    let (expand_rows, expand_cols) = get_expand_indices(input_data_raw);
    let mut coords: HashMap<(usize, usize), char> = HashMap::new();

    // Get modified coordinates of #'s
    for (i, row) in input_data_raw.trim().lines().enumerate() {
        for (j, col) in row.chars().enumerate() {
            match col {
                '#' => {
                    let i_offset = expand_rows.iter().filter(|&i2| i2 < &i).collect_vec().len();
                    let i_expanded = i + i_offset * (offset_size.unwrap_or(2) - 1);

                    let j_offset = expand_cols.iter().filter(|&j2| j2 < &j).collect_vec().len();
                    let j_expanded = j + j_offset * (offset_size.unwrap_or(2) - 1);

                    coords.insert((i_expanded, j_expanded), col);
                }
                _ => continue,
            }
        }
    }
    return coords;
}

fn get_total_distance(coords: &HashMap<(usize, usize), char>) -> u64 {
    let mut distance = 0;

    for pair in coords.iter().combinations(2) {
        let ((i1, j1), _) = pair.get(0).unwrap();
        let ((i2, j2), _) = pair.get(1).unwrap();

        if i1 > i2 {
            distance += i1 - i2;
        } else {
            distance += i2 - i1;
        }

        if j1 > j2 {
            distance += j1 - j2;
        } else {
            distance += j2 - j1;
        }
    }

    distance as u64
}

fn get_expand_indices(input_data_raw: &str) -> (Vec<usize>, Vec<usize>) {
    // Indexes for rows to be expanded.
    let mut expand_rows: Vec<usize> = Vec::new();
    // Boolean flag for whether column should be expanded.
    let mut col_empty: HashMap<usize, bool> = HashMap::new();

    for (i, row) in input_data_raw.trim().lines().enumerate() {
        for (j, col) in row.chars().enumerate() {
            match col {
                '#' => {
                    col_empty.insert(j, false);
                }
                _ => {
                    // Assume true unless already false
                    let default = *col_empty.get(&j).unwrap_or(&true);
                    col_empty.insert(j, default);
                }
            }
        }

        match row.chars().all_equal_value() {
            Ok('.') => {
                expand_rows.push(i);
            }
            _ => continue,
        }
    }

    // Get indexes for columns to be expanded
    let expand_cols: Vec<usize> = col_empty
        .iter()
        .filter(|(_, &is_empty)| is_empty)
        .map(|(&j, _)| j)
        .collect();

    return (expand_rows, expand_cols);
}
