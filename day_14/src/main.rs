use std::collections::HashMap;
use std::ops::{Index, IndexMut};

use aoc2023::aoc_solution;
use itertools::Itertools;

fn main() -> eyre::Result<()> {
    aoc_solution(14, |input| {
        part_1(input)?;
        part_2(input)?;
        Ok(())
    })
}

fn part_1(input: &str) -> eyre::Result<()> {
    let mut grid = Grid::parse_from(input);

    grid.tilt();

    println!("{}", grid.find_load());

    Ok(())
}

fn part_2(input: &str) -> eyre::Result<()> {
    let mut grid = Grid::parse_from(input);

    let mut map = HashMap::new();

    // Start iterating for a billion cycles
    for i in 0..1_000_000_000 {
        grid.cycle();

        // Record the current state along with the iteration index. If we've seen
        // this state before, there is a loop.
        if let Some(prev_i) = map.insert(grid.cells.clone(), i) {
            // If there is a loop, we can skip most remaining iterations
            // and just do a few more to get to 1 billion, depending on the
            // length of the loop.
            let loop_len = i - prev_i;
            let remaining = (1_000_000_000 - i - 1) % loop_len;

            for _ in 0..remaining {
                grid.cycle();
            }

            break;
        }
    }

    println!("{}", grid.find_load());

    Ok(())
}

enum Orientation {
    North,
    West,
    South,
    East,
}

impl Orientation {
    fn next(&self) -> Orientation {
        match self {
            Orientation::North => Orientation::West,
            Orientation::West => Orientation::South,
            Orientation::South => Orientation::East,
            Orientation::East => Orientation::North,
        }
    }
}

struct Grid {
    cells: Vec<Vec<char>>,
    width: isize,
    height: isize,
    orientation: Orientation,
}

impl Grid {
    fn parse_from(input: &str) -> Grid {
        let cells = input
            .lines()
            .map(|line| line.chars().collect_vec())
            .collect_vec();

        let width = cells[0].len() as isize;
        let height = cells.len() as isize;

        Grid {
            cells,
            width,
            height,
            orientation: Orientation::North,
        }
    }

    fn width(&self) -> isize {
        match self.orientation {
            Orientation::North => self.width,
            Orientation::West => self.height,
            Orientation::South => self.width,
            Orientation::East => self.height,
        }
    }

    fn height(&self) -> isize {
        match self.orientation {
            Orientation::North => self.height,
            Orientation::West => self.width,
            Orientation::South => self.height,
            Orientation::East => self.width,
        }
    }

    fn reorient_index(&self, (x, y): (isize, isize)) -> (isize, isize) {
        match self.orientation {
            Orientation::North => (x, y),
            Orientation::West => (y, self.height - x - 1),
            Orientation::South => (self.width - x - 1, self.height - y - 1),
            Orientation::East => (self.width - y - 1, x),
        }
    }

    fn tilt(&mut self) {
        for x in 0..self.width() {
            let mut wall = -1;
            for y in 0..self.height() {
                if self[(x, y)] == 'O' {
                    wall += 1;
                    self[(x, y)] = '.';
                    self[(x, wall)] = 'O';
                } else if self[(x, y)] == '#' {
                    wall = y;
                }
            }
        }
    }

    fn rotate(&mut self) {
        self.orientation = self.orientation.next();
    }

    fn cycle(&mut self) {
        for _ in 0..4 {
            self.tilt();
            self.rotate();
        }
    }

    fn find_load(&self) -> isize {
        let mut load = 0;
        for x in 0..self.width {
            for y in 0..self.height {
                if self.cells[y as usize][x as usize] == 'O' {
                    load += self.height - y;
                }
            }
        }
        load
    }
}

impl Index<(isize, isize)> for Grid {
    type Output = char;

    fn index(&self, (x, y): (isize, isize)) -> &Self::Output {
        let (x, y) = self.reorient_index((x, y));
        &self.cells[y as usize][x as usize]
    }
}

impl IndexMut<(isize, isize)> for Grid {
    fn index_mut(&mut self, (x, y): (isize, isize)) -> &mut Self::Output {
        let (x, y) = self.reorient_index((x, y));
        &mut self.cells[y as usize][x as usize]
    }
}
