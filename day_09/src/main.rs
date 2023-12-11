use aoc2023::aoc_solution;
use itertools::Itertools;

fn main() -> eyre::Result<()> {
    aoc_solution(9, |input| {
        part_1(input)?;
        part_2(input)?;
        Ok(())
    })
}

fn part_1(input: &str) -> eyre::Result<()> {
    solve(input, |seq, d| seq.last().unwrap() + d)
}

fn part_2(input: &str) -> eyre::Result<()> {
    solve(input, |seq, d| seq.first().unwrap() - d)
}

fn solve(input: &str, strategy: fn(&[i64], i64) -> i64) -> eyre::Result<()> {
    let sum: i64 = input
        .lines()
        .map(|line| {
            line.split_ascii_whitespace()
                .map(|v| v.parse::<i64>().unwrap())
                .collect_vec()
        })
        .map(|sequence| extrapolate(&sequence, strategy))
        .sum();

    println!("{sum}");

    Ok(())
}

fn extrapolate(sequence: &[i64], strategy: fn(&[i64], i64) -> i64) -> i64 {
    if sequence.iter().all(|&it| it == 0) {
        return 0;
    }

    let diffs = sequence
        .iter()
        .tuple_windows()
        .map(|(a, b)| b - a)
        .collect_vec();

    strategy(sequence, extrapolate(&diffs, strategy))
}
