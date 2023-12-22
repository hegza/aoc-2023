use itertools::Itertools;
use std::{
    collections::HashSet,
    fmt,
    ops::{Deref, DerefMut, RangeInclusive},
};

const INPUT: &str = include_str!("inputs/day22.txt");
const _TEST_INPUT: &str = include_str!("inputs/day22_test.txt");

type R = (usize, usize);

fn try_add(
    co: &(usize, usize, usize),
    offset: Offset,
    xs: usize,
    ys: usize,
    zs: usize,
) -> Option<Co> {
    let x = co.0.checked_add_signed(offset.0);
    let y = co.1.checked_add_signed(offset.1);
    let z = co.2.checked_add_signed(offset.2);
    x.zip(y)
        .zip(z)
        .map(|((x, y), z)| (x, y, z))
        .filter(|&(x, y, z)| (x < xs && y < ys && z < zs))
        .map(|(x, y, z)| Co((x, y, z)))
}

struct Co((usize, usize, usize));

impl Deref for Co {
    type Target = (usize, usize, usize);

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Co {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Co {
    pub fn as_tuple(&self) -> (usize, usize, usize) {
        (self.0 .0, self.0 .1, self.0 .2)
    }
}

type Offset = (isize, isize, isize);

impl From<(usize, usize, usize)> for Co {
    fn from(value: (usize, usize, usize)) -> Self {
        Self(value)
    }
}

#[derive(Clone)]
struct Block {
    xs: R,
    ys: R,
    zs: R,
}

impl fmt::Debug for Block {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Block")
            .field("x", &(self.xs.0, self.xs.1))
            .field("y", &(self.ys.0, self.ys.1))
            .field("z", &(self.zs.0, self.zs.1))
            .finish()
    }
}

impl Block {
    fn from_cos(a: Co, b: Co) -> Self {
        let (a, b) = (a.as_tuple(), b.as_tuple());
        assert!(a.0 <= b.0 && a.1 <= b.1 && a.2 <= b.2);
        let xs = (a.0, b.0);
        let ys = (a.1, b.1);
        let zs = (a.2, b.2);
        let ret = Self { xs, ys, zs };
        ret.validate();
        ret
    }

    fn validate(&self) {
        let differ_count = if self.xs.0 != self.xs.1 { 1 } else { 0 }
            + if self.ys.0 != self.ys.1 { 1 } else { 0 }
            + if self.zs.0 != self.zs.1 { 1 } else { 0 };
        assert!(
            (differ_count == 1)
                || (self.xs.0 == self.xs.1 && self.ys.0 == self.ys.1 && self.zs.0 == self.zs.1),
            "{:?}",
            &self
        );
    }

    fn validate_with_grid(&self, idx: usize, grid: &mut [Vec<Vec<Option<usize>>>]) {
        self.validate();
        for (x, y, z) in self.coords() {
            assert_eq!(grid[z][y][x], Some(idx));
        }
    }

    fn xmax(&self) -> usize {
        self.xs.1
    }
    fn ymax(&self) -> usize {
        self.ys.1
    }
    fn zmin(&self) -> usize {
        self.zs.0
    }
    fn zmax(&self) -> usize {
        self.zs.1
    }

    fn zs(&self) -> RangeInclusive<usize> {
        self.zs.0..=self.zs.1
    }
    fn ys(&self) -> RangeInclusive<usize> {
        self.ys.0..=self.ys.1
    }
    fn xs(&self) -> RangeInclusive<usize> {
        self.xs.0..=self.xs.1
    }

    fn translated(&self, offset: Offset) -> Option<Block> {
        let t = |v: usize, o: isize| v.checked_add_signed(o);

        let (xmin, xmax) = (t(self.xs.0, offset.0)?, t(self.xs.1, offset.0)?);
        let (ymin, ymax) = (t(self.ys.0, offset.1)?, t(self.ys.1, offset.1)?);
        let (zmin, zmax) = (t(self.zs.0, offset.2)?, t(self.zs.1, offset.2)?);

        let ret = Block {
            xs: (xmin, xmax),
            ys: (ymin, ymax),
            zs: (zmin, zmax),
        };
        ret.validate();
        Some(ret)
    }

    fn coords(&self) -> impl Iterator<Item = (usize, usize, usize)> {
        self.zs()
            .cartesian_product(self.ys())
            .cartesian_product(self.xs())
            .map(|((z, y), x)| (x, y, z))
    }
}

fn render(blocks: &[Block]) -> Vec<Vec<Vec<Option<usize>>>> {
    let xlen = blocks.iter().map(|b| b.xmax()).max().unwrap() + 1;
    let ylen = blocks.iter().map(|b| b.ymax()).max().unwrap() + 1;
    let zlen = blocks.iter().map(|b| b.zmax()).max().unwrap() + 1;

    let mut grid = vec![vec![vec![None; xlen]; ylen]; zlen];

    for (idx, b) in blocks.iter().enumerate() {
        for z in b.zs() {
            for y in b.ys() {
                for x in b.xs() {
                    grid[z][y][x] = Some(idx);
                }
            }
        }
    }

    grid
}

/// N.b., block must match block idx in grid
fn translate(block: &mut Block, offset: Offset, idx: usize, grid: &mut [Vec<Vec<Option<usize>>>]) {
    block.validate_with_grid(idx, grid);
    let ncoords = block
        .coords()
        .map(|(x, y, z)| {
            try_add(
                &(x, y, z),
                offset,
                grid[0][0].len(),
                grid[0].len(),
                grid.len(),
            )
            .unwrap()
            .as_tuple()
        })
        .collect_vec();
    erase_coords(block.coords(), grid, idx);
    render_coords(ncoords.into_iter(), grid, idx);
    *block = block.translated(offset).unwrap();
    block.validate_with_grid(idx, grid);
}

fn render_coords(
    coords: impl Iterator<Item = (usize, usize, usize)>,
    grid: &mut [Vec<Vec<Option<usize>>>],
    idx: usize,
) {
    for (x, y, z) in coords {
        assert_eq!(grid[z][y][x], None);
        grid[z][y][x] = Some(idx);
    }
}

fn erase_coords(
    coords: impl Iterator<Item = (usize, usize, usize)>,
    grid: &mut [Vec<Vec<Option<usize>>>],
    idx: usize,
) {
    for (x, y, z) in coords {
        assert_eq!(grid[z][y][x], Some(idx));
        grid[z][y][x] = None;
    }
}

fn _print_zlayer(z: usize, grid: &[Vec<Vec<Option<usize>>>]) {
    for yline in &grid[z] {
        for x in yline {
            if let Some(n) = x {
                print!("{n}");
            } else {
                print!(" ");
            }
        }
        println!();
    }
}

fn main() -> anyhow::Result<()> {
    let p1 = part1::solve(INPUT);
    assert_eq!(p1, 477);
    println!("Part 1: {p1}");

    let mut airborne_blocks = parse_input(&INPUT);
    airborne_blocks.sort_by(|a, b| a.zmin().cmp(&b.zmin()));

    let airborne_grid = render(&airborne_blocks);

    // Bricks fall
    let mut grid = airborne_grid.clone();
    let mut blocks = airborne_blocks.into_iter().enumerate().collect_vec();

    while simulates(&mut blocks, &mut grid, true).len() != 0 {}

    //print_tower(&grid);

    let fall_count: usize = blocks
        .iter()
        .map(|&(idx, _)| {
            let mut blocks = blocks.clone();
            let mut grid = grid.clone();

            remove_block(idx, &mut blocks, &mut grid);

            let mut all_fallers: HashSet<usize> = HashSet::new();
            loop {
                let fallers = simulates(&mut blocks, &mut grid, true);
                let fallers = fallers.into_iter().collect::<HashSet<_>>();
                // Stop iterating if there are no new fallers
                if fallers.is_subset(&all_fallers) {
                    break;
                }
                all_fallers.extend(fallers);
            }

            let fall_count = all_fallers.len();
            /*println!(
                "Disintegrating block {} would cause {} bricks to fall",
                idx, fall_count
            );*/
            fall_count
        })
        .sum();

    println!("Part 2: {fall_count}");

    Ok(())
}

fn remove_block(
    idx: usize,
    blocks: &mut Vec<(usize, Block)>,
    grid: &mut Vec<Vec<Vec<Option<usize>>>>,
) {
    let remblock = blocks.swap_remove(idx).1;
    erase_coords(remblock.coords(), grid, idx);
}

fn _print_tower(grid: &Vec<Vec<Vec<Option<usize>>>>) {
    println!("---");
    for z in (0..grid.len()).rev() {
        _print_zlayer(z, grid);
        println!("---");
    }
}

fn simulates(
    blocks: &mut [(usize, Block)],
    grid: &mut [Vec<Vec<Option<usize>>>],
    actuate: bool,
) -> Vec<usize> {
    let mut moved_blocks = vec![];
    for (idx, b) in blocks.iter_mut() {
        if let Some(below) = b.translated((0, 0, -1)) {
            let can_move = below
                .coords()
                .all(|(x, y, z)| grid[z][y][x].is_none() || grid[z][y][x] == Some(*idx));
            if can_move {
                if actuate {
                    translate(b, (0, 0, -1), *idx, grid);
                }
                //println!("{idx} would move");
                moved_blocks.push(*idx);
            }
        }
    }
    moved_blocks
}

fn parse_input(input: &str) -> Vec<Block> {
    let mut idx = 0;
    input
        .lines()
        .map(|line| {
            let mut it = line.split('~');

            let mut it = (0..2).map(|_| {
                let cos = it.next().unwrap();
                let mut cos_it = cos.split(',');
                let mut cos_it = (0..3).map(|_| cos_it.next().unwrap().parse::<usize>().unwrap());

                let (x, y, z) = (
                    cos_it.next().unwrap(),
                    cos_it.next().unwrap(),
                    cos_it.next().unwrap(),
                );
                idx += 1;
                (x, y, z)
            });

            Block::from_cos(it.next().unwrap().into(), it.next().unwrap().into())
        })
        .collect_vec()
}

mod part1 {
    use itertools::Itertools;

    use crate::{erase_coords, parse_input, render, simulates};

    pub(crate) fn solve(input: &str) -> i64 {
        let mut airborne_blocks = parse_input(&input);
        airborne_blocks.sort_by(|a, b| a.zmin().cmp(&b.zmin()));

        let airborne_grid = render(&airborne_blocks);

        // Bricks fall
        let mut grid = airborne_grid.clone();
        let mut blocks = airborne_blocks.into_iter().enumerate().collect_vec();

        while simulates(&mut blocks, &mut grid, true).len() != 0 {}

        //print_tower(&grid);

        let safe_to_remove = blocks
            .iter()
            .map(|(idx, _)| *idx)
            .filter(|&idx| {
                let mut blocks = blocks.clone();
                let remblock = blocks.swap_remove(idx).1;

                let mut grid = grid.clone();
                erase_coords(remblock.coords(), &mut grid, idx);

                simulates(&mut blocks, &mut grid, false).len() == 0
            })
            .count();

        safe_to_remove as i64
    }
}
