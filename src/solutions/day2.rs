use std::collections::HashMap;

use crate::utils::get_input_for_day;
use fancy_regex::Regex;
use itertools::Itertools;
use textwrap::dedent;

pub fn print_solutions_day2() {
    let example_data_raw_part1 = dedent(
        "
        Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
        Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
        Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
        Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
        Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
    ",
    );
    let example_games = parse_games(&example_data_raw_part1);

    // Example part 1
    let example_solution_part1: u32 = get_solution_day2_part1(&example_games);
    let example_expected_part1: u32 = 8;
    assert_eq!(example_solution_part1, example_expected_part1);

    // Example part 2
    let example_solution_part2: u32 = get_solution_day2_part2(&example_games);
    let example_expected_part2: u32 = 2286;
    assert_eq!(example_solution_part2, example_expected_part2);

    // Load and process input data
    let input_data_raw: String = get_input_for_day(2);
    let games: Vec<Game> = parse_games(&input_data_raw);

    // Part 1
    let solution_part1: u32 = get_solution_day2_part1(&games);
    println!("Day 2, part 1: {solution_part1}");

    // Part 2
    let solution_part2: u32 = get_solution_day2_part2(&games);
    println!("Day 2, part 2: {solution_part2}");
}

fn get_solution_day2_part1(games: &Vec<Game>) -> u32 {
    let valid_games: Vec<&Game> = games.iter().filter(|&x| is_valid_game(x)).collect_vec();
    let valid_game_ids: Vec<u32> = valid_games.iter().map(|&x| x.game_id).collect_vec();
    let solution: u32 = valid_game_ids.iter().sum();
    return solution;
}

fn get_solution_day2_part2(games: &Vec<Game>) -> u32 {
    let powers: Vec<u32> = games.iter().map(get_power).collect();
    let solution: u32 = powers.iter().sum();
    return solution;
}

/*
Extracts the 'power' from a game, defined as the prduct of the minimum number of each
ball required to play it.
*/
fn get_power(game: &Game) -> u32 {
    let mut max_red: u32 = 0;
    let mut max_green: u32 = 0;
    let mut max_blue: u32 = 0;

    for set in game.game_sets.iter() {
        for count in set.ball_counts.iter() {
            match count.colour.as_str() {
                "red" => {
                    if count.count > max_red {
                        max_red = count.count;
                    }
                }
                "green" => {
                    if count.count > max_green {
                        max_green = count.count;
                    }
                }
                "blue" => {
                    if count.count > max_blue {
                        max_blue = count.count;
                    }
                }
                _ => continue,
            }
        }
    }
    let power: u32 = max_red * max_green * max_blue;
    return power;
}

/*
Parses string input contain the game data to generate a vector of 'Game' structs.
*/
fn parse_games(input_data_raw: &str) -> Vec<Game> {
    let game_re = Regex::new(r"(?<=Game\s)(\d+)").unwrap();
    let blue_re: Regex = Regex::new(r"(\d+)(?=\sblue)").unwrap();
    let red_re: Regex = Regex::new(r"(\d+) red").unwrap();
    let green_re: Regex = Regex::new(r"(\d+) green").unwrap();

    let mut games: Vec<Game> = vec![];

    for line in input_data_raw.lines() {
        if line.is_empty() {
            continue;
        }

        let (game, sets) = line.split(":").collect_tuple().unwrap();

        let game_id: u32 = game
            .split_whitespace()
            .nth_back(0)
            .unwrap_or("0")
            .parse()
            .unwrap();

        let mut game_sets: Vec<GameSet> = vec![];

        for (set_idx, set) in sets.split(";").enumerate() {
            let ball_counts: Vec<BallCount> = set
                .split(",")
                .map(|x| x.split_whitespace().collect_tuple().unwrap())
                .map(|(count, colour)| BallCount {
                    colour: colour.to_string(),
                    count: count.parse().unwrap_or(0),
                })
                .collect();

            game_sets.push(GameSet {
                set_idx,
                ball_counts,
            });
        }

        games.push(Game { game_id, game_sets })
    }

    return games;
}

fn is_valid_game(game: &Game) -> bool {
    for set in game.game_sets.iter() {
        if !is_valid_set(set) {
            return false;
        }
    }
    return true;
}

fn is_valid_set(set: &GameSet) -> bool {
    let (max_red, max_green, max_blue) = (12, 13, 14);

    for count in set.ball_counts.iter() {
        match count.colour.as_str() {
            "red" => {
                if count.count > max_red {
                    return false;
                }
            }
            "green" => {
                if count.count > max_green {
                    return false;
                }
            }
            "blue" => {
                if count.count > max_blue {
                    return false;
                }
            }
            _ => continue,
        }
    }

    return true;
}

#[derive(Debug)]
struct BallCount {
    colour: String,
    count: u32,
}

#[derive(Debug)]
struct GameSet {
    set_idx: usize,
    ball_counts: Vec<BallCount>,
}

struct Game {
    game_id: u32,
    game_sets: Vec<GameSet>,
}
