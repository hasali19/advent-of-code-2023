use std::collections::{HashMap, HashSet};
use std::iter;

use aoc2023::aoc_solution;
use genawaiter::rc::Gen;

fn main() -> eyre::Result<()> {
    aoc_solution(3, |input| {
        part_1(input)?;
        part_2(input)?;
        Ok(())
    })
}

fn part_1(input: &str) -> eyre::Result<()> {
    let (symbols, numbers) = parse_input(input);

    let mut sum = 0;

    for (n, len, (x, y)) in numbers {
        for (x, y) in adjacent(x, y, len) {
            if symbols.contains(&(x, y)) {
                sum += n;
                break;
            }
        }
    }

    println!("{sum}");

    Ok(())
}

fn part_2(input: &str) -> eyre::Result<()> {
    let (symbols, numbers) = parse_input(input);

    let mut gear_ratios = HashMap::new();

    for (n, len, (x, y)) in numbers {
        for (x, y) in adjacent(x, y, len) {
            if symbols.contains(&(x, y)) {
                gear_ratios.entry((x, y)).or_insert_with(Vec::new).push(n);
            }
        }
    }

    let mut sum = 0;

    for parts in gear_ratios.values() {
        if parts.len() == 2 {
            sum += parts[0] * parts[1];
        }
    }

    println!("{sum}");

    Ok(())
}

type SymbolSet = HashSet<(usize, usize)>;
type Numbers = Vec<(u32, usize, (usize, usize))>;

fn parse_input(input: &str) -> (SymbolSet, Numbers) {
    let mut symbols = HashSet::new();
    let mut numbers = vec![];
    for (y, line) in input.lines().enumerate() {
        let mut current_num = None;
        // An extra '.' is appended to the row to handle the edge case of a number
        // that ends on the last column.
        for (x, c) in line.chars().chain(iter::once('.')).enumerate() {
            if let Some(digit) = c.to_digit(10) {
                current_num = Some(match current_num {
                    None => (digit, 1),
                    Some((n, len)) => (n * 10 + digit, len + 1),
                });
            } else {
                if let Some((n, len)) = current_num {
                    numbers.push((n, len, (x - len, y)));
                    current_num = None;
                }
                if c != '.' {
                    symbols.insert((x, y));
                }
            }
        }
    }
    (symbols, numbers)
}

fn adjacent(x: usize, y: usize, len: usize) -> impl Iterator<Item = (usize, usize)> {
    Gen::new(|co| async move {
        for ax in x.saturating_sub(1)..=x + len {
            for ay in y.saturating_sub(1)..=y + 1 {
                co.yield_((ax, ay)).await;
            }
        }
    })
    .into_iter()
}
