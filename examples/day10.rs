use aoc_2023::{adjacents, find2d, Co2};
use itertools::Itertools;
use std::{collections::HashSet, num::TryFromIntError, ops};

const INPUT: &str = include_str!("inputs/day10.txt");

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Co(Co2<usize>);

impl ops::Deref for Co {
    type Target = Co2<usize>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<(usize, usize)> for Co {
    fn from(value: (usize, usize)) -> Self {
        Co(Co2::from(value))
    }
}

impl ops::Sub for Co {
    type Output = (isize, isize);

    fn sub(self, rhs: Self) -> Self::Output {
        (
            self.0 .0 as isize - rhs.0 .0 as isize,
            self.0 .1 as isize - rhs.0 .1 as isize,
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

impl TryFrom<(isize, isize)> for Co {
    type Error = TryFromIntError;

    fn try_from(value: (isize, isize)) -> Result<Self, Self::Error> {
        let row = usize::try_from(value.0)?;
        let col = usize::try_from(value.1)?;
        Ok(Self(Co2(row, col)))
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
    let pipe_sym = map[origin.0 .0][origin.0 .1];

    connects_to(origin, pipe_sym)
        .into_iter()
        .filter(|dest| connects_to(*dest, map[dest.0 .0][dest.0 .1]).contains(&origin))
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
            let co: Co = co.into();
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

    // Find the 'S' and make it into a coordinate
    let start: Co = find2d(&'S', &map).unwrap().into();

    // Trace the path / cycle by following the pipes
    let path = trace_path(start, &map);

    println!("Part 1: {}", path.len() / 2);

    let enclosed = find_enclosed(path, map);

    println!("Part 2: {}", enclosed.len());

    Ok(())
}

fn find_enclosed(path: Vec<Co>, map: Vec<Vec<char>>) -> HashSet<Co> {
    let mut enclosed = HashSet::new();

    let mut path_it = path.iter();
    let start = *path_it.next().unwrap();
    let mut prev = start;

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
    enclosed
}

fn trace_path(start: Co, map: &[Vec<char>]) -> Vec<Co> {
    let mut prev = start;
    let mut cursor = start;
    let mut path = vec![start];
    loop {
        // Update cursor
        let next = conns(cursor, map)
            .into_iter()
            .find(|conn| *conn != prev)
            .unwrap();
        prev = cursor;
        cursor = next;

        if cursor == start {
            break;
        }

        path.push(cursor);
    }
    path
}

#[test]
fn day10_part1() {
    let lines = INPUT.lines();

    let map = lines
        .into_iter()
        .map(|line| line.chars().collect_vec())
        .collect_vec();

    // Find the 'S' and make it into a coordinate
    let start: Co = find2d(&'S', &map).unwrap().into();

    // Trace the path / cycle by following the pipes
    let path = trace_path(start, &map);

    assert_eq!(path.len() / 2, 6714);
}

#[test]
fn day10_part2() {
    let lines = INPUT.lines();

    let map = lines
        .into_iter()
        .map(|line| line.chars().collect_vec())
        .collect_vec();

    // Find the 'S' and make it into a coordinate
    let start: Co = find2d(&'S', &map).unwrap().into();

    // Trace the path / cycle by following the pipes
    let path = trace_path(start, &map);

    let enclosed = find_enclosed(path, map);

    assert_eq!(enclosed.len(), 429);
}
