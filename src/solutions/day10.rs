use hashbrown::HashMap;
use itertools::Itertools;
use num::Integer;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;
use textwrap::dedent;

use crate::helpers::{Example, Solution};
use crate::utils::get_input_for_day;

pub fn print_solutions_day10() {
    let day: u32 = 10;
    let example = Example {
        input_data: dedent(
            "
            .F----7F7F7F7F-7....
            .|F--7||||||||FJ....
            .||.FJ||||||||L7....
            FJL7L7LJLJ||LJ.L-7..
            L--J.L7...LJS7F-7L7.
            ....F-J..F7FJ|L7L7L7
            ....L7.F7||L7|.L7L7|
            .....|FJLJ|FJ|F7|.LJ
            ....FJL-7.||.||||...
            ....L---J.LJ.LJLJ...
        ",
        ),
        expected_part1: 70,
        expected_part2: 8,
    };
    let solution = Solution {
        input_data: get_input_for_day(day),
        get_solution_part1,
        get_solution_part2,
    };
    solution.print_solutions(day, example);
}

fn get_solution_part1(input_data_raw: &str) -> u64 {
    let board = get_board(input_data_raw);
    let forward_path = get_loop_path(&board);
    forward_path.nodes.len().div_floor(&2) as u64
}

fn get_solution_part2(input_data_raw: &str) -> u64 {
    // TODO: each '.' should have an odd number of pipes up, down, left, right to count
    let board: HashMap<(usize, usize), char> = get_board(input_data_raw);
    let forward_path = get_loop_path(&board);

    // https://en.wikipedia.org/wiki/Pick%27s_theorem
    let mut area: f64 = 0.;
    for ((x1, y1), (x2, y2)) in forward_path
        .nodes
        .iter()
        .zip(forward_path.nodes.iter().skip(1))
    {
        area += determinant(*x1, *x2, *y1, *y2);
    }
    area /= 2.;

    // https://en.wikipedia.org/wiki/Shoelace_formula#Example
    // Minus 1 to boundary because start appears twice.
    let num_boundary = forward_path.nodes.len() as f64 - 1.;
    let num_enclosed = area - num_boundary / 2. + 1.;

    num_enclosed as u64
}

fn determinant(a: usize, b: usize, c: usize, d: usize) -> f64 {
    ((a * d) as f64) - ((b * c) as f64)
}

fn get_board(input_data_raw: &str) -> HashMap<(usize, usize), char> {
    // Coordinates mapped to characters
    input_data_raw
        .trim()
        .lines()
        .enumerate()
        .map(|(i, line)| {
            line.chars()
                .enumerate()
                .map(|(j, c)| ((i, j), c))
                .collect_vec()
        })
        .flatten()
        .collect()
}

fn get_loop_path(board: &HashMap<(usize, usize), char>) -> Path {
    let ((i0, j0), _): (&(usize, usize), &char) =
        board.iter().filter(|(_, &c)| c == 'S').next().unwrap();

    // Catalogue of possible transitions from one node to others.
    let transitions: HashMap<(&usize, &usize), Vec<(usize, usize)>> = board
        .iter()
        .map(|((i, j), c)| ((i, j), get_transitions(i, j, c, &board)))
        .collect();

    let initial_path = Path {
        nodes: vec![(*i0, *j0)],
    };
    let mut paths: Vec<Path> = vec![initial_path];
    let mut completed_paths: Vec<Path> = Vec::new();
    let max_steps = 100000;

    for _ in 0..max_steps {
        let mut active_paths: Vec<Path> = Vec::new();
        for path in paths.iter() {
            // New paths after next step
            let new_paths = path.one_step(&transitions);

            // Check if paths are completed or still going
            for new_path in new_paths.iter().clone() {
                if new_path.nodes.last().unwrap() == &(*i0, *j0) {
                    // Completed if final node is the start node
                    completed_paths.push(new_path.to_owned());
                } else {
                    // Else path is active
                    active_paths.push(new_path.to_owned());
                }
            }
        }

        if active_paths.len() == 0 {
            break;
        } else {
            // Continue with the remaining active paths
            paths = active_paths;
        }
    }

    // Additional length filter to remove branched paths
    let min_length = completed_paths
        .iter()
        .map(|path| path.nodes.len())
        .min()
        .unwrap_or(0);
    completed_paths = completed_paths
        .into_iter()
        .filter(|path| path.nodes.len() == min_length)
        .collect();

    let forward_path = completed_paths.get(0).unwrap();
    // let backward_path = *completed_paths.get(1).unwrap();
    // assert_eq!(forward_path.nodes.len(), backward_path.nodes.len());
    return forward_path.to_owned();
}

fn get_transitions(
    i: &usize,
    j: &usize,
    c: &char,
    board: &HashMap<(usize, usize), char>,
) -> Vec<(usize, usize)> {
    let mut transitions: Vec<(usize, usize)> = Vec::new();

    for t in Transition::iter() {
        // Target coords
        let (i2, j2) = get_transition_coords(i, j, &t);
        if (&i2, &j2) == (i, j) {
            continue;
        }

        // Check character
        let c2 = board.get(&(i2, j2)).unwrap_or(&'X');
        if validate_transition(c, c2, &t) {
            transitions.insert(0, (i2, j2))
        }
    }

    return transitions;
}

fn validate_transition(c1: &char, c2: &char, t: &Transition) -> bool {
    // From -> to
    let downs = ['|', '7', 'F'];
    let ups = ['|', 'L', 'J'];
    let lefts = ['-', 'J', '7'];
    let rights = ['-', 'L', 'F'];

    match t {
        Transition::Up => {
            ((ups.contains(c1) || c1 == &'S') && downs.contains(c2))
                || ((ups.contains(c1)) && (downs.contains(c2) || c2 == &'S'))
        }
        Transition::Down => {
            ((downs.contains(c1) || c1 == &'S') && ups.contains(c2))
                || (downs.contains(c1) && (ups.contains(c2) || c1 == &'S'))
        }
        Transition::Left => {
            ((lefts.contains(c1) || c1 == &'S') && rights.contains(c2))
                || (lefts.contains(c1) && (rights.contains(c2) || c1 == &'S'))
        }
        _ => {
            ((rights.contains(c1) || c1 == &'S') && lefts.contains(c2))
                || (rights.contains(c1) && (lefts.contains(c2) || c1 == &'S'))
        }
    }
}

fn get_transition_coords(i: &usize, j: &usize, t: &Transition) -> (usize, usize) {
    match t {
        Transition::Up => (i - ((i > &0) as usize), *j),
        Transition::Down => (i + 1, *j),
        Transition::Left => (*i, j - ((j > &0) as usize)),
        _ => (*i, j + 1),
    }
}

#[derive(Debug, EnumIter)]
enum Transition {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, Clone)]
struct Path {
    nodes: Vec<(usize, usize)>,
}

impl Path {
    fn one_step(&self, transitions: &HashMap<(&usize, &usize), Vec<(usize, usize)>>) -> Vec<Path> {
        let (icurr, jcurr) = self.nodes.last().unwrap().to_owned();
        let current_coords = (icurr, jcurr);

        // Last position excluded from next
        let prev_coords = self
            .nodes
            .iter()
            .rev()
            .skip(1)
            .next()
            .unwrap_or(&current_coords);

        let next_coords = transitions
            .get(&(&icurr, &jcurr))
            .unwrap()
            .iter()
            .filter(|&coords| coords != prev_coords);

        let mut new_paths: Vec<Path> = Vec::new();

        for coords in next_coords {
            let next_coord = coords.to_owned();
            let mut new_nodes = self.nodes.to_owned();
            new_nodes.push(next_coord);
            new_paths.push(Path { nodes: new_nodes });
        }

        return new_paths;
    }
}
