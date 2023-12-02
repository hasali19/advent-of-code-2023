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
    let solution_path = PathBuf::from(format!("solutions/day_{day}"));
    let aoc_session = env::var("AOC_SESSION")?;
    let aoc_cookie = format!("cookie: session={aoc_session}");

    if !solution_path.exists() {
        cmd!("cargo", "new", solution_path).run()?;
    }

    cmd!("xh", input_url, aoc_cookie)
        .stdout_path(format!("solutions/day_{day}/input.txt"))
        .run()?;

    Ok(())
}
