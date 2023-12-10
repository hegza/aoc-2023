use itertools::Itertools;
use std::{fmt, ops};

const INPUT: &str = include_str!("inputs/day10.txt");

#[derive(Debug, Clone, Copy, PartialEq)]
struct Co(usize, usize);

impl ops::Add for Co {
    type Output = Co;

    fn add(self, rhs: Self) -> Self::Output {
        Co(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl ops::Add<Dir> for Co {
    type Output = Co;

    fn add(self, rhs: Dir) -> Self::Output {
        let tuple = self.as_tuple();
        let delta = rhs.as_delta();
        (tuple.0 as isize + delta.0, tuple.1 as isize + delta.1).into()
    }
}

impl From<(usize, usize)> for Co {
    fn from(value: (usize, usize)) -> Self {
        Self(value.0, value.1)
    }
}

impl From<(isize, isize)> for Co {
    fn from(value: (isize, isize)) -> Self {
        Self(value.0 as usize, value.1 as usize)
    }
}

impl Co {
    fn as_tuple(&self) -> (usize, usize) {
        (self.0, self.1)
    }

    fn as_rowcol(&self) -> (usize, usize) {
        (self.0 + 1, self.1 + 1)
    }
}

#[derive(Debug, Clone, Copy)]
enum Dir {
    North,
    South,
    East,
    West,
}

impl Dir {
    fn as_delta(&self) -> (isize, isize) {
        match self {
            Dir::North => (-1, 0),
            Dir::South => (1, 0),
            Dir::East => (0, 1),
            Dir::West => (0, -1),
        }
    }
}

fn pipe_dirs(pipe_sym: char) -> &'static [Dir] {
    use Dir::*;
    match pipe_sym {
        '|' => &[North, South],
        '-' => &[East, West],
        'L' => &[North, East],
        'J' => &[North, West],
        '7' => &[South, West],
        'F' => &[South, East],
        '.' => &[],
        'S' => &[North, South, East, West],
        _ => panic!(),
    }
}

fn connects_to(origin: Co, pipe_sym: char) -> Vec<Co> {
    pipe_dirs(pipe_sym)
        .into_iter()
        .map(|dir| origin + *dir)
        .collect()
}

fn conns(origin: Co, map: &[Vec<char>]) -> Vec<Co> {
    let pipe_sym = map[origin.0][origin.1];

    connects_to(origin, pipe_sym)
        .into_iter()
        .filter(|dest| connects_to(*dest, map[dest.0][dest.1]).contains(&origin))
        .collect_vec()
}

fn main() -> anyhow::Result<()> {
    let lines = INPUT.lines();

    let map = lines
        .into_iter()
        .map(|line| line.chars().collect_vec())
        .collect_vec();

    let start: Co = map
        .iter()
        .enumerate()
        .find_map(|(row, line)| {
            line.iter()
                .position(|&c| c == 'S')
                .and_then(|col| Some((row, col)))
        })
        .unwrap()
        .into();

    let mut prev = start;
    let mut cursor = start;
    let mut step_count = 0;
    loop {
        let next = conns(cursor, &map)
            .into_iter()
            .find(|conn| *conn != prev)
            .unwrap();
        prev = cursor;
        cursor = next;
        step_count += 1;

        if cursor == start {
            break;
        }
    }

    println!("cycle len: {}", step_count);
    println!("furthest: {}", step_count / 2);

    Ok(())
}
