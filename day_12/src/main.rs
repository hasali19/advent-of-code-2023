#![feature(let_chains)]

use std::collections::HashMap;
use std::iter;

use aoc2023::aoc_solution;
use itertools::Itertools;
use tap::Tap;

fn main() -> eyre::Result<()> {
    aoc_solution(12, |input| {
        part_1(input)?;
        part_2(input)?;
        Ok(())
    })
}

fn part_1(input: &str) -> eyre::Result<()> {
    let sum: usize = parse_input(input)
        .iter()
        .map(|(conditions, counts)| Solver::default().solve(conditions, counts, 0))
        .sum();

    println!("{sum}");

    Ok(())
}

fn part_2(input: &str) -> eyre::Result<()> {
    let sum: usize = parse_input(input)
        .into_iter()
        .map(|(conditions, counts)| {
            #[allow(unstable_name_collisions)]
            let conditions = iter::repeat(conditions.as_slice())
                .take(5)
                .intersperse(&[Condition::Unknown])
                .flatten()
                .copied()
                .collect_vec();

            let counts = iter::repeat(counts.iter().copied())
                .take(5)
                .flatten()
                .collect_vec();

            (conditions, counts)
        })
        .map(|(conditions, counts)| Solver::default().solve(&conditions, &counts, 0))
        .sum();

    println!("{sum}");

    Ok(())
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
enum Condition {
    Operational,
    Damaged,
    Unknown,
}

#[derive(Default)]
struct Solver<'a> {
    cache: HashMap<(&'a [Condition], &'a [usize], usize), usize>,
}

impl<'a> Solver<'a> {
    fn solve(&mut self, conditions: &'a [Condition], counts: &'a [usize], n: usize) -> usize {
        if let Some(v) = self.cache.get(&(conditions, counts, n)) {
            return *v;
        }

        self._solve(conditions, counts, n).tap(|v| {
            self.cache.insert((conditions, counts, n), *v);
        })
    }

    fn _solve(&mut self, conditions: &'a [Condition], group_sizes: &'a [usize], n: usize) -> usize {
        use Condition::*;

        let Some(&condition) = conditions.first() else {
            // Base case: `conditions` is empty
            return match group_sizes {
                // no group left to fill -> valid solution
                [] => 1,
                // 1 group left, group size matches current run length -> valid solution
                &[c] if c == n => 1,
                // multiple groups left, but no more conditions -> groups are unsatisfiable
                [..] => 0,
            };
        };

        let mut possibilities = 0;

        if let Operational | Unknown = condition {
            if !group_sizes.is_empty() && group_sizes[0] == n {
                // current run length matches target group size, group has been filled
                possibilities += self.solve(&conditions[1..], &group_sizes[1..], 0);
            } else if n == 0 {
                // current run length is 0 so previous condition must have been '.', consume and recurse
                possibilities += self.solve(&conditions[1..], group_sizes, 0);
            } else {
                // previous condition must have been '#' but group isn't full yet so this can't be a '.'
            }
        }

        if let Damaged | Unknown = condition {
            if !group_sizes.is_empty() && n < group_sizes[0] {
                // current run length < target group size, consume and recurse
                possibilities += self.solve(&conditions[1..], group_sizes, n + 1);
            } else {
                // current run length is >= target group size, so there can't be any more consecutive '#'
            }
        }

        possibilities
    }
}

fn parse_input(input: &str) -> Vec<(Vec<Condition>, Vec<usize>)> {
    input
        .lines()
        .map(|line| {
            let (conditions, groups) = line.split_once(' ').unwrap();

            let conditions = conditions
                .chars()
                .map(|c| match c {
                    '.' => Condition::Operational,
                    '#' => Condition::Damaged,
                    '?' => Condition::Unknown,
                    _ => unreachable!(),
                })
                .collect_vec();

            let groups = groups.split(',').map(|g| g.parse().unwrap()).collect_vec();

            (conditions, groups)
        })
        .collect_vec()
}
