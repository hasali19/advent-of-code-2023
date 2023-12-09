use std::collections::HashMap;

use aoc2023::aoc_solution;
use eyre::eyre;
use winnow::ascii::line_ending;
use winnow::combinator::{alt, preceded, repeat, separated, separated_pair, terminated};
use winnow::error::{ErrorKind, ParseError};
use winnow::token::take_while;
use winnow::Parser;

fn main() -> eyre::Result<()> {
    aoc_solution(2, |input| {
        part_1(input)?;
        part_2(input)?;
        Ok(())
    })
}

fn part_1(input: &str) -> eyre::Result<()> {
    let sum: u32 = parse_input(input)
        .map_err(|e| eyre!("{e}"))?
        .iter()
        .filter_map(|(id, subsets)| {
            let is_possible = subsets.iter().all(|colors| {
                *colors.get("red").unwrap_or(&0) <= 12
                    && *colors.get("green").unwrap_or(&0) <= 13
                    && *colors.get("blue").unwrap_or(&0) <= 14
            });

            if is_possible {
                Some(id)
            } else {
                None
            }
        })
        .sum();

    println!("{sum}");

    Ok(())
}

fn part_2(input: &str) -> eyre::Result<()> {
    let cube_sum: u32 = parse_input(input)
        .map_err(|e| eyre!("{e}"))?
        .iter()
        .map(|(_id, subsets)| {
            let max_for_color = |color| {
                subsets
                    .iter()
                    .map(|colors| colors.get(color).unwrap_or(&0))
                    .copied()
                    .max()
                    .unwrap_or(0)
            };

            max_for_color("red") * max_for_color("green") * max_for_color("blue")
        })
        .sum();

    println!("{cube_sum}");

    Ok(())
}

type Game<'a> = (u32, Vec<HashMap<&'a str, u32>>);

fn parse_input(input: &str) -> Result<Vec<Game>, ParseError<&str, ErrorKind>> {
    let u32 = || take_while(1.., '0'..='9').parse_to::<u32>();
    let id = preceded("Game ", u32());

    let pair = separated_pair(u32(), ' ', alt(("red", "green", "blue"))).map(|(n, c)| (c, n));
    let subset =
        separated(0.., pair, ", ").map(|v: Vec<_>| v.into_iter().collect::<HashMap<_, _>>());
    let subsets = separated(0.., subset, "; ");

    let game = terminated(separated_pair(id, ": ", subsets), line_ending);

    repeat(0.., game).parse(input)
}
