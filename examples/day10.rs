use aoc_2023::adjacents;
use itertools::Itertools;
use std::{collections::HashSet, num::TryFromIntError, ops};

const INPUT: &str = include_str!("inputs/day10.txt");

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Co(usize, usize);

impl ops::Add for Co {
    type Output = Co;

    fn add(self, rhs: Self) -> Self::Output {
        Co(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl ops::Sub for Co {
    type Output = (isize, isize);

    fn sub(self, rhs: Self) -> Self::Output {
        (
            self.0 as isize - rhs.0 as isize,
            self.1 as isize - rhs.1 as isize,
        )
    }
}

impl ops::Add<Dir> for Co {
    type Output = (isize, isize);

    fn add(self, rhs: Dir) -> Self::Output {
        let tuple = self.as_tuple();
        let delta = rhs.as_delta();
        (tuple.0 as isize + delta.0, tuple.1 as isize + delta.1)
    }
}

impl From<(usize, usize)> for Co {
    fn from(value: (usize, usize)) -> Self {
        Self(value.0, value.1)
    }
}

impl TryFrom<(isize, isize)> for Co {
    type Error = TryFromIntError;

    fn try_from(value: (isize, isize)) -> Result<Self, Self::Error> {
        let row = usize::try_from(value.0)?;
        let col = usize::try_from(value.1)?;
        Ok(Self(row, col))
    }
}

impl Co {
    fn as_tuple(&self) -> (usize, usize) {
        (self.0, self.1)
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

impl From<(isize, isize)> for Dir {
    fn from(value: (isize, isize)) -> Self {
        match value {
            (-1, 0) => Dir::North,
            (1, 0) => Dir::South,
            (0, 1) => Dir::East,
            (0, -1) => Dir::West,
            _ => panic!("{:?}", value),
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
        .filter_map(|dir| (origin + *dir).try_into().ok())
        .collect()
}

fn conns(origin: Co, map: &[Vec<char>]) -> Vec<Co> {
    let pipe_sym = map[origin.0][origin.1];

    connects_to(origin, pipe_sym)
        .into_iter()
        .filter(|dest| connects_to(*dest, map[dest.0][dest.1]).contains(&origin))
        .collect_vec()
}

fn _starboard(dir: Dir) -> Dir {
    match dir {
        Dir::North => Dir::East,
        Dir::South => Dir::West,
        Dir::East => Dir::South,
        Dir::West => Dir::North,
    }
}

fn _port(dir: Dir) -> Dir {
    match dir {
        Dir::North => Dir::West,
        Dir::South => Dir::East,
        Dir::East => Dir::North,
        Dir::West => Dir::South,
    }
}

fn flood_fill(enclosed: &mut HashSet<Co>, path: &[Co], co: Co, map: &[Vec<char>]) {
    if !path.contains(&co) {
        enclosed.insert(co);
        adjacents(co.as_tuple(), map.len(), map[0].len()).for_each(|co| {
            let co = co.into();
            if !enclosed.contains(&co) && !path.contains(&co) {
                flood_fill(enclosed, path, co, map)
            }
        });
    }
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

    let mut path = vec![start];

    let mut prev = start;
    let mut cursor = start;
    let mut step_count = 0;
    let mut enclosed = HashSet::new();
    loop {
        // Update cursor
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

        path.push(cursor);
    }

    println!("cycle len: {}", step_count);
    println!("furthest: {}", step_count / 2);

    let mut prev = start;
    let mut path_it = path.iter().skip(1);

    while let Some(next) = path_it.next() {
        let dir = Dir::from(*next - prev);

        let closed_co = prev + _port(dir);
        if !(closed_co.0 == -1
            || closed_co.1 == -1
            || closed_co.0 == map.len() as isize
            || closed_co.1 == map[0].len() as isize)
        {
            flood_fill(&mut enclosed, &path, closed_co.try_into().unwrap(), &map);
        }

        prev = *next;

        let closed_co = *next + _port(dir);
        if !(closed_co.0 == -1
            || closed_co.1 == -1
            || closed_co.0 == map.len() as isize
            || closed_co.1 == map[0].len() as isize)
        {
            flood_fill(&mut enclosed, &path, closed_co.try_into().unwrap(), &map);
        }
    }

    println!("Part 2: {}", enclosed.len());

    /*
    for (row, line) in map.iter().enumerate() {
        for (col, c) in line.iter().enumerate() {
            let co: Co = (row, col).into();
            if enclosed.contains(&co) {
                print!("#");
            } else if path.contains(&co) {
                print!("{c}");
            } else {
                print!(" ");
            }
        }
        println!();
    } */

    Ok(())
}
