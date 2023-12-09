use std::cmp::{Ordering, Reverse};
use std::collections::HashMap;

use aoc2023::aoc_solution;
use itertools::Itertools;

fn main() -> eyre::Result<()> {
    aoc_solution(7, |input| {
        part_1(input)?;
        part_2(input)?;
        Ok(())
    })
}

struct Hand<'a> {
    hand: &'a str,
    bid: u32,
}

fn part_1(input: &str) -> eyre::Result<()> {
    let mut hands = parse_input(input);

    let strength_map = "AKQJT98765432"
        .char_indices()
        .map(|(i, c)| (c, i as u8))
        .collect::<HashMap<_, _>>();

    let get_strength = |c| *strength_map.get(&c).unwrap();

    hands.sort_by(|a, b| {
        hand_type(a.hand)
            .cmp(&hand_type(b.hand))
            .then_with(|| compare_hand_strengths(a.hand, b.hand, get_strength))
    });

    let winnings: u32 = hands
        .into_iter()
        .enumerate()
        .map(|(i, hand)| (i as u32 + 1) * hand.bid)
        .sum();

    println!("{winnings}");

    Ok(())
}

fn part_2(input: &str) -> eyre::Result<()> {
    let mut hands = parse_input(input);

    let strength_map = "AKQT98765432J"
        .char_indices()
        .map(|(i, c)| (c, i as u8))
        .collect::<HashMap<_, _>>();

    let get_strength = |c| *strength_map.get(&c).unwrap();

    hands.sort_by(|a, b| {
        hand_type_with_wildcards(a.hand)
            .cmp(&hand_type_with_wildcards(b.hand))
            .then_with(|| compare_hand_strengths(a.hand, b.hand, get_strength))
    });

    let winnings: u32 = hands
        .into_iter()
        .enumerate()
        .map(|(i, hand)| (i as u32 + 1) * hand.bid)
        .sum();

    println!("{winnings}");

    Ok(())
}

fn parse_input(input: &str) -> Vec<Hand> {
    input
        .lines()
        .map(|line| {
            let (hand, bid) = line.split_once(' ').unwrap();
            let bid = bid.parse::<u32>().unwrap();
            Hand { hand, bid }
        })
        .collect_vec()
}

fn hand_type(hand: &str) -> Vec<u8> {
    hand.chars()
        .into_grouping_map_by(|v| *v)
        .fold(0, |acc, _k, _v| acc + 1)
        .into_values()
        .sorted_by_key(|n| Reverse(*n))
        .collect_vec()
}

fn hand_type_with_wildcards(hand: &str) -> Vec<u8> {
    let mut char_counts = hand
        .chars()
        .into_grouping_map_by(|v| *v)
        .fold(0, |acc, _k, _v| acc + 1);

    let jokers = char_counts.remove(&'J').unwrap_or(0);

    let mut char_counts = char_counts
        .into_values()
        .sorted_by_key(|n| Reverse(*n))
        .collect_vec();

    if char_counts.is_empty() {
        char_counts.push(jokers);
    } else {
        char_counts[0] += jokers;
    }

    char_counts
}

fn compare_hand_strengths(a: &str, b: &str, strength: impl Fn(char) -> u8) -> Ordering {
    a.chars()
        .zip(b.chars())
        .map(|(a, b)| strength(b).cmp(&strength(a)))
        .find(|o| o.is_ne())
        .unwrap_or(Ordering::Equal)
}
