use hashbrown::HashMap;
use itertools::Itertools;
use textwrap::dedent;

use crate::helpers::{print_solutions, Example, Solution};
use crate::utils::get_input_for_day;

pub fn print_solutions_day5() {
    let day: u32 = 5;
    let example = Example {
        input_data: dedent(
            "
            seeds: 79 14 55 13

            seed-to-soil map:
            50 98 2
            52 50 48

            soil-to-fertilizer map:
            0 15 37
            37 52 2
            39 0 15

            fertilizer-to-water map:
            49 53 8
            0 11 42
            42 0 7
            57 7 4

            water-to-light map:
            88 18 7
            18 25 70

            light-to-temperature map:
            45 77 23
            81 45 19
            68 64 13

            temperature-to-humidity map:
            0 69 1
            1 0 69

            humidity-to-location map:
            60 56 37
            56 93 4
                    ",
        ),
        expected_part1: 35,
        expected_part2: 46,
    };
    let solution = Solution {
        input_data: get_input_for_day(day),
        get_solution_part1: get_solution_day5_part1,
        get_solution_part2: get_solution_day5_part2,
    };
    print_solutions(day, example, solution);
}

fn get_solution_day5_part1(input_data_raw: &str) -> u32 {
    let mut inputs: Vec<&str> = input_data_raw.trim().split("\n\n").collect_vec();
    let seeds_input = inputs.remove(0).trim();
    let seed_values: Vec<(i64, i64)> = seeds_input
        .trim()
        .strip_prefix("seeds: ")
        .unwrap()
        .split_whitespace()
        .map(|x| (x.parse().unwrap(), 1)) //_or(0) as i64)
        .collect_vec();

    let map_of_maps = build_map_of_maps(&inputs);

    // Solution
    get_min_location(&seed_values, &map_of_maps)
        .try_into()
        .unwrap()
}

fn get_solution_day5_part2(input_data_raw: &str) -> u32 {
    let mut inputs: Vec<&str> = input_data_raw.trim().split("\n\n").collect_vec();
    let seeds_input = inputs.remove(0).trim();

    // Extract the values. Rollup pairs to create tuples (range_start, length).
    let seed_values: Vec<(i64, i64)> = seeds_input
        .trim()
        .strip_prefix("seeds: ")
        .unwrap()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .tuples()
        .collect_vec();

    let map_of_maps = build_map_of_maps(&inputs);

    // Solution
    get_min_location(&seed_values, &map_of_maps)
        .try_into()
        .unwrap()
}

fn build_map_of_maps(inputs: &Vec<&str>) -> HashMap<String, Vec<Map>> {
    let mut map_of_maps: HashMap<String, Vec<Map>> = HashMap::new();

    for map_input in inputs.iter() {
        // Extract the values from the map.
        // Each map consists of a tuple (dest_start, src_start, length).
        let mut map_parts = map_input.split("\n").collect_vec();
        let map_name = map_parts.remove(0).strip_suffix(" map:").unwrap();

        // Extract the values in the map
        // Each row of input comprises (dest_start, src_start, length)
        // Need to split the inputs, parse as ints and then put into a Map object.
        let map_values: Vec<Map> = map_parts
            .iter()
            .map(|x| {
                x.split_whitespace()
                    .map(|y| y.parse().unwrap())
                    .collect_tuple()
                    .unwrap()
            })
            .map(|(dest_start, src_start, length)| Map {
                src_start,
                src_end: src_start + length,
                dest_start,
                dest_end: dest_start + length,
            })
            .sorted_by(|x, y| x.src_start.cmp(&y.src_start))
            .collect_vec();

        map_of_maps.insert(map_name.to_owned(), map_values);
    }
    return map_of_maps;
}

fn get_min_location(seed_values: &Vec<(i64, i64)>, map_of_maps: &HashMap<String, Vec<Map>>) -> i64 {
    let mut min_location: i64 = 1000000000;
    let map_names = [
        "seed-to-soil",
        "soil-to-fertilizer",
        "fertilizer-to-water",
        "water-to-light",
        "light-to-temperature",
        "temperature-to-humidity",
        "humidity-to-location",
    ];

    /*
    Rather than mapping individual values within a range, which is computationally intensive
    we can just maintain the start and end of ranges. But we need to be careful to split up the
    ranges according to how they intersect with the boundaries defining the maps.

    We maintain 2 vectors: one for ranges to be mapped and another for ranges that
    have been mapped. These are mutable so we can append/pop items according to
    what has been mapped during each iteration. This allows us to split up
    ranges that they overlap with the boundaries of a map, storing the leftover
    segments to the right and left of the boundaries for mapping later.
    */
    let mut ranges_to_map: Vec<(i64, i64)> = seed_values
        .iter()
        .map(|(x, len)| (*x, x + len))
        .collect_vec();
    let mut mapped_ranges: Vec<(i64, i64)> = vec![];

    for map_name in map_names.iter() {
        let map_values = map_of_maps.get(*map_name).unwrap();
        mapped_ranges.clear();

        while 0 < ranges_to_map.len() {
            let (mut range_start, mut range_end) = ranges_to_map.pop().unwrap();
            let mut mapped = false;

            for map in map_values
                .iter()
                .sorted_by(|x, y| x.src_start.cmp(&y.src_end))
            {
                // Remove left segment, store for processing later
                if range_start <= map.src_start && map.src_start < range_end {
                    ranges_to_map.insert(0, (range_start.clone(), map.src_start.clone()));
                    range_start = map.src_start;
                }
                // Remove right segment, store for processing later
                if range_start < map.src_end && map.src_end <= range_end {
                    ranges_to_map.insert(0, (map.src_end.clone(), range_end.clone()));
                    range_end = map.src_end;
                }
                // Map middle segment within the boundaries
                if map.src_start <= range_start && range_end <= map.src_end {
                    let offset = map.dest_end - map.src_end;
                    mapped_ranges.insert(0, (range_start + offset, range_end + offset));
                    mapped = true;
                    break;
                }
            }

            // Any ranges that are unmapped are left unchanged, but added to the mapped ranges.
            if !mapped {
                mapped_ranges.insert(0, (range_start, range_end));
            }
        }
        ranges_to_map = mapped_ranges.clone();
    }

    for (location, _) in mapped_ranges.iter() {
        if *location < min_location {
            min_location = *location;
        }
    }

    return min_location.try_into().unwrap();
}

#[derive(Clone, Debug)]
struct Map {
    src_start: i64,
    src_end: i64,
    dest_start: i64,
    dest_end: i64,
}
