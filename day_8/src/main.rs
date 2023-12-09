use std::collections::HashMap;

use aoc2023::aoc_solution;
use itertools::{FoldWhile, Itertools};
use num::integer::lcm;
use winnow::ascii::alphanumeric1;
use winnow::combinator::{delimited, separated_pair};
use winnow::error::{ErrorKind, ParseError};
use winnow::Parser;

fn main() -> eyre::Result<()> {
    aoc_solution(8, |input| {
        part_1(input)?;
        part_2(input)?;
        Ok(())
    })
}

enum Direction {
    Left,
    Right,
}

impl Direction {
    fn apply<T>(&self, (l, r): (T, T)) -> T {
        match self {
            Direction::Left => l,
            Direction::Right => r,
        }
    }
}

fn part_1(input: &str) -> eyre::Result<()> {
    let (directions, nodes) = parse_input(input);

    let steps = steps(
        "AAA",
        &directions,
        |node| *nodes.get(node).unwrap(),
        |node| node == "ZZZ",
    );

    println!("{steps}");

    Ok(())
}

fn part_2(input: &str) -> eyre::Result<()> {
    let (directions, nodes) = parse_input(input);

    let starting_points = nodes.keys().filter(|k| k.ends_with('A')).collect_vec();
    let steps = starting_points
        .iter()
        .map(|node| {
            steps(
                node,
                &directions,
                |node| *nodes.get(node).unwrap(),
                |node| node.ends_with('Z'),
            )
        })
        .reduce(lcm)
        .unwrap();

    println!("{steps}");

    Ok(())
}

fn steps<'a>(
    start: &'a str,
    directions: &'a [Direction],
    next: impl Fn(&'a str) -> (&'a str, &'a str),
    is_end: impl Fn(&'a str) -> bool,
) -> usize {
    use FoldWhile::*;

    directions
        .iter()
        .cycle()
        .fold_while((start, 0usize), |(current, steps), direction| {
            if is_end(current) {
                Done((current, steps))
            } else {
                Continue((direction.apply(next(current)), steps + 1))
            }
        })
        .into_inner()
        .1
}

type Instruction<'a> = (&'a str, (&'a str, &'a str));

fn parse_instruction(instruction: &str) -> Result<Instruction, ParseError<&str, ErrorKind>> {
    let mut parser = separated_pair(
        alphanumeric1,
        " = ",
        delimited('(', separated_pair(alphanumeric1, ", ", alphanumeric1), ')'),
    );

    parser.parse(instruction)
}

fn parse_input(input: &str) -> (Vec<Direction>, HashMap<&str, (&str, &str)>) {
    let mut sections = input.split("\n\n");

    let directions = sections
        .next()
        .unwrap()
        .chars()
        .map(|c| match c {
            'L' => Direction::Left,
            'R' => Direction::Right,
            _ => unreachable!(),
        })
        .collect_vec();

    let nodes = sections
        .next()
        .unwrap()
        .lines()
        .map(|line| parse_instruction(line).unwrap())
        .collect::<HashMap<_, _>>();

    (directions, nodes)
}
