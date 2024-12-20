use itertools::Itertools;
use std::{collections::*, iter};

const INPUT: &str = include_str!("inputs/day17.txt");
const TEST_INPUT: &str = include_str!("inputs/day17_test.txt");
const CUSTOM_INPUT: &str = include_str!("inputs/day17_custom.txt");

type Co = (usize, usize);

fn try_add(co: &Co, offset: (isize, isize), rows: usize, cols: usize) -> Option<Co> {
    let row = co.0.checked_add_signed(offset.0);
    let col = co.1.checked_add_signed(offset.1);
    row.zip(col).filter(|&(row, col)| row < rows && col < cols)
}

fn possible_steps(
    to: Co,
    path: &[Co],
    rows: usize,
    cols: usize,
) -> impl Iterator<Item = (usize, usize)> + '_ {
    [(-1, 0), (1, 0), (0, -1), (0, 1)]
        .into_iter()
        // Avoid walls
        .filter_map(move |offset| try_add(&to, offset, rows, cols))
        // Do not go backwards
        .filter(|next| {
            if path.len() <= 1 {
                true
            } else {
                *next != path[path.len() - 2]
            }
        })
        // Do not revisit nodes in a given path (cycling paths are always longer than no cycles)
        .filter(|next| !path.contains(next))
        // Avoid going more than 3 times in the same direction (note the 4th element from arriving on a given col or row)
        .filter(|next| {
            if path.len() <= 3 {
                return true;
            }

            let (rows, cols): (Vec<usize>, Vec<usize>) = path[path.len() - 4..]
                .iter()
                .chain(iter::once(next))
                .map(|&(r, c)| (r, c))
                .unzip();

            !rows.windows(2).all(|w| w[0] == w[1]) && !cols.windows(2).all(|w| w[0] == w[1])
        })
}

const VIZ: bool = true;

fn dfs(co: Co, end: Co, cost: u32, global_min: &mut u32, path: &[Co], grid: &[Vec<u32>], d: usize) {
    if VIZ {
        for (y, row) in grid.iter().enumerate() {
            for (x, c) in row.iter().enumerate() {
                if co == (y, x) {
                    print!("*");
                } else {
                    print!("{c}");
                }
            }
            println!();
        }
    }

    // If this path is costlier than global min, end the path
    if cost > *global_min {
        if VIZ {
            println!("DIED");
            println!();
        }
        return;
    }

    // If this path is at finish and its still cheaper than global min, record cost and end the path
    if co == end {
        *global_min = cost;
        if VIZ {
            println!("FINISHED");
            println!();
        }
        return;
    }

    /*if d >= 10 {
        if VIZ {
            println!("MAXED");
            println!();
        }
        return;
    }*/

    if VIZ {
        println!();
    }

    // Find potential next steps
    let mut potential_next = possible_steps(co, path, grid.len(), grid[0].len()).collect_vec();

    // Optimize order
    potential_next.sort_by(|a, b| grid[a.0][a.1].cmp(&grid[b.0][b.1]));

    potential_next.into_iter().for_each(|next| {
        let mut npath = path.to_vec();
        npath.push(next);
        let ncost = cost + grid[next.0][next.1];
        dfs(next, end, ncost, global_min, &npath, grid, d + 1)
    });
}

fn init_dfs(start: Co, end: Co, grid: &[Vec<u32>]) -> u32 {
    let mut min = u32::MAX;
    dfs(start, end, 0, &mut min, &vec![start], grid, 0);
    min
}

#[derive(Clone, Eq, PartialEq)]
struct State {
    co: Co,
    cost: u32,
    path: Vec<Co>,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.cost.cmp(&other.cost)
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn dijkstra(start: Co, end: Co, grid: &[Vec<u32>]) -> u32 {
    let mut heap = BinaryHeap::new();
    heap.push(State {
        co: start,
        cost: 0,
        path: vec![start],
    });

    let mut global_min = 720;
    let mut min_map = vec![vec![u32::MAX; grid[0].len()]; grid.len()];
    let mut longest_path = 0;

    while let Some(State { co, cost, path }) = heap.pop() {
        if VIZ {
            if path.len() > longest_path {
                longest_path = path.len();
                println!("Path len: {longest_path}");
            }
        }

        // Record cheapest for this position
        if cost < min_map[co.0][co.1] {
            min_map[co.0][co.1] = cost;
        }
        // If this path is costlier than current min to get to this position, end the path
        else {
            continue;
        }

        // If this path is at finish, record cost and end the path
        if co == end {
            global_min = cost;
            if VIZ {
                println!(
                    "Found new min: {}, remaining paths to analyze: {}",
                    global_min,
                    heap.len()
                );
            }
            continue;
        }

        // Record next potential steps
        for state in possible_steps(co, &path, grid.len(), grid[0].len()).map(|next| {
            let mut npath = path.to_vec();
            npath.push(next);
            let ncost = cost + grid[next.0][next.1];
            State {
                co: next,
                cost: ncost,
                path: npath,
            }
        }) {
            heap.push(state);
        }
    }

    global_min - grid[start.0][start.1]
}

fn main() -> anyhow::Result<()> {
    let lines = INPUT.lines();

    let grid = lines
        .into_iter()
        .map(|line| line.chars().map(|c| c.to_digit(10).unwrap()).collect_vec())
        .collect_vec();

    let min = dijkstra((0, 0), (grid.len() - 1, grid[0].len() - 1), &grid) as i64;

    println!("{min}");

    Ok(())
}
