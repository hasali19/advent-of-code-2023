use aoc2023::aoc_solution;
use itertools::Itertools;

fn main() -> eyre::Result<()> {
    aoc_solution(18, |input| {
        part_1(input)?;
        part_2(input)?;
        Ok(())
    })
}

fn part_1(input: &str) -> eyre::Result<()> {
    let instructions = input
        .lines()
        .map(|line| {
            let (dir, n, _colour) = line.split_ascii_whitespace().collect_tuple().unwrap();
            (dir, n.parse::<u32>().unwrap())
        })
        .collect_vec();

    println!("{}", shoelace(&instructions));

    Ok(())
}

fn part_2(input: &str) -> eyre::Result<()> {
    let instructions = input
        .lines()
        .map(|line| {
            let (_, _, hex) = line.split_ascii_whitespace().collect_tuple().unwrap();

            let hex = hex
                .trim_start_matches('(')
                .trim_end_matches(')')
                .trim_start_matches('#');

            let mut n = 0;
            for c in hex[..5].chars() {
                n *= 16;
                n += c.to_digit(16).unwrap();
            }

            let dir = match &hex[5..] {
                "0" => "R",
                "1" => "D",
                "2" => "L",
                "3" => "U",
                _ => unreachable!(),
            };

            (dir, n)
        })
        .collect_vec();

    println!("{}", shoelace(&instructions));

    Ok(())
}

/// Shoelace algorithm calculates the area of a polygon given the coordinates of its vertices.
fn shoelace(instructions: &[(&str, u32)]) -> isize {
    let mut pos = (0, 0);
    let mut trench_len = 0;
    let mut vertices = vec![];

    for &(dir, n) in instructions {
        vertices.push(pos);

        let delta = match dir {
            "R" => (1, 0),
            "L" => (-1, 0),
            "U" => (0, -1),
            "D" => (0, 1),
            _ => unreachable!(),
        };

        pos.0 += delta.0 * n as isize;
        pos.1 += delta.1 * n as isize;

        trench_len += n as isize;
    }

    let mut x_sum = 0;
    let mut y_sum = 0;
    for i in 0..vertices.len() {
        x_sum += vertices[i].0 * vertices[(i + 1) % vertices.len()].1;
        y_sum += vertices[i].1 * vertices[(i + 1) % vertices.len()].0;
    }

    ((x_sum - y_sum).abs() / 2) + (trench_len / 2) + 1
}
