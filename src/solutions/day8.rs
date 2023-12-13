use hashbrown::HashMap;
use itertools::Itertools;
use num::integer::lcm;
use regex::Regex;
use textwrap::dedent;

use crate::helpers::{Example, Solution};
use crate::utils::get_input_for_day;

pub fn print_solutions_day8() {
    let day: u32 = 8;
    let example = Example {
        input_data: dedent(
            "
            LR

            AAA = (11B, XXX)
            11B = (XXX, ZZZ)
            ZZZ = (11B, XXX)
            22A = (22B, XXX)
            22B = (22C, 22C)
            22C = (22Z, 22Z)
            22Z = (22B, 22B)
            XXX = (XXX, XXX)
        ",
        ),
        expected_part1: 2,
        expected_part2: 6,
    };
    let solution = Solution {
        input_data: get_input_for_day(day),
        get_solution_part1,
        get_solution_part2,
    };
    solution.print_solutions(day, example);
}

fn get_solution_part1(input_data_raw: &str) -> u64 {
    get_nsteps_to_finish(input_data_raw, "AAA", "ZZZ")
}

fn get_solution_part2(input_data_raw: &str) -> u64 {
    get_nsteps_to_finish(input_data_raw, "A", "Z")
}

fn get_nsteps_to_finish(
    input_data_raw: &str,
    start_node_endswith: &str,
    target_node_endswith: &str,
) -> u64 {
    // Parse input
    let mut input_data = input_data_raw.trim().lines();
    let instructions = input_data.next().unwrap();

    let all_nodes = input_data
        .filter(|x| !x.is_empty())
        .map(|node_lr_map| Node::from_input(node_lr_map))
        .collect_vec();

    let all_nodes_map: HashMap<String, &Node> =
        all_nodes.iter().map(|x| (x.node.to_owned(), x)).collect();

    // Ghosts start at the following nodes
    let ghost_nodes = all_nodes
        .iter()
        .filter(|node| node.node.ends_with(start_node_endswith))
        .collect_vec();

    // Track nodes which are currently active having started at a given node.
    let mut start_current_map: HashMap<String, &Node> = ghost_nodes
        .iter()
        .map(|node| (node.node.to_owned(), *node))
        .collect();

    // Find intervals where the nodes finish
    let mut nstep: u32 = 1;
    let max_cycles = 1000;
    let mut cycle_intervals: HashMap<String, Vec<u32>> = ghost_nodes
        .iter()
        .map(|node| (node.node.to_owned(), Vec::new()))
        .collect();
    for _ in 0..max_cycles {
        for direction in instructions.chars() {
            for (ghost, node) in start_current_map.clone().iter() {
                let next_node = one_step(*node, &all_nodes_map, direction);

                if next_node.node.ends_with(&target_node_endswith) {
                    let mut intervals = cycle_intervals.get(ghost).unwrap().to_vec();
                    intervals.insert(0, nstep);
                    cycle_intervals.insert(ghost.to_owned(), intervals);
                }

                start_current_map.insert(ghost.to_owned(), next_node);
            }
            nstep += 1;
        }
    }

    let cycle_periods: HashMap<String, Vec<u32>> = cycle_intervals
        .iter()
        .map(|(ghost, intervals)| (ghost.to_owned(), diff(intervals)))
        .collect();

    let result = cycle_periods
        .iter()
        .map(|(_ghost, periods)| periods.get(0).unwrap_or(&0).to_owned() as u64)
        .reduce(lcm)
        .unwrap_or(0);

    return result;
}

/*
Rolling difference between elements
NOTE: this can result in overflow errors if say you provide a u32 and the
difference becomes negative!
*/
fn diff<'a, T>(x: &'a Vec<T>) -> Vec<T>
where
    &'a T: std::ops::Sub<&'a T, Output = T>,
{
    let pairs = x.chunks(2);
    let collect_vec: Vec<T> = pairs
        .filter(|pair| pair.len() == 2)
        .map(|pair: &[T]| pair.get(0).unwrap() - pair.get(1).unwrap())
        .collect_vec();
    collect_vec
}

fn one_step<'a>(
    start_node: &'a Node,
    all_nodes_map: &'a HashMap<String, &Node>,
    direction: char,
) -> &'a Node {
    match direction {
        'L' => all_nodes_map.get(&start_node.left).unwrap(),
        _ => all_nodes_map.get(&start_node.right).unwrap(),
    }
}

#[derive(Debug)]
struct Node {
    node: String,
    left: String,
    right: String,
}

impl Node {
    fn from_input(node_lr_map: &str) -> Node {
        let re = Regex::new("[A-Z0-9]{3}").unwrap();
        let (node, left, right) = re
            .find_iter(node_lr_map)
            .map(|x| x.as_str().to_string())
            .collect_tuple()
            .unwrap();

        Node { node, left, right }
    }
}
