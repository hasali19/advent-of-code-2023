use std::collections::HashMap;

use aoc2023::aoc_solution;
use itertools::Itertools;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

fn main() -> eyre::Result<()> {
    aoc_solution(5, |input| {
        part_1(input)?;
        part_2(input)?;
        Ok(())
    })
}

type Seeds = Vec<usize>;
type Ranges = Vec<(usize, usize, usize)>;

fn part_1(input: &str) -> eyre::Result<()> {
    let (seeds, maps) = parse_input(input);

    let min_location: usize = seeds
        .iter()
        .map(|seed| seed_location(*seed, &maps))
        .min()
        .unwrap();

    println!("{min_location}");

    Ok(())
}

fn part_2(input: &str) -> eyre::Result<()> {
    let (seeds, maps) = parse_input(input);
    let seeds = seeds.into_iter().tuples().collect_vec();

    let min_location: usize = seeds
        .par_iter()
        .flat_map(|(seed_start, seed_len)| *seed_start..*seed_start + *seed_len)
        .map(|seed| seed_location(seed, &maps))
        .min()
        .unwrap();

    println!("{min_location}");

    Ok(())
}

fn seed_location(seed: usize, maps: &HashMap<&str, (&str, Ranges)>) -> usize {
    let mut current_type = "seed";
    let mut current_val = seed;

    while let Some((next_type, ranges)) = maps.get(current_type) {
        current_type = next_type;
        current_val = ranges
            .iter()
            .find_map(|&(dst_start, src_start, len)| {
                if (src_start..src_start + len).contains(&current_val) {
                    Some(dst_start + (current_val - src_start))
                } else {
                    None
                }
            })
            .unwrap_or(current_val);
    }

    current_val
}

fn parse_input(input: &str) -> (Seeds, HashMap<&str, (&str, Ranges)>) {
    let mut sections = input.split("\n\n");

    fn parse_seeds(line: &str) -> Vec<usize> {
        let seed_list = &line.split_once(": ").unwrap().1;
        seed_list
            .split_ascii_whitespace()
            .map(|v| v.parse().unwrap())
            .collect_vec()
    }

    fn parse_map_range(line: &str) -> (usize, usize, usize) {
        line.splitn(3, ' ')
            .map(|v| v.parse::<usize>().unwrap())
            .collect_tuple()
            .unwrap()
    }

    fn parse_map_section(section: &str) -> (&str, (&str, Ranges)) {
        let mut lines = section.lines();
        let name = lines.next().unwrap().split_once(' ').unwrap().0;
        let (src_type, target_type) = name.split_once("-to-").unwrap();
        let ranges = lines.map(parse_map_range).collect_vec();
        (src_type, (target_type, ranges))
    }

    let seeds = parse_seeds(sections.next().unwrap());
    let maps = sections.map(parse_map_section).collect();

    (seeds, maps)
}
