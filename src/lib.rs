use std::path::{Path, PathBuf};

use clap::Parser;
use eyre::{eyre, Context};

#[derive(Parser)]
struct Args {
    input: Option<PathBuf>,
}

pub fn aoc_solution(day: u32, runner: impl Fn(&str) -> eyre::Result<()>) -> eyre::Result<()> {
    color_eyre::install().unwrap();

    let args = Args::parse();

    let input_path = args.input.unwrap_or_else(|| {
        Path::new(env!("CARGO_MANIFEST_DIR"))
            .join(format!("day_{day}"))
            .join("input.txt")
    });

    let input = std::fs::read_to_string(&input_path)
        .wrap_err_with(|| eyre!("failed to read path: {input_path:?}"))?;

    runner(&input)
}
