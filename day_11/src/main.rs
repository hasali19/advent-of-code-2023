use aoc2023::aoc_solution;
use itertools::Itertools;

fn main() -> eyre::Result<()> {
    aoc_solution(11, |input| {
        part_1(input)?;
        part_2(input)?;
        Ok(())
    })
}

fn part_1(input: &str) -> eyre::Result<()> {
    solve(input, 2);
    Ok(())
}

fn part_2(input: &str) -> eyre::Result<()> {
    solve(input, 1_000_000);
    Ok(())
}

fn solve(input: &str, expansion_factor: usize) {
    let grid = input
        .lines()
        .map(|row| row.chars().collect_vec())
        .collect_vec();

    let width = grid[0].len();
    let height = grid.len();

    let row_sizes = (0..height)
        .map(|y| {
            if grid[y].iter().all(|&c| c == '.') {
                expansion_factor
            } else {
                1
            }
        })
        .collect_vec();

    let col_sizes = (0..width)
        .map(|x| {
            if (0..height).all(|y| grid[y][x] == '.') {
                expansion_factor
            } else {
                1
            }
        })
        .collect_vec();

    let galaxies = (0..width)
        .cartesian_product(0..height)
        .filter(|&(x, y)| grid[y][x] == '#')
        .collect_vec();

    let distance_sum: usize = galaxies
        .iter()
        .tuple_combinations()
        .map(|(&a, &b)| {
            let dx: usize = (usize::min(a.0, b.0) + 1..=usize::max(a.0, b.0))
                .map(|col| col_sizes[col])
                .sum();

            let dy: usize = (usize::min(a.1, b.1) + 1..=usize::max(a.1, b.1))
                .map(|row| row_sizes[row])
                .sum();

            dx + dy
        })
        .sum();

    println!("{distance_sum}");
}
