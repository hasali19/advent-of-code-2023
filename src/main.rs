use std::env;
use std::path::PathBuf;

use clap::{Args, Parser};
use duct::cmd;

#[derive(Parser)]
enum Command {
    Init(InitArgs),
}

#[derive(Args)]
struct InitArgs {
    day: u32,
}

fn main() -> eyre::Result<()> {
    let command = Command::parse();
    match command {
        Command::Init(InitArgs { day }) => init(day),
    }
}

fn init(day: u32) -> eyre::Result<()> {
    let input_url = format!("https://adventofcode.com/2023/day/{day}/input");
    let solution_path = PathBuf::from(format!("day_{day}"));
    let aoc_session = env::var("AOC_SESSION")?;

    if !solution_path.exists() {
        cmd!("cargo", "new", &solution_path).run()?;

        std::fs::write(
            solution_path.join("Cargo.toml"),
            include_str!("template/Cargo.toml.template").replace("<DAY>", &day.to_string()),
        )?;

        std::fs::write(
            solution_path.join("src/main.rs"),
            include_str!("template/main.rs.template").replace("<DAY>", &day.to_string()),
        )?;
    }

    let input = ureq::get(&input_url)
        .set("Cookie", &format!("session={aoc_session}"))
        .call()?
        .into_string()?;

    std::fs::write(format!("day_{day}/input.txt"), input)?;

    Ok(())
}
