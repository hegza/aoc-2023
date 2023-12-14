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
    fn offset_from(&self, pos: (usize, usize)) -> (usize, usize) {
        let of = self.offset();
        (
            (of.0 + pos.0 as isize) as usize,
            (of.1 + pos.1 as isize) as usize,
        )
    }
}

fn can_move_north(start_pos: (usize, usize), rocks: &[Vec<char>]) -> usize {
    let mut pos = start_pos.clone();
    let mut n = 0;
    loop {
        // Row zero -> cannot move at all
        if pos.0 == 0 {
            return n;
        }
        let npos = (pos.0 - 1, pos.1);
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

fn roll_north(start_pos: (usize, usize), rocks: &[Vec<char>]) -> (usize, usize) {
    let offset = can_move_north(start_pos, rocks);

    (start_pos.0 - offset, start_pos.1)
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
    for row in 0..moved_rocks.len() {
        let mut moves = vec![];
        let line = &moved_rocks[row];
        for (col, c) in line.iter().enumerate() {
            let pos = (row, col);
            // Move rocks
            if c == &'O' {
                let npos = roll_north(pos, &moved_rocks);
                moves.push((pos, npos));
            }
        }

        for (src, dest) in moves {
            moved_rocks[src.0][src.1] = '.';
            moved_rocks[dest.0][dest.1] = 'O';
        }
    }

    for line in &moved_rocks {
        println!("{}", &line.iter().collect::<String>());
    }

    let load: usize = moved_rocks
        .iter()
        .rev()
        .enumerate()
        .map(|(row, x)| (row + 1, x))
        .map(|(load_per_rock, line)| line.iter().filter(|&c| c == &'O').count() * load_per_rock)
        .sum();

    println!("{load}");

    Ok(())
}
