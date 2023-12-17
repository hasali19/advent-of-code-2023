#![feature(let_chains)]

use std::collections::HashMap;

use aoc2023::aoc_solution;
use itertools::Itertools;
use pathfinding::directed::dijkstra::dijkstra;

fn main() -> eyre::Result<()> {
    aoc_solution(17, |input| {
        part_1(input)?;
        part_2(input)?;
        Ok(())
    })
}

fn part_1(input: &str) -> eyre::Result<()> {
    solve(input, 1, 3);
    Ok(())
}

fn part_2(input: &str) -> eyre::Result<()> {
    solve(input, 4, 10);
    Ok(())
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
enum Direction {
    North,
    South,
    East,
    West,
}

impl Direction {
    fn turns(&self) -> [Direction; 2] {
        use Direction::*;
        match self {
            North | South => [East, West],
            East | West => [North, South],
        }
    }

    fn delta(&self) -> (isize, isize) {
        use Direction::*;
        match self {
            North => (0, -1),
            South => (0, 1),
            East => (1, 0),
            West => (-1, 0),
        }
    }
}

fn solve(input: &str, forward_min: i32, forward_max: i32) {
    let grid = input
        .lines()
        .map(|line| line.chars().map(|c| c.to_digit(10).unwrap()).collect_vec())
        .collect_vec();

    let width = grid[0].len() as isize;
    let height = grid.len() as isize;

    let map = (0..width)
        .cartesian_product(0..height)
        .map(|(x, y)| ((x, y), grid[y as usize][x as usize]))
        .collect::<HashMap<_, _>>();

    let (_, cost) = dijkstra(
        &((0, 0), Direction::East, 0),
        |&((x, y), d, v)| {
            let mut succ = vec![];

            if v < forward_max {
                let (dx, dy) = d.delta();
                if let Some(dh) = map.get(&(x + dx, y + dy)) {
                    succ.push((((x + dx, y + dy), d, v + 1), *dh));
                }
            }

            if v >= forward_min {
                for d in d.turns() {
                    let (dx, dy) = d.delta();
                    if let Some(dh) = map.get(&(x + dx, y + dy)) {
                        succ.push((((x + dx, y + dy), d, 1), *dh));
                    }
                }
            }

            succ
        },
        |&((x, y), _, v)| x == width - 1 && y == height - 1 && v >= forward_min,
    )
    .unwrap();

    println!("{cost}");
}
