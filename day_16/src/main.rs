use std::collections::HashSet;

use aoc2023::aoc_solution;
use itertools::Itertools;

fn main() -> eyre::Result<()> {
    aoc_solution(16, |input| {
        part_1(input)?;
        part_2(input)?;
        Ok(())
    })
}

fn part_1(input: &str) -> eyre::Result<()> {
    let grid = input
        .lines()
        .map(|line| line.chars().collect_vec())
        .collect_vec();

    println!("{}", count_energized(&grid, ((0, 0), (1, 0))));

    Ok(())
}

fn part_2(input: &str) -> eyre::Result<()> {
    let grid = input
        .lines()
        .map(|line| line.chars().collect_vec())
        .collect_vec();

    let width = grid[0].len() as isize;
    let height = grid.len() as isize;

    let max = (0..width)
        .flat_map(|x| [((x, 0), (0, 1)), ((x, height - 1), (0, -1))])
        .chain((0..height).flat_map(|y| [((0, y), (1, 0)), ((width - 1, y), (-1, 0))]))
        .map(|start| count_energized(&grid, start))
        .max()
        .unwrap();

    println!("{max}");

    Ok(())
}

type Grid = Vec<Vec<char>>;

fn count_energized(grid: &Grid, start: ((isize, isize), (isize, isize))) -> usize {
    let width = grid[0].len();
    let height = grid.len();

    let mut stack = vec![start];
    let mut visited = HashSet::new();
    let mut energized = HashSet::new();

    while let Some(((x, y), (dx, dy))) = stack.pop() {
        if !visited.insert(((x, y), (dx, dy))) {
            continue;
        }

        energized.insert((x, y));

        let mut push = |(x, y), (dx, dy)| {
            if (0..width).contains(&((x + dx) as usize))
                && (0..height).contains(&((y + dy) as usize))
            {
                stack.push(((x + dx, y + dy), (dx, dy)));
            }
        };

        match grid[y as usize][x as usize] {
            '.' => push((x, y), (dx, dy)),
            '|' if dx == 1 || dx == -1 => {
                push((x, y), (0, 1));
                push((x, y), (0, -1));
            }
            '|' if dy == 1 || dy == -1 => push((x, y), (dx, dy)),
            '-' if dx == 1 || dx == -1 => push((x, y), (dx, dy)),
            '-' if dy == 1 || dy == -1 => {
                push((x, y), (1, 0));
                push((x, y), (-1, 0));
            }
            '/' => {
                let (dx, dy) = match (dx, dy) {
                    (1, 0) => (0, -1),
                    (-1, 0) => (0, 1),
                    (0, 1) => (-1, 0),
                    (0, -1) => (1, 0),
                    _ => unreachable!(),
                };
                push((x, y), (dx, dy));
            }
            '\\' => {
                let (dx, dy) = match (dx, dy) {
                    (1, 0) => (0, 1),
                    (-1, 0) => (0, -1),
                    (0, 1) => (1, 0),
                    (0, -1) => (-1, 0),
                    _ => unreachable!(),
                };
                push((x, y), (dx, dy));
            }
            _ => unreachable!(),
        }
    }

    energized.len()
}
