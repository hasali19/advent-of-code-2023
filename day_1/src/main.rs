use aoc2023::aoc_solution;

fn main() -> eyre::Result<()> {
    aoc_solution(1, |input| {
        part_1(input)?;
        part_2(input)?;
        Ok(())
    })
}

fn part_1(input: &str) -> eyre::Result<()> {
    let sum: u32 = input
        .lines()
        .map(|line| {
            let first = line.chars().find_map(|c| c.to_digit(10)).unwrap();
            let last = line.chars().rev().find_map(|c| c.to_digit(10)).unwrap();
            first * 10 + last
        })
        .sum();

    println!("{sum}");

    Ok(())
}

fn part_2(input: &str) -> eyre::Result<()> {
    let sum: u32 = input
        .lines()
        .map(|line| {
            let chars = line.chars().collect::<Vec<char>>();

            let first = (0..chars.len())
                .find_map(|i| parse_number_at(&chars[i..]))
                .unwrap();

            let last = (0..chars.len())
                .rev()
                .find_map(|i| parse_number_at(&chars[i..]))
                .unwrap();

            first * 10 + last
        })
        .sum();

    println!("{sum}");

    Ok(())
}

fn parse_number_at(input_chars: &[char]) -> Option<u32> {
    if input_chars[0].is_ascii_digit() {
        return input_chars[0].to_digit(10);
    }

    let numbers = [
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];

    for (i, number) in numbers.iter().enumerate() {
        if input_chars.len() < number.len() {
            continue;
        }

        if number
            .chars()
            .zip(input_chars.iter().take(number.len()))
            .all(|(a, b)| a == *b)
        {
            return Some(i as u32 + 1);
        }
    }

    None
}
