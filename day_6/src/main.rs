use aoc2023::aoc_solution;
use itertools::Itertools;

fn main() -> eyre::Result<()> {
    aoc_solution(6, |input| {
        part_1(input)?;
        part_2(input)?;
        Ok(())
    })
}

fn part_1(input: &str) -> eyre::Result<()> {
    let (times, distances) = parse_input_as_list(input);
    let races = times.into_iter().zip(distances).collect_vec();

    let prod: usize = races
        .iter()
        .map(|&(time, distance)| count_possible_hold_times(time, distance))
        .product();

    println!("{prod}");

    Ok(())
}

fn part_2(input: &str) -> eyre::Result<()> {
    let (time, distance) = parse_input_as_single(input);

    let possibilities = count_possible_hold_times(time, distance);

    println!("{possibilities}");

    Ok(())
}

fn count_possible_hold_times(time: usize, min_distance: usize) -> usize {
    let check_hold_time = |&hold_time: &usize| {
        let race_time = time - hold_time;
        let total_dist = race_time * hold_time;
        total_dist > min_distance
    };

    let min_hold_time = (0..=time).find(check_hold_time);
    let max_hold_time = (0..=time).rev().find(check_hold_time);

    max_hold_time.unwrap() - min_hold_time.unwrap() + 1
}

fn parse_input<T>(input: &str, strategy: impl Fn(&str) -> T) -> (T, T) {
    let mut lines = input
        .lines()
        .map(|line| line.split_once(':').unwrap().1.trim());

    let times = strategy(lines.next().unwrap());
    let distances = strategy(lines.next().unwrap());

    (times, distances)
}

fn parse_input_as_list(input: &str) -> (Vec<usize>, Vec<usize>) {
    parse_input(input, |line| {
        line.split_ascii_whitespace()
            .map(|v| v.parse().unwrap())
            .collect_vec()
    })
}

fn parse_input_as_single(input: &str) -> (usize, usize) {
    parse_input(input, |line| line.replace(' ', "").parse().unwrap())
}
