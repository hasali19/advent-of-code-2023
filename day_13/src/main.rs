use aoc2023::aoc_solution;
use itertools::Itertools;

fn main() -> eyre::Result<()> {
    aoc_solution(13, |input| {
        part_1(input)?;
        part_2(input)?;
        Ok(())
    })
}

type Pattern = Vec<Vec<char>>;

fn part_1(input: &str) -> eyre::Result<()> {
    solve(input, 0);
    Ok(())
}

fn part_2(input: &str) -> eyre::Result<()> {
    solve(input, 1);
    Ok(())
}

fn solve(input: &str, expected_error_count: usize) {
    let patterns = input
        .split("\n\n")
        .map(|section| {
            section
                .lines()
                .map(|line| line.chars().collect_vec())
                .collect_vec()
        })
        .collect_vec();

    let sum = patterns
        .iter()
        .map(|pattern| {
            let width = pattern[0].len();
            let height = pattern.len();

            (0..width)
                .tuple_windows()
                .find(|mid_x| is_reflected_x(pattern, *mid_x, expected_error_count))
                .map(|(x1, _)| x1 + 1)
                .or_else(|| {
                    (0..height)
                        .tuple_windows()
                        .find(|mid_y| is_reflected_y(pattern, *mid_y, expected_error_count))
                        .map(|(y1, _)| (y1 + 1) * 100)
                })
                .expect("should be at least one line of symmetry in the pattern")
        })
        .sum::<usize>();

    println!("{sum}");
}

fn is_reflected_x(pattern: &Pattern, mid_x: (usize, usize), expected_error_count: usize) -> bool {
    let width = pattern[0].len();
    let height = pattern.len();

    let mut error_count = 0;
    for x in 0..=usize::min(mid_x.0, width - mid_x.1 - 1) {
        error_count += (0..height)
            .filter(|&y| pattern[y][mid_x.0 - x] != pattern[y][mid_x.1 + x])
            .count();
    }

    error_count == expected_error_count
}

fn is_reflected_y(pattern: &Pattern, mid_y: (usize, usize), expected_error_count: usize) -> bool {
    let width = pattern[0].len();
    let height = pattern.len();

    let mut error_count = 0;
    for y in 0..=usize::min(mid_y.0, height - mid_y.1 - 1) {
        error_count += (0..width)
            .filter(|&x| pattern[mid_y.0 - y][x] != pattern[mid_y.1 + y][x])
            .count();
    }

    error_count == expected_error_count
}
