use itertools::Itertools;
use std::collections::*;

const INPUT: &str = include_str!("inputs/day16.txt");
const INPUT_TEST: &str = include_str!("inputs/day16_test.txt");

type Co = (usize, usize);

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
enum Dir {
    North,
    East,
    South,
    West,
}

impl Dir {
    fn as_tuple(&self) -> (isize, isize) {
        match self {
            Dir::North => (-1, 0),
            Dir::East => (0, 1),
            Dir::South => (1, 0),
            Dir::West => (0, -1),
        }
    }

    fn is_horizontal(&self) -> bool {
        *self == Dir::East || *self == Dir::West
    }

    fn is_vertical(&self) -> bool {
        *self == Dir::North || *self == Dir::South
    }
}

fn try_add(co: Co, dir: Dir, w: usize, h: usize) -> Option<Co> {
    let d = dir.as_tuple();
    co.0.checked_add_signed(d.0)
        .filter(|&row| row < h)
        .and_then(|a| {
            co.1.checked_add_signed(d.1)
                .filter(|&col| col < w)
                .map(|b| (a, b))
        })
}

fn next(co: Co, dir: Dir, square: char, w: usize, h: usize) -> Box<dyn Iterator<Item = (Co, Dir)>> {
    use Dir as D;
    match square {
        // Continue in the same direction
        '.' => Box::new(try_step(co, dir, w, h).into_iter()),
        '/' => {
            let ndir = match dir {
                D::North => D::East,
                D::East => D::North,
                D::South => D::West,
                D::West => D::South,
            };
            Box::new(try_step(co, ndir, w, h).into_iter())
        }
        '\\' => {
            let ndir = match dir {
                D::North => D::West,
                D::East => D::South,
                D::South => D::East,
                D::West => D::North,
            };
            Box::new(try_step(co, ndir, w, h).into_iter())
        }
        '|' => {
            if dir.is_horizontal() {
                Box::new(try_step(co, Dir::North, w, h).into_iter().chain(try_step(
                    co,
                    Dir::South,
                    w,
                    h,
                )))
            } else {
                // Continue in the same direction
                Box::new(try_step(co, dir, w, h).into_iter())
            }
        }
        '-' => {
            if dir.is_vertical() {
                Box::new(try_step(co, Dir::West, w, h).into_iter().chain(try_step(
                    co,
                    Dir::East,
                    w,
                    h,
                )))
            } else {
                // Continue in the same direction
                Box::new(try_step(co, dir, w, h).into_iter())
            }
        }
        _ => panic!(),
    }
}

fn try_step(co: Co, dir: Dir, w: usize, h: usize) -> Option<((usize, usize), Dir)> {
    try_add(co, dir, w, h).map(|nco| (nco, dir))
}

fn trace(co: Co, dir: Dir, grid: &[Vec<char>], visited: &mut HashSet<(Co, Dir)>, depth: usize) {
    /*
    for (y, line) in grid.iter().enumerate() {
        for (x, c) in line.iter().enumerate() {
            if visited.contains(&(y, x)) {
                print!("#");
            } else {
                print!("{c}");
            }
        }
        println!();
    }
    println!();
    */

    // Break on cycles
    if !visited.insert((co, dir)) {
        return;
    }

    let (w, h) = (grid[0].len(), grid.len());

    next(co, dir, grid[co.0][co.1], w, h)
        .for_each(|(nco, ndir)| trace(nco, ndir, grid, visited, depth + 1));
}

fn count_energized(start: (Co, Dir), grid: &[Vec<char>]) -> usize {
    let mut visited: HashSet<(Co, Dir)> = [].into_iter().collect();
    trace(start.0, start.1, &grid, &mut visited, 0);

    let energized = visited
        .into_iter()
        .map(|(co, _dir)| co)
        .collect::<HashSet<_>>()
        .len();
    energized
}

fn main() -> anyhow::Result<()> {
    let p1 = part1::solve(INPUT);
    assert_eq!(p1, 7608);
    println!("Part 1: {p1}");

    let grid = INPUT
        .lines()
        .map(|line| line.chars().collect_vec())
        .collect_vec();

    let max = (0..grid.len())
        .into_iter()
        .map(|row| ((row, 0), Dir::East))
        .chain(
            (0..grid.len())
                .into_iter()
                .map(|row| ((row, grid[0].len() - 1), Dir::West)),
        )
        .chain(
            (0..grid[0].len())
                .into_iter()
                .map(|col| ((0, col), Dir::South)),
        )
        .chain(
            (0..grid[0].len())
                .into_iter()
                .map(|col| ((grid.len() - 1, col), Dir::North)),
        )
        .map(|start| count_energized(start, &grid))
        .max()
        .unwrap();
    println!("Part 2: {max}");

    Ok(())
}

mod part1 {
    use crate::{trace, Co, Dir};
    use itertools::Itertools;
    use std::collections::HashSet;

    pub(crate) fn solve(input: &str) -> i64 {
        let grid = input
            .lines()
            .map(|line| line.chars().collect_vec())
            .collect_vec();

        let mut visited: HashSet<(Co, Dir)> = [].into_iter().collect();
        trace((0, 0), Dir::East, &grid, &mut visited, 0);

        let energized = visited
            .into_iter()
            .map(|(co, _dir)| co)
            .collect::<HashSet<_>>()
            .len();
        energized as i64
    }
}
