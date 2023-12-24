use aoc2023::aoc_solution;
use itertools::Itertools;
use nalgebra::{Matrix2, RowVector2, Vector2};

fn main() -> eyre::Result<()> {
    aoc_solution(24, |input| {
        part_1(input)?;
        part_2(input)?;
        Ok(())
    })
}

// const MIN: f64 = 7.0;
// const MAX: f64 = 27.0;
const MIN: f64 = 200000000000000.0;
const MAX: f64 = 400000000000000.0;

fn part_1(input: &str) -> eyre::Result<()> {
    let hailstones = input
        .lines()
        .map(|line| parse_input_line(line).unwrap())
        .collect_vec();

    let intersections = hailstones
        .iter()
        .tuple_combinations()
        .filter_map(|(a, b)| {
            let coeefficients = Matrix2::from_rows(&[
                RowVector2::new(a.1 .1, -a.1 .0),
                RowVector2::new(b.1 .1, -b.1 .0),
            ]);

            let inverse = coeefficients.try_inverse()?;

            let rhs = Vector2::new(
                a.0 .0 * a.1 .1 - a.1 .0 * a.0 .1,
                b.0 .0 * b.1 .1 - b.1 .0 * b.0 .1,
            );

            let intersection = inverse * rhs;

            let t1 = (intersection[0] - a.0 .0) / a.1 .0;
            let t2 = (intersection[0] - b.0 .0) / b.1 .0;

            Some((a, b, (intersection[0], intersection[1]), t1, t2))
        })
        .filter(|&(_, _, (x, y), t1, t2)| {
            (MIN..=MAX).contains(&x) && (MIN..=MAX).contains(&y) && t1 > 0.0 && t2 > 0.0
        })
        .count();

    println!("{intersections}");

    Ok(())
}

fn part_2(input: &str) -> eyre::Result<()> {
    Ok(())
}

#[derive(Debug)]
struct Position(f64, f64, f64);
#[derive(Debug)]
struct Velocity(f64, f64, f64);

fn parse_input_line(line: &str) -> eyre::Result<(Position, Velocity)> {
    let (p, v) = line.split_once(" @ ").unwrap();
    let (px, py, pz) = p.splitn(3, ", ").map(|it| it.trim()).next_tuple().unwrap();
    let (vx, vy, vz) = v.splitn(3, ", ").map(|it| it.trim()).next_tuple().unwrap();
    Ok((
        Position(px.parse()?, py.parse()?, pz.parse()?),
        Velocity(vx.parse()?, vy.parse()?, vz.parse()?),
    ))
}
