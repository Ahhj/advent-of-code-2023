use itertools::Itertools;
use textwrap::dedent;

use crate::utils::get_input_for_day;

pub fn print_solutions_day3() {
    let example_data_raw: &str = &dedent(
        "
            467..114..
            ...*......
            ..35..633.
            ......#...
            617*......
            .....+.58.
            ..592.....
            ......755.
            ...$.*....
            .664.598..
        ",
    );

    // Example part 1
    let example_solution_part1 = get_solution_day3_part1(&example_data_raw);
    let example_expected_part1 = 4361;
    assert_eq!(example_expected_part1, example_solution_part1);

    // Example part 2
    let example_solution_part2 = get_solution_day3_part2(example_data_raw);
    let example_expected_part2 = 467835;
    assert_eq!(example_expected_part2, example_solution_part2);

    // Part 1
    let input_data_raw = get_input_for_day(3);
    let solution_part1 = get_solution_day3_part1(&input_data_raw);
    println!("Day 3, part 1: {solution_part1}");

    // Part 2
    let solution_part2 = get_solution_day3_part2(&input_data_raw);
    println!("Day 3, part 2: {solution_part2}")
}

fn get_solution_day3_part1(input_data_raw: &str) -> u32 {
    let symbols: Vec<Symbol> = get_symbols(&input_data_raw);

    // Extract the part numbers
    let mut part_numbers: Vec<Number> = symbols.iter().map(|x| x.part_numbers.to_owned()).concat();

    // Need to dedupe incase numbers are registered from multiple symbols.
    dedupe_part_numbers(&mut part_numbers);

    let values: Vec<u32> = part_numbers.iter().map(|number| number.value).collect();
    let solution: u32 = values.iter().sum();

    return solution;
}

fn get_solution_day3_part2(input_data_raw: &str) -> u32 {
    let symbols: Vec<Symbol> = get_symbols(&input_data_raw);

    let mut solution: u32 = 0;

    for symbol in symbols.iter() {
        if symbol.value == '*' && symbol.part_numbers.len() == 2 {
            let (g1, g2) = symbol
                .part_numbers
                .iter()
                .map(|x: &Number| x.value)
                .collect_tuple()
                .unwrap();
            let gear_ratio = g1 * g2;
            solution += gear_ratio;
        }
    }

    return solution;
}

fn get_numbers(input_data_raw: &str) -> Vec<Number> {
    let mut numbers: Vec<Number> = vec![];

    for (row_idx, line) in input_data_raw.lines().enumerate() {
        let mut col_start_idx: usize = 0;
        let mut col_end_idx: usize = 0;
        let mut scanning_number = false;

        for (col_idx, char) in line.char_indices() {
            if char.is_numeric() {
                col_end_idx = col_idx;
                if !scanning_number {
                    col_start_idx = col_idx;
                    scanning_number = true;
                }
            }

            if (!char.is_numeric() || col_idx == line.len() - 1) && scanning_number {
                // Extract and parse value
                let value: u32 = line
                    .get(col_start_idx..col_end_idx + 1)
                    .unwrap()
                    .parse()
                    .unwrap();

                let number = Number {
                    row_idx,
                    col_start_idx,
                    col_end_idx,
                    value,
                };

                numbers.push(number);
                scanning_number = false;
            }
        }
    }
    return numbers;
}

fn get_symbols(input_data_raw: &str) -> Vec<Symbol> {
    let numbers: Vec<Number> = get_numbers(&input_data_raw);
    let mut symbols: Vec<Symbol> = vec![];
    // TODO shorthand
    let offsets: [(i32, i32); 8] = [
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ];

    for (row_idx, line) in input_data_raw.lines().enumerate() {
        for (col_idx, char) in line.char_indices() {
            if !char.is_numeric() && char != '.' {
                let mut part_numbers: Vec<Number> = vec![];

                for (row_offset, col_offset) in offsets.iter() {
                    let search_row_idx = ((row_idx as i32) + row_offset) as usize;
                    let search_col_idx = ((col_idx as i32) + col_offset) as usize;

                    for number in numbers.iter() {
                        if number.col_start_idx <= search_col_idx
                            && search_col_idx <= number.col_end_idx
                            && search_row_idx == number.row_idx
                        {
                            part_numbers.push(number.clone());
                        }
                    }
                }

                // Need to dedupe incase numbers are registered from multiple overlapping positions
                dedupe_part_numbers(&mut part_numbers);

                symbols.push(Symbol {
                    value: char,
                    part_numbers,
                })
            }
        }
    }
    return symbols;
}

fn dedupe_part_numbers(part_numbers: &mut Vec<Number>) {
    part_numbers.sort_by(|a, b| a.row_idx.cmp(&b.row_idx));
    part_numbers.sort_by(|a, b| a.col_start_idx.cmp(&b.col_start_idx));
    part_numbers.dedup_by(|a, b| {
        a.col_start_idx == b.col_start_idx
            && a.col_end_idx == b.col_end_idx
            && a.row_idx == b.row_idx
    });
}

#[derive(Debug)]
struct Symbol {
    value: char,
    part_numbers: Vec<Number>,
}

#[derive(Debug, Clone)]
struct Number {
    row_idx: usize,
    col_start_idx: usize,
    col_end_idx: usize,
    value: u32,
}
