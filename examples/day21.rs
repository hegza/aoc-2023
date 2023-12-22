use itertools::Itertools;
use std::{
    cmp,
    collections::{BTreeMap, HashMap, HashSet},
};

const INPUT: &str = include_str!("inputs/day21.txt");
const TEST_INPUT: &str = include_str!("inputs/day21_test.txt");

type Co = (usize, usize);

fn wrap(co: &(isize, isize), rows: usize, cols: usize) -> Co {
    let rows = rows as isize;
    let cols = cols as isize;

    let mut row = co.0 % rows;
    let mut col = co.1 % cols;

    if row.is_negative() {
        row = rows + row;
    }
    if col.is_negative() {
        col = cols + col;
    }

    (row as usize, col as usize)
}

fn reach(
    start: (isize, isize),
    step_count: usize,
    grid: &[Vec<char>],
    cheat: bool,
) -> Vec<(isize, isize)> {
    let rows = grid.len();
    let cols = grid[0].len();

    let mut cos = vec![(start.0 as isize, start.1 as isize)];
    for _n in 0..step_count {
        cos = cos
            .iter()
            // Find coordinates reachable from this position
            .flat_map(|co| {
                [(-1, 0), (1, 0), (0, -1), (0, 1)]
                    .into_iter()
                    .filter_map(move |ofs| {
                        let inext = (co.0 + ofs.0, co.1 + ofs.1);
                        let next_grid_co = wrap(&inext, rows, cols);
                        (grid[next_grid_co.0][next_grid_co.1] == '.').then_some(inext)
                    })
            })
            .unique()
            .collect_vec();
    }

    cos
}

#[derive(Clone, Eq, PartialEq)]
struct Reach {
    // `n` steps reaches...
    n: usize,
    // `cos` coordinates
    cos: Vec<(usize, usize)>,
}

impl Ord for Reach {
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        // Notice that the we flip the ordering on costs.
        // In case of a tie we compare positions - this step is necessary
        // to make implementations of `PartialEq` and `Ord` consistent.
        self.n.cmp(&other.n)
    }
}

// `PartialOrd` needs to be implemented as well.
impl PartialOrd for Reach {
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn main() -> anyhow::Result<()> {
    let p1 = part1::solve();
    assert_eq!(p1, 3658);

    /*
    let (test_grid, test_start) = parse_input(TEST_INPUT);
    let test_start = (test_start.0 as isize, test_start.1 as isize);
    for step_count in [6, 10, 50, 100, 500, 1000, 5000] {
        let r = reach(test_start, step_count, &test_grid).len();
        println!("{} -> {}", step_count, r);
    }
    */

    let (grid, start) = parse_input(INPUT);
    let start = (start.0 as isize, start.1 as isize);

    let (rows, cols) = (grid.len(), grid[0].len());
    let parity0_grid = (0..rows).map(|row| (0..cols).map(|col| {}));

    let r = reach(start, 26501365, &grid, true).len();
    println!("Part 2: {r}");

    Ok(())
}

fn parse_input(input: &str) -> (Vec<Vec<char>>, (usize, usize)) {
    let mut start = (0, 0);
    (
        input
            .lines()
            .enumerate()
            .map(|(row, line)| {
                line.chars()
                    .enumerate()
                    .map(|(col, c)| {
                        if c == 'S' {
                            start = (row, col);
                            '.'
                        } else {
                            c
                        }
                    })
                    .collect_vec()
            })
            .collect_vec(),
        start,
    )
}

mod part1 {
    use itertools::Itertools;

    use crate::{parse_input, Co, INPUT};

    fn try_add(co: &Co, offset: (isize, isize), rows: usize, cols: usize) -> Option<Co> {
        let row = co.0.checked_add_signed(offset.0);
        let col = co.1.checked_add_signed(offset.1);
        row.zip(col).filter(|&(row, col)| row < rows && col < cols)
    }

    fn reach(start: Co, step_count: usize, grid: &[Vec<char>]) -> usize {
        let rows = grid.len();
        let cols = grid[0].len();

        let mut cos = vec![start];
        for _n in 0..step_count {
            cos = cos
                .iter()
                // Find coordinates reachable from this position
                .flat_map(|co| {
                    [(-1, 0), (1, 0), (0, -1), (0, 1)]
                        .into_iter()
                        .filter_map(move |ofs| try_add(&co, ofs, rows, cols))
                        .filter(|next| grid[next.0][next.1] == '.')
                })
                .unique()
                .collect_vec();
        }

        cos.len()
    }

    pub(crate) fn solve() -> i64 {
        let (grid, start) = parse_input(INPUT);
        reach(start, 64, &grid) as i64
    }
}

/*
fn logreach(start: (isize, isize), step_count: usize, grid: &[Vec<char>]) -> Vec<(isize, isize)> {
    // Maps each coordinate to the coordinates reachable in n steps (with possible duplicates due to wrapping)
    let mut memo: HashMap<(usize, usize), BTreeMap<usize, Vec<(usize, usize)>>> = HashMap::new();

    let (rows, cols) = (grid.len(), grid[0].len());
    let wrap = |co: (isize, isize)| wrap(&co, rows, cols);

    // Init the memo with a heap containing the initial step for each coordinate
    for row in 0..grid.len() {
        for col in 0..grid[0].len() {
            let mut local = BTreeMap::new();
            let r = reach((row as isize, col as isize), 1, grid)
                .into_iter()
                // 'r' may contain duplicates after this. This is intended.
                .map(|co| wrap(co))
                .collect_vec();
            local.insert(1, r);
            memo.insert((row, col), local);
        }
    }

    let mut cos = vec![(start.0, start.1)];
    let mut steps_taken = 0;
    loop {
        // Where can I get to from `cos` in N steps where N is highest available in all of the respective heaps?

        // Get the best step available
        let best_step = cos
            .iter()
            .filter_map(|co| {
                let co = wrap(*co);
                memo.get(&co)
                    .and_then(|btree| btree.last_key_value())
                    .map(|(n, _)| *n)
            })
            .max()
            .unwrap();
        println!("Steps taken: {steps_taken}, best step: {best_step}");

        if steps_taken + best_step > step_count {
            let rem_steps = step_count - steps_taken;
            println!("Counting the rest (N = {}) manually", rem_steps);
            // Count the rest manually
            return cos
                .into_iter()
                .flat_map(|co| reach(co, rem_steps, grid))
                .collect_vec();
        }

        // Take the best step and fill in missing
        cos = cos
            .into_iter()
            .flat_map(|co| {
                let co = wrap(co);
                let local_steps = memo.entry(co).or_insert(BTreeMap::new());

                if let Some((local_best, local_best_cos)) =
                    local_steps.last_key_value().map(|(a, b)| (*a, b.clone()))
                {
                    // Take the remaining steps for parity
                    if local_best < best_step {
                        let rem_steps = best_step - local_best;
                        let parity_steps = local_best_cos
                            .iter()
                            .flat_map(|nco| {
                                let ico = (nco.0 as isize, nco.1 as isize);
                                let r = reach(ico, rem_steps, grid);

                                // Be sure to store this valuable result
                                memo.entry(*nco).or_insert(BTreeMap::new()).insert(
                                    rem_steps,
                                    r.iter()
                                        // 'r' may contain duplicates after this. This is intended.
                                        .map(|co| wrap(*co))
                                        .collect_vec(),
                                );
                                memo.entry(co).or_insert(BTreeMap::new()).insert(
                                    rem_steps + 1,
                                    r.iter()
                                        // 'r' may contain duplicates after this. This is intended.
                                        .map(|co| wrap(*co))
                                        .collect_vec(),
                                );

                                r
                            })
                            .collect_vec();
                        parity_steps
                    } else {
                        local_best_cos
                            .iter()
                            .map(|(row, col)| (*row as isize, *col as isize))
                            .collect_vec()
                    }
                } else {
                    // Take all the steps for parity
                    let rem_steps = best_step;
                    let parity_steps = {
                        let ico = (co.0 as isize, co.1 as isize);
                        let r = reach(ico, rem_steps, grid);
                        // Be sure to store this valuable result
                        memo.entry(co).or_insert(BTreeMap::new()).insert(
                            rem_steps,
                            r.iter()
                                // 'r' may contain duplicates after this. This is intended.
                                .map(|co| wrap(*co))
                                .collect_vec(),
                        );

                        r
                    };
                    parity_steps
                }
            })
            .collect_vec();

        steps_taken += best_step;
        if steps_taken == step_count {
            return cos;
        }
    }
}
*/
