use std::collections::{HashMap, HashSet};

use aoc2023::aoc_solution;
use eyre::eyre;
use winnow::ascii::{line_ending, space1};
use winnow::combinator::{preceded, repeat, separated, separated_pair, terminated};
use winnow::error::{ErrorKind, ParseError};
use winnow::token::take_while;
use winnow::Parser;

fn main() -> eyre::Result<()> {
    aoc_solution(4, |input| {
        part_1(input)?;
        part_2(input)?;
        Ok(())
    })
}

fn part_1(input: &str) -> eyre::Result<()> {
    let cards = parse_input(input).map_err(|e| eyre!("{e}"))?;

    let sum: usize = cards
        .values()
        .map(|(winning, owned)| card_score(winning, owned))
        .sum();

    println!("{sum}");

    Ok(())
}

fn card_score(winning: &HashSet<usize>, owned: &HashSet<usize>) -> usize {
    owned.iter().fold(0, |acc, n| {
        if winning.contains(n) {
            usize::max(1, acc * 2)
        } else {
            acc
        }
    })
}

fn part_2(input: &str) -> eyre::Result<()> {
    let cards = parse_input(input).map_err(|e| eyre!("{e}"))?;

    let mut total = 0;
    let mut queue = Vec::from_iter(1..=cards.len());

    while let Some(id) = queue.pop() {
        let (winning, have) = cards.get(&id).unwrap();
        let matching = have.iter().filter(|n| winning.contains(*n)).count();
        for i in 1..=matching {
            queue.push(id + i);
        }
        total += 1;
    }

    println!("{total}");

    Ok(())
}

type CardMap = HashMap<usize, (HashSet<usize>, HashSet<usize>)>;

fn parse_input(input: &str) -> Result<CardMap, ParseError<&str, ErrorKind>> {
    let int = || take_while(1.., '0'..='9').parse_to();
    let id = preceded(("Card", space1), int());

    let numbers = || separated(0.., int(), space1).map(|v: Vec<_>| HashSet::from_iter(v));
    let card_numbers = separated_pair(numbers(), (space1, '|', space1), numbers());

    let card = terminated(separated_pair(id, (":", space1), card_numbers), line_ending);

    repeat(0.., card).parse(input)
}
