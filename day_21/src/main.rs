use std::collections::HashSet;
use std::mem;

use aoc2023::aoc_solution;

fn main() -> eyre::Result<()> {
    aoc_solution(21, |input| {
        part_1(input)?;
        part_2(input)?;
        Ok(())
    })
}

fn part_1(input: &str) -> eyre::Result<()> {
    let mut garden_plots = HashSet::new();
    let mut start = None;

    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c == 'S' {
                start = Some((x, y));
                garden_plots.insert((x, y));
            } else if c == '.' {
                garden_plots.insert((x, y));
            }
        }
    }

    let start = start.unwrap();

    let mut current = HashSet::from([start]);
    let mut next = HashSet::new();

    for _ in 0..6 {
        for &(x, y) in &current {
            let neighbours = [
                (x.saturating_sub(1), y),
                (x + 1, y),
                (x, y.saturating_sub(1)),
                (x, y + 1),
            ];

            for pos in neighbours {
                if garden_plots.contains(&pos) {
                    next.insert(pos);
                }
            }
        }

        current = mem::take(&mut next);
    }

    println!("{}", current.len());

    Ok(())
}

fn part_2(input: &str) -> eyre::Result<()> {
    Ok(())
}
