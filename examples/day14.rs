use std::iter;

use itertools::Itertools;

const INPUT: &str = include_str!("inputs/day14.txt");

#[derive(PartialEq, Clone, Copy)]
enum Dir {
    North,
    South,
    East,
    West,
}

impl Dir {
    fn offset(&self) -> (isize, isize) {
        match self {
            Dir::North => (-1, 0),
            Dir::South => (1, 0),
            Dir::East => (0, 1),
            Dir::West => (0, -1),
        }
    }
    fn offset_from(&self, pos: (usize, usize), n: isize) -> (usize, usize) {
        let of = self.offset();
        (
            (of.0 * n + pos.0 as isize) as usize,
            (of.1 * n + pos.1 as isize) as usize,
        )
    }
}

fn can_move(start_pos: (usize, usize), dir: Dir, rocks: &[Vec<char>]) -> usize {
    let mut pos = start_pos;
    let mut n = 0;
    loop {
        // Row zero -> cannot move at all
        if (dir == Dir::North && pos.0 == 0)
            || (dir == Dir::West && pos.1 == 0)
            || (dir == Dir::South && pos.0 == rocks.len() - 1)
            || (dir == Dir::East && pos.1 == rocks[0].len() - 1)
        {
            return n;
        }
        let npos = dir.offset_from(pos, 1);
        // Free, try next
        if rocks[npos.0][npos.1] == '.' {
            n += 1;
            pos = npos;
        }
        // Not free, return previous value
        else {
            return n;
        }
    }
}

fn roll(start_pos: (usize, usize), dir: Dir, rocks: &[Vec<char>]) -> (usize, usize) {
    let offset = can_move(start_pos, dir, rocks);

    dir.offset_from(start_pos, offset as isize)
}

fn find_cycle(v: &[usize]) -> Option<(usize, usize)> {
    for start in 0..(v.len() - 1) {
        for end in (start + 1)..v.len() {
            let cmp = &v[start..end];
            let len = cmp.len();
            // HACK:
            if len < 500 {
                continue;
            }
            for x in 0..(v.len() - len) {
                if start != x && cmp == &v[x..x + len] {
                    println!("Cycle detected, offset: {}, length: {}", start, len);
                    return Some((start, len));
                }
            }
        }
    }
    None
}

fn main() -> anyhow::Result<()> {
    let lines = INPUT.lines();

    let rocks = lines.map(|line| line.chars().collect_vec()).collect_vec();

    /*
        let mut rocks_in_col = vec![0; rocks[0].len()];
        for line in &rocks {
            for (idx, c) in line.iter().enumerate() {
                if c == &'O' {
                    rocks_in_col[idx] += 1;
                }
            }
        }
    */

    let mut moved_rocks = rocks.clone();
    let it = iter::repeat([Dir::North, Dir::West, Dir::South, Dir::East])
        .enumerate()
        .take(1000000000);
    let mut load_trace = vec![];
    let mut cycle = (0, 0);
    for (spin_idx, spin_dirs) in it {
        // Spin 4 times
        spin_dirs.iter().for_each(|spin_dir| {
            let mut row_it = 0..moved_rocks.len();

            while let Some(row) = if spin_dir == &Dir::South {
                row_it.next_back()
            } else {
                row_it.next()
            } {
                let line = moved_rocks[row].clone();
                let mut col_it = 0..line.len();
                while let Some(col) = if spin_dir == &Dir::East {
                    col_it.next_back()
                } else {
                    col_it.next()
                } {
                    let c = line[col];
                    let pos = (row, col);
                    // Move rocks
                    if c == 'O' {
                        let npos = roll(pos, *spin_dir, &moved_rocks);
                        moved_rocks[pos.0][pos.1] = '.';
                        moved_rocks[npos.0][npos.1] = 'O';
                    }
                }
            }
        });
        let load: usize = calc_load(&moved_rocks);
        load_trace.push(load);
        println!("spin idx: {spin_idx}, load: {load}");
        if spin_idx % 10_000 == 0 {
            println!("spin idx: {} k", spin_idx / 1_000);
        }
        if let Some(fcycle) = find_cycle(&load_trace) {
            println!("Cycle: {:?}", fcycle);
            cycle = fcycle;
            break;
        }
    }

    let total = 1000000000;
    let load = load_trace
        .iter()
        .take(cycle.0)
        .chain(load_trace[cycle.0..cycle.0 + cycle.1].iter())
        .take(total - cycle.0)
        .last()
        .unwrap();

    //let load: usize = calc_load(&moved_rocks);

    println!("{load}");

    Ok(())
}

fn calc_load(moved_rocks: &[Vec<char>]) -> usize {
    moved_rocks
        .iter()
        .rev()
        .enumerate()
        .map(|(row, x)| (row + 1, x))
        .map(|(load_per_rock, line)| line.iter().filter(|&c| c == &'O').count() * load_per_rock)
        .sum()
}
