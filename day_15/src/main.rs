use aoc2023::aoc_solution;
use winnow::combinator::alt;
use winnow::error::{ErrorKind, ParseError};
use winnow::stream::AsChar;
use winnow::token::take_while;
use winnow::Parser;

fn main() -> eyre::Result<()> {
    aoc_solution(15, |input| {
        part_1(input)?;
        part_2(input)?;
        Ok(())
    })
}

fn part_1(input: &str) -> eyre::Result<()> {
    let mut sum: usize = 0;
    for step in input.trim().split(',') {
        sum += hash(step) as usize;
    }

    println!("{sum}");

    Ok(())
}

fn part_2(input: &str) -> eyre::Result<()> {
    let mut boxes: [Vec<(&str, u32)>; 256] = std::array::from_fn(|_| vec![]);

    for step in input.trim().split(',') {
        let (label, op) = parse_step(step).unwrap();
        let hash = hash(label);
        match op {
            Operation::Dash => {
                boxes[hash as usize].retain(|&(s, _)| s != label);
            }
            Operation::Equals(focal_len) => {
                let lens_box = &mut boxes[hash as usize];
                let lens = lens_box.iter_mut().find(|&&mut (s, _)| s == label);
                if let Some((_, slot)) = lens {
                    *slot = focal_len;
                } else {
                    lens_box.push((label, focal_len));
                }
            }
        }
    }

    let mut focusing_power = 0;
    for (i, b) in boxes.iter().enumerate() {
        for (j, (_, f)) in b.iter().enumerate() {
            focusing_power += (1 + i) * (1 + j) * *f as usize;
        }
    }

    println!("{focusing_power}");

    Ok(())
}

fn hash(input: &str) -> u8 {
    let mut hash: u8 = 0;
    for c in input.as_bytes() {
        hash += c;
        hash = hash.overflowing_mul(17).0;
    }
    hash
}

#[derive(Clone, Copy, Debug)]
enum Operation {
    Dash,
    Equals(u32),
}

fn parse_step(input: &str) -> Result<(&str, Operation), ParseError<&str, ErrorKind>> {
    let label = take_while(1.., AsChar::is_alpha);
    let dash = '-'.value(Operation::Dash);
    let equals = ('=', take_while(1.., '0'..='9').parse_to()).map(|(_, v)| Operation::Equals(v));
    let operation = alt((dash, equals));
    (label, operation).parse(input)
}
