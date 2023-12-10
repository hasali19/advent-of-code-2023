#![feature(coroutines, iter_from_coroutine)]

use std::collections::{HashMap, HashSet};
use std::ops::Index;

use aoc2023::aoc_solution;
use itertools::Itertools;
use pathfinding::directed::dfs::dfs_reach;
use pathfinding::directed::dijkstra::dijkstra_all;

fn main() -> eyre::Result<()> {
    aoc_solution(10, |input| {
        part_1(input)?;
        part_2(input)?;
        Ok(())
    })
}

fn part_1(input: &str) -> eyre::Result<()> {
    let grid = Grid::parse_from(input);

    let start = (0..grid.width)
        .cartesian_product(0..grid.height)
        .find(|&pos| grid[pos] == 'S')
        .unwrap();

    // Find the shortest distance to every node reachable from the start
    let result = dijkstra_all(&start, |&pos| {
        connected_pipes(&grid, pos).into_iter().map(|p| (p, 1))
    });

    // Find the largest distance cost
    let distance = result.values().map(|(_, d)| d).max().unwrap();

    println!("{distance}");

    Ok(())
}

fn part_2(input: &str) -> eyre::Result<()> {
    let grid = Grid::parse_from(input);

    let start = (0..grid.width)
        .cartesian_product(0..grid.height)
        .find(|&pos| grid[pos] == 'S')
        .unwrap();

    let start_type = get_start_pipe_type(&grid, start);

    // Find the set of nodes that are part of the main cycle
    let cycle_nodes = dfs_reach(start, |&pos| connected_pipes(&grid, pos)).collect::<HashSet<_>>();

    // Filter out all pipes from the grid that are not part of the cycle
    // Also set the start tile to its actual pipe type
    let grid = grid.map_into(|pos, c| {
        if start == pos {
            start_type
        } else if cycle_nodes.contains(&pos) {
            c
        } else {
            '.'
        }
    });

    println!("{}", count_inner_tiles(&grid));

    Ok(())
}

struct Grid {
    tiles: Vec<Vec<char>>,
    width: usize,
    height: usize,
}

impl Grid {
    fn parse_from(input: &str) -> Grid {
        let tiles = input
            .lines()
            .map(|line| line.chars().collect_vec())
            .collect_vec();

        let width = tiles[0].len();
        let height = tiles.len();

        Grid {
            tiles,
            width,
            height,
        }
    }

    fn map_into(self, mut mapper: impl FnMut((usize, usize), char) -> char) -> Grid {
        let tiles = self
            .tiles
            .into_iter()
            .enumerate()
            .map(|(y, row)| {
                row.into_iter()
                    .enumerate()
                    .map(|(x, c)| mapper((x, y), c))
                    .collect_vec()
            })
            .collect_vec();

        Grid {
            tiles,
            width: self.width,
            height: self.height,
        }
    }
}

impl Index<(usize, usize)> for Grid {
    type Output = char;

    fn index(&self, (x, y): (usize, usize)) -> &Self::Output {
        &self.tiles[y][x]
    }
}

/// Determines the coordinates of pipes connected to a given pipe.
fn connected_pipes(grid: &Grid, (x, y): (usize, usize)) -> [(usize, usize); 2] {
    let mut len = 0;
    let mut successors = [(0, 0); 2];

    let mut push = |v| {
        assert!(len < 2);
        successors[len] = v;
        len += 1;
    };

    if y > 0 && "S|LJ".contains(grid[(x, y)]) && "|7F".contains(grid[(x, y - 1)]) {
        push((x, y - 1));
    }

    if y + 1 < grid.height && "S|7F".contains(grid[(x, y)]) && "|LJ".contains(grid[(x, y + 1)]) {
        push((x, y + 1));
    }

    if x > 0 && "S-J7".contains(grid[(x, y)]) && "-LF".contains(grid[(x - 1, y)]) {
        push((x - 1, y));
    }

    if x + 1 < grid.width && "S-LF".contains(grid[(x, y)]) && "-J7".contains(grid[(x + 1, y)]) {
        push((x + 1, y));
    }

    successors
}

/// Determines the type of the start pipe, given its two connected pipes.
fn get_start_pipe_type(grid: &Grid, start: (usize, usize)) -> char {
    type Direction = (isize, isize);

    let [a, b] = connected_pipes(grid, start);

    let a = (
        a.0 as isize - start.0 as isize,
        a.1 as isize - start.1 as isize,
    );

    let b = (
        b.0 as isize - start.0 as isize,
        b.1 as isize - start.1 as isize,
    );

    let north = (0, -1);
    let south = (0, 1);
    let east = (1, 0);
    let west = (-1, 0);

    fn build_map(
        values: &[((Direction, Direction), char)],
    ) -> HashMap<(Direction, Direction), char> {
        HashMap::from_iter(
            values
                .iter()
                .copied()
                .flat_map(|((a, b), c)| [((a, b), c), ((b, a), c)].into_iter()),
        )
    }

    let map = build_map(&[
        ((north, south), '|'),
        ((east, west), '-'),
        ((north, east), 'L'),
        ((north, west), 'J'),
        ((south, west), '7'),
        ((south, east), 'F'),
    ]);

    map.get(&(a, b)).copied().unwrap()
}

/// Counts the number of tiles that fall within the loop in the grid.
fn count_inner_tiles(grid: &Grid) -> i32 {
    let mut inside_count = 0;

    // This works by going through the grid row by row, and maintaining a
    // flag to indicate whether we are currently inside the pipe loop.
    // Each time a vertical pipe is encountered, the flag is flipped.
    // Note that only {|,J,L} are used to flip the flag. Intuitively, it
    // might make sense to consider {F,7} too, as these also represent
    // vertical pipe sections. However, this would be wrong in the case
    // of e.g. L7, which is essentially equivalent to a zigzagging |.
    // Therefore, it is necessary to consider only pipes that move vertically
    // in the same direction, i.e. you can use either {|,J,L} (all move up)
    // or {|,F,7} (all move down) but not both.

    for row in &grid.tiles {
        let mut is_inside = false;
        for &c in row {
            if is_inside && c == '.' {
                inside_count += 1;
            }

            if "|JL".contains(c) {
                is_inside = !is_inside;
            }
        }
    }

    inside_count
}
