use itertools::Itertools;
use regex::Regex;
use textwrap::dedent;

use crate::helpers::{print_solutions, Example, Solution};
use crate::utils::get_input_for_day;

pub fn print_solutions_day6() {
    let day: u32 = 6;
    let example = Example {
        input_data: dedent(
            "
            Time:      7  15   30
            Distance:  9  40  200
        ",
        ),
        expected_part1: 288,
        expected_part2: 71503,
    };
    let solution = Solution {
        input_data: get_input_for_day(day),
        get_solution_part1,
        get_solution_part2,
    };
    print_solutions(day, example, solution);
}

fn get_solution_part1(input_data_raw: &str) -> u32 {
    // Extract time line
    let times_re = Regex::new(r"Time:\s+(.*)").unwrap();
    let times_inputs = times_re.find(&input_data_raw).unwrap().as_str();

    // Extract distance line
    let distances_re = Regex::new(r"Distance:\s+(.*)").unwrap();
    let distances_inputs = distances_re.find(&input_data_raw).unwrap().as_str();

    // Parse times and distance values
    let numbers_re = Regex::new(r"(\d+)").unwrap();
    let times: Vec<f64> = numbers_re
        .find_iter(&times_inputs)
        .map(|x| x.as_str().parse().unwrap())
        .collect_vec();
    let distances: Vec<f64> = numbers_re
        .find_iter(&distances_inputs)
        .map(|x| x.as_str().parse().unwrap())
        .collect_vec();

    get_solution(times, distances)
}

fn get_solution_part2(input_data_raw: &str) -> u32 {
    // Extract time line
    let times_re = Regex::new(r"Time:\s+(.*)").unwrap();
    let times_inputs = times_re.find(&input_data_raw).unwrap().as_str();

    // Extract distance line
    let distances_re = Regex::new(r"Distance:\s+(.*)").unwrap();
    let distances_inputs = distances_re.find(&input_data_raw).unwrap().as_str();

    // Parse times and distance values
    let numbers_re = Regex::new(r"(\d+)").unwrap();
    let time: f64 = numbers_re
        .find_iter(&times_inputs)
        .map(|x| x.as_str())
        .join("")
        .parse()
        .unwrap();
    let distances: f64 = numbers_re
        .find_iter(&distances_inputs)
        .map(|x| x.as_str())
        .join("")
        .parse()
        .unwrap();

    get_solution(vec![time], vec![distances])
}

fn get_solution(times: Vec<f64>, distances: Vec<f64>) -> u32 {
    let accel = 1.;
    let races = times.iter().zip(distances).map(|(time, distance)| Race {
        time: time.to_owned(),
        distance,
        accel,
    });

    races
        .map(|r| r.count_winning_strategies())
        .reduce(|x, y| x * y)
        .unwrap_or(0)
}

#[derive(Debug)]
struct Race {
    time: f64,
    distance: f64,
    accel: f64,
}

trait Strategy {
    fn calc_winning_press_times(&self) -> (u32, u32) {
        (0, 0)
    }

    fn count_winning_strategies(&self) -> u32 {
        let (tmin, tmax) = &self.calc_winning_press_times();
        tmax - tmin + 1
    }
}

impl Strategy for Race {
    fn calc_winning_press_times(&self) -> (u32, u32) {
        let sqrt_component = (self.accel.powf(2.) * self.time.powf(2.)
            - 4. * self.accel * (self.distance + 1.))
            .sqrt();

        // TODO: quadratic formula
        let tmin = (self.accel * self.time - sqrt_component) / (2. * self.accel);
        let tmax = (self.accel * self.time + sqrt_component) / (2. * self.accel);
        (tmin.ceil() as u32, tmax.floor() as u32)
    }
}
