use itertools::Itertools;
use std::cmp::Ordering;
use std::collections::HashMap;
use textwrap::dedent;

use crate::helpers::{print_solutions, Example, Solution};
use crate::utils::get_input_for_day;

pub fn print_solutions_day7() {
    let day: u32 = 7;
    let example = Example {
        input_data: dedent(
            "
            32T3K 765
            T55J5 684
            KK677 28
            KTJJT 220
            QQQJA 483
        ",
        ),
        expected_part1: 6440,
        expected_part2: 5905,
    };
    let solution = Solution {
        input_data: get_input_for_day(day),
        get_solution_part1,
        get_solution_part2,
    };
    print_solutions(day, example, solution);
}

fn get_solution_part1(input_data_raw: &str) -> u32 {
    let input_data = input_data_raw.trim().lines();
    let mut hands: Vec<Hand> = input_data
        .map(|line| line.split_whitespace().collect_tuple().unwrap())
        .map(|(cards, bid)| Hand::create(cards, bid))
        .collect_vec();

    hands.sort_by(Hand::compare);
    hands
        .iter()
        .rev()
        .enumerate()
        .map(|(i, hand)| hand.bid * ((i as u32) + 1))
        .sum()
}

fn get_solution_part2(input_data_raw: &str) -> u32 {
    // Substitute Jacks (J) for Jokers (X)
    let input_data_prepped = &input_data_raw.trim().replace("J", "X");
    let mut hands: Vec<Hand> = input_data_prepped
        .lines()
        .map(|line| line.split_whitespace().collect_tuple().unwrap())
        .map(|(cards, bid)| Hand::create(cards, bid))
        .collect_vec();

    hands.sort_by(Hand::compare);
    hands
        .iter()
        .rev()
        .enumerate()
        .map(|(i, hand)| hand.bid * ((i as u32) + 1))
        .sum()
}

#[derive(Debug)]
struct Hand {
    cards: String,
    bid: u32,
    hand_type: HandType,
}

impl Hand {
    fn create(cards: &str, bid: &str) -> Hand {
        let mut card_freq_map: HashMap<char, u32> = cards
            .chars()
            .sorted()
            .into_grouping_map_by(|&x| x)
            .fold(0, |acc, _key, _value| acc + 1);

        let num_jokers = card_freq_map.remove(&'X').unwrap_or(0);

        // Convert to vector and sort by value descending
        let card_freqs: Vec<(char, u32)> = card_freq_map
            .iter()
            .map(|(char, count)| (char.to_owned(), count.to_owned()))
            .collect_vec();

        let hand_type = Self::_get_hand_type(&card_freqs, &num_jokers);

        Hand {
            cards: cards.to_string(),
            bid: bid.parse().unwrap(),
            hand_type,
        }
    }

    fn _get_hand_type(card_freqs: &Vec<(char, u32)>, num_jokers: &u32) -> HandType {
        let freqs: (u32, u32, u32, u32, u32) = card_freqs
            .iter()
            .map(|(_card, freq)| freq.to_owned())
            .sorted()
            .rev()
            .pad_using(5, |_i| 0)
            .collect_tuple()
            .unwrap();

        match (freqs, num_jokers) {
            ((5, 0, 0, 0, 0), 0) => HandType::FiveKind,
            ((4, 0, 0, 0, 0), 1) => HandType::FiveKind,
            ((3, 0, 0, 0, 0), 2) => HandType::FiveKind,
            ((2, 0, 0, 0, 0), 3) => HandType::FiveKind,
            ((1, 0, 0, 0, 0), 4) => HandType::FiveKind,
            ((0, 0, 0, 0, 0), 5) => HandType::FiveKind,
            ((4, 1, 0, 0, 0), 0) => HandType::FourKind,
            ((3, 1, 0, 0, 0), 1) => HandType::FourKind,
            ((2, 1, 0, 0, 0), 2) => HandType::FourKind,
            ((1, 1, 0, 0, 0), 3) => HandType::FourKind,
            ((3, 2, 0, 0, 0), 0) => HandType::FullHouse,
            ((2, 2, 0, 0, 0), 1) => HandType::FullHouse,
            ((3, 1, 1, 0, 0), 0) => HandType::ThreeKind,
            ((2, 1, 1, 0, 0), 1) => HandType::ThreeKind,
            ((1, 1, 1, 0, 0), 2) => HandType::ThreeKind,
            ((2, 2, 1, 0, 0), 0) => HandType::TwoPair,
            ((2, 1, 1, 1, 0), 0) => HandType::OnePair,
            ((1, 1, 1, 1, 0), 1) => HandType::OnePair,
            _ => HandType::HighCard,
        }
    }

    fn compare(x: &Hand, y: &Hand) -> Ordering {
        let by_type = x.hand_type.cmp(&y.hand_type);

        if let Ordering::Equal = by_type {
            x._compare_card_by_card(y)
        } else {
            by_type
        }
    }

    fn _compare_card_by_card(&self, other: &Hand) -> Ordering {
        // Jokers represented by 'X' get the lowest order
        let card_order = [
            'A', 'K', 'Q', 'J', 'T', '9', '8', '7', '6', '5', '4', '3', '2', '1', 'X',
        ];
        let card_order_map: HashMap<char, usize> = card_order
            .iter()
            .enumerate()
            .map(|(i, card)| (card.clone(), i))
            .collect();

        for (card1, card2) in self.cards.chars().zip(other.cards.chars()) {
            let order = card_order_map.get(&card1).cmp(&card_order_map.get(&card2));

            if let Ordering::Equal = order {
                continue;
            } else {
                return order;
            }
        }

        Ordering::Equal
    }
}

#[derive(Debug, PartialOrd, Ord, PartialEq, Eq)]
enum HandType {
    FiveKind,
    FourKind,
    FullHouse,
    ThreeKind,
    TwoPair,
    OnePair,
    HighCard,
}
