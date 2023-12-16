use std::collections::HashSet;

const INPUT: &str = include_str!("inputs/day16.txt");
const _INPUT_TEST: &str = include_str!("inputs/day16_test.txt");

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
    let a = co.0.checked_add_signed(d.0);
    let b = co.1.checked_add_signed(d.1);
    a.zip(b).filter(|&(row, col)| row < h && col < w)
}

fn next(co: Co, dir: Dir, square: char, w: usize, h: usize) -> Box<dyn Iterator<Item = (Co, Dir)>> {
    // Continue in the same direction
    let fwd = || Box::new(try_step(co, dir, w, h).into_iter());

    // Turn to new direction
    let turn = |ndir| Box::new(try_step(co, ndir, w, h).into_iter());

    // Split beam in two
    let split = |dir1, dir2| {
        Box::new(
            try_step(co, dir1, w, h)
                .into_iter()
                .chain(try_step(co, dir2, w, h)),
        )
    };

    use Dir as D;
    match square {
        // Continue in the same direction
        '.' => fwd(),
        // Refract
        '/' => {
            let ndir = match dir {
                D::North => D::East,
                D::East => D::North,
                D::South => D::West,
                D::West => D::South,
            };
            turn(ndir)
        }
        // Refract
        '\\' => {
            let ndir = match dir {
                D::North => D::West,
                D::East => D::South,
                D::South => D::East,
                D::West => D::North,
            };
            turn(ndir)
        }
        '|' => {
            // Split in two
            if dir.is_horizontal() {
                split(Dir::North, Dir::South)
            } else {
                // Continue in the same direction
                fwd()
            }
        }
        '-' => {
            // Split in two
            if dir.is_vertical() {
                split(Dir::West, Dir::East)
            } else {
                // Continue in the same direction
                fwd()
            }
        }
        _ => panic!(),
    }
}

fn try_step(co: Co, dir: Dir, w: usize, h: usize) -> Option<((usize, usize), Dir)> {
    try_add(co, dir, w, h).map(|nco| (nco, dir))
}

fn trace(co: Co, dir: Dir, grid: &[Vec<char>], visited: &mut HashSet<(Co, Dir)>) {
    // Break on cycles
    if !visited.insert((co, dir)) {
        return;
    }

    let (w, h) = (grid[0].len(), grid.len());

    next(co, dir, grid[co.0][co.1], w, h).for_each(|(nco, ndir)| trace(nco, ndir, grid, visited));
}

fn count_energized(start: (Co, Dir), grid: &[Vec<char>]) -> usize {
    let mut visited: HashSet<(Co, Dir)> = [].into_iter().collect();
    trace(start.0, start.1, grid, &mut visited);

    visited
        .into_iter()
        .map(|(co, _dir)| co)
        .collect::<HashSet<_>>()
        .len()
}

fn main() -> anyhow::Result<()> {
    let p1 = part1::solve(INPUT);
    assert_eq!(p1, 7608);
    println!("Part 1: {p1}");

    let p2 = part2::solve(INPUT);
    assert_eq!(p2, 8221);
    println!("Part 2: {p2}");

    Ok(())
}

mod part1 {
    use crate::{count_energized, Dir};
    use itertools::Itertools;

    pub(crate) fn solve(input: &str) -> i64 {
        let grid = input
            .lines()
            .map(|line| line.chars().collect_vec())
            .collect_vec();

        count_energized(((0, 0), Dir::East), &grid) as i64
    }
}

mod part2 {
    use itertools::Itertools;

    use crate::{count_energized, Dir};

    pub(crate) fn solve(input: &str) -> i64 {
        let grid = input
            .lines()
            .map(|line| line.chars().collect_vec())
            .collect_vec();

        let max = (0..grid.len())
            .map(|row| ((row, 0), Dir::East))
            .chain((0..grid.len()).map(|row| ((row, grid[0].len() - 1), Dir::West)))
            .chain((0..grid[0].len()).map(|col| ((0, col), Dir::South)))
            .chain((0..grid[0].len()).map(|col| ((grid.len() - 1, col), Dir::North)))
            .map(|start| count_energized(start, &grid))
            .max()
            .unwrap();

        max as i64
    }
}
