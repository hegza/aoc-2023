use itertools::Itertools;
use lazy_static::lazy_static;
use part1::flood_fill;
use regex::Regex;
use std::{collections::HashSet, iter};

use crate::part1::{draw_grid, interpret_part1_input, make_grid};

const INPUT: &str = include_str!("inputs/day18.txt");
const TEST_INPUT: &str = include_str!("inputs/day18_test.txt");
const VIZ: bool = false;

lazy_static! {
    pub static ref RE: Regex =
        Regex::new(r"^([RUDL]) (\d+) \(#([[:alnum:]]{5}?)([[:alnum:]]{1}?)\)").unwrap();
}

type Line = (Co, Co);
type Co = (usize, usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Dir {
    Up,
    Left,
    Down,
    Right,
}

impl Dir {
    fn as_tuple(&self) -> (isize, isize) {
        match self {
            Dir::Up => (-1, 0),
            Dir::Left => (0, -1),
            Dir::Down => (1, 0),
            Dir::Right => (0, 1),
        }
    }
    fn from_char(c: char) -> Self {
        use Dir as D;
        match c {
            'R' => D::Right,
            'D' => D::Down,
            'L' => D::Left,
            'U' => D::Up,
            _ => panic!(),
        }
    }
    fn from_nchar(nc: char) -> Self {
        use Dir as D;
        match nc {
            '0' => D::Right,
            '1' => D::Down,
            '2' => D::Left,
            '3' => D::Up,
            _ => panic!(),
        }
    }
}

fn scale(a: (isize, isize), b: isize) -> (isize, isize) {
    (a.0 * b, a.1 * b)
}

fn add(a: (isize, isize), b: (isize, isize)) -> (isize, isize) {
    (a.0 + b.0, a.1 + b.1)
}

fn norm(x: (isize, isize)) -> (isize, isize) {
    (
        if x.0 == 0 { 0 } else { x.0 / x.0.abs() },
        if x.1 == 0 { 0 } else { x.1 / x.1.abs() },
    )
}

fn try_add(co: &Co, offset: (isize, isize), rows: usize, cols: usize) -> Option<Co> {
    let row = co.0.checked_add_signed(offset.0);
    let col = co.1.checked_add_signed(offset.1);
    row.zip(col).filter(|&(row, col)| row < rows && col < cols)
}

pub(crate) fn flood_map(start: Co, grid: &[Vec<bool>], fill: &mut HashSet<Co>) {
    let (rows, cols) = (grid.len(), grid[0].len());
    let steplist = [(-1, 0), (1, 0), (0, -1), (0, 1)]
        .into_iter()
        .filter_map(|ofs| try_add(&start, ofs, rows, cols))
        .filter(|co| !grid[co.0][co.1])
        .filter(|co| !fill.contains(co))
        .collect_vec();
    for next in steplist.into_iter() {
        fill.insert(next);
        flood_map(next, grid, fill)
    }
}

fn remap(pos: (isize, isize), low_row: isize, low_col: isize) -> Co {
    ((pos.0 - low_row) as usize, (pos.1 - low_col) as usize)
}

fn interpret_part2_input(line: &str) -> (Dir, isize) {
    let mut cap = RE.captures_iter(line);
    let m = cap.next().unwrap();

    let dir = Dir::from_nchar(m[4].chars().next().unwrap());
    let len = isize::from_str_radix(&m[3], 16).unwrap();

    (dir, len)
}

fn hcollides(row: usize, vlines: &[Line]) -> Vec<usize> {
    vlines
        .iter()
        .filter_map(|&(start, end)| {
            let (srow, scol) = start;
            let (erow, _) = end;

            if row >= srow && row <= erow {
                Some(scol)
            } else {
                None
            }
        })
        .collect()
}

fn len(line: &Line) -> usize {
    let (srow, scol) = line.0;
    let (erow, ecol) = line.1;

    // Horizontal
    if srow == erow {
        ecol + 1 - scol
    }
    // Vertical
    else if scol == ecol {
        erow + 1 - srow
    } else {
        panic!()
    }
}

fn part2(input: &str, interpret: Interpret) -> anyhow::Result<i64> {
    let lines = input.lines();

    let instrs = lines
        .into_iter()
        .map(|x| match interpret {
            Interpret::Part1 => interpret_part1_input(x),
            Interpret::Part2 => interpret_part2_input(x),
        })
        .collect_vec();

    let mut pos = (0, 0);
    let mut trace = vec![pos];
    let mut vlines = vec![];
    let mut hlines = vec![];

    for (dir, len) in instrs {
        let npos = add(pos, scale(dir.as_tuple(), len));
        let ydiff = npos.0 - pos.0;
        let xdiff = npos.1 - pos.1;
        if ydiff.is_positive() {
            vlines.push((pos, npos));
        } else if ydiff.is_negative() {
            vlines.push((npos, pos));
        }
        if xdiff.is_positive() {
            hlines.push((pos, npos));
        } else if xdiff.is_negative() {
            hlines.push((npos, pos));
        }
        trace.push(npos);
        pos = npos;
    }

    let low_row = trace
        .iter()
        .min_by(|pos0, pos1| pos0.0.cmp(&pos1.0))
        .unwrap()
        .0;
    let low_col = trace
        .iter()
        .min_by(|pos0, pos1| pos0.1.cmp(&pos1.1))
        .unwrap()
        .1;

    let trace = trace
        .into_iter()
        .map(|pos| remap(pos, low_row, low_col))
        .collect_vec();

    let mut vlines = vlines
        .into_iter()
        .map(|(start, end)| (remap(start, low_row, low_col), remap(end, low_row, low_col)))
        .collect_vec();
    let mut hlines = hlines
        .into_iter()
        .map(|(start, end)| (remap(start, low_row, low_col), remap(end, low_row, low_col)))
        .collect_vec();

    // Sort vlines
    vlines.sort_by(|line_a, line_b| {
        let (a_start, _) = line_a;
        let (b_start, _) = line_b;
        a_start.0.cmp(&b_start.0).then(a_start.1.cmp(&b_start.1))
    });

    // Sort hlines
    hlines.sort_by(|line_a, line_b| {
        let (a_start, _) = line_a;
        let (b_start, _) = line_b;
        a_start.1.cmp(&b_start.1).then(a_start.0.cmp(&b_start.0))
    });

    let mut interesting_rows = trace.iter().map(|co| co.0).unique().collect_vec();
    interesting_rows.sort();

    for co in &trace {
        println!("{co:?}");
    }

    //println!("Inter. rows: {interesting_rows:?}");
    let windows = interesting_rows.windows(2);
    let win_count = windows.clone().count();
    let center_area: usize = windows
        .enumerate()
        .map(|(win_num, win)| {
            let (row0, row1) = (win[0], win[1]);
            let row_count = row1 - row0;

            let mut coll = hcollides(row0 + 1, &vlines);
            coll.sort();

            let fill_ranges = coll.chunks(2);
            let counts = fill_ranges.clone().map(|chunk| {
                let (start, end) = (chunk[0], chunk[1]);
                end - start - 1
            });

            fn interfering_hlines<'a>(
                cols: (usize, usize),
                line_cols: &'a [(usize, usize)],
            ) -> impl Iterator<Item = &'a (usize, usize)> {
                let (start, end) = cols;
                line_cols.iter().filter(move |&&(ostart, oend)| {
                    (oend > start && oend < end)
                        || (ostart > start && ostart < end)
                        || (oend == end && ostart == start)
                })
            }

            fn overlaps_for_interfering_lines(
                cols: (usize, usize),
                line_cols: &[&(usize, usize)],
            ) -> usize {
                let (s, e) = cols;

                if let Some(ret_cols) = line_cols
                    .iter()
                    .find(|&&&(os, oe)| s == os && e == oe)
                    .and_then(|&line| Some(line.1 - line.0 - 1))
                {
                    return ret_cols;
                };

                line_cols
                    .iter()
                    // ???: the error must be here
                    .find(|&&&(os, oe)| s == oe || e == os || s == os || e == oe)
                    .and_then(|&line| Some(line.1 - line.0))
                    .unwrap()
            }

            let extras: usize = {
                let hline_cols_at_row1 = hlines
                    .iter()
                    .filter_map(|hline| {
                        let hline_row = hline.0 .0;
                        assert_eq!(hline.1 .0, hline_row);
                        (hline_row == row1).then_some((hline.0 .1, hline.1 .1))
                    })
                    .collect_vec();
                println!("Hlines at row1 {hline_cols_at_row1:?}");

                let extra_count = fill_ranges.clone().map(|chunk| {
                    let (start, end) = (chunk[0], chunk[1]);
                    let chunk_width = end - start - 1;
                    println!("Chunk_width: {chunk_width}");
                    println!("Request local hline for cols ({start}, {end})");
                    let local_hline_cols =
                        interfering_hlines((start, end), &hline_cols_at_row1).collect_vec();
                    let overlap = overlaps_for_interfering_lines((start, end), &local_hline_cols);
                    println!("Overlap here: {overlap}");

                    chunk_width - overlap
                });
                extra_count.sum()
            };

            /*
                        let hlines_at_row1 = hlines
                            .iter()
                            .filter(|hline| {
                                let hline_row = hline.0 .0;
                                assert_eq!(hline.1 .0, hline_row);
                                hline_row == row1
                            })
                            .collect_vec();
                        let matching_hlines = hlines_at_row1.iter().filter(|hline| {
                            let (start, end) = hline;
                            let (scol, ecol) = (start.1, end.1);
                        });
                        let hline_lens = matching_hlines
                            .iter()
                            .map(|hline| len(hline) - 1)
                            .sum::<usize>();
                        let extra = counts.clone().sum::<usize>() - hline_lens;
            */

            println!(
                "Chunk area: {}x{} + {}",
                (row_count - 1),
                counts.clone().sum::<usize>(),
                extras
            );

            (counts.sum::<usize>() * (row_count - 1) + extras) as usize

            // hlines & vlines are counted separately
        })
        .sum();
    // Add hlines and vlines separately, without corners
    let line_area: usize = hlines
        .iter()
        .chain(vlines.iter())
        .map(|line| {
            // Subtract corners for each line, they are counted separately
            let len = len(line);
            //println!("{line:?}, len = {len}");
            len - 2
        })
        .sum();
    // Add corners separately
    let corner_count = hlines.len() * 2;

    /*
    let specials: usize = interesting_rows[1..interesting_rows.len() - 1]
        .iter()
        .map(|&row| {
            let mut coll = hcollides(row, &vlines);
            coll.sort();
            let fill_ranges = coll.windows(2);
            let counts = fill_ranges.map(|win| {
                let (start, end) = (win[0], win[1]);
                end - start - 1
            });
            let count = counts.sum::<usize>();

            let hlines_here = hlines
                .iter()
                .filter(|hline| {
                    let hline_row = hline.0 .0;
                    assert_eq!(hline.1 .0, hline_row);
                    hline_row == row
                })
                .collect_vec();

            println!("Hlines here: {hlines_here:?}");
            count
                - hlines_here
                    .iter()
                    .map(|hline| len(hline) - 2)
                    .sum::<usize>()
        })
        .sum();
    */
    /*
    let specials: usize = interesting_rows[1..interesting_rows.len() - 1]
        .iter()
        .map(|&row| {
            let mut coll = hcollides(row, &vlines);
            coll.sort();

            let mut sum = 0;
            let mut capture = false;
            let mut it = coll.windows(2);
            while let Some(win) = it.next() {
                let (col0, col1) = (win[0], win[1]);

                if capture {
                    sum += col1 - col0 - 1;
                }
                capture = !capture;
            }
            println!("Row: {}, collisions: {coll:?}, sum: {}", row, sum);
            sum
        })
        .sum();
    */

    println!(
        "Center area: {}, line area: {}, corner count: {}",
        center_area, line_area, corner_count
    );

    if VIZ {
        for hline in &hlines {
            println!("{:?}", hline);
        }
    }

    let rows = trace
        .iter()
        .max_by(|pos0, pos1| pos0.0.cmp(&pos1.0))
        .unwrap()
        .0
        + 1;
    let cols = trace
        .iter()
        .max_by(|pos0, pos1| pos0.1.cmp(&pos1.1))
        .unwrap()
        .1
        + 1;

    if VIZ {
        let mut grid = vec![vec![false; cols]; rows];
        for co in hlines.iter().flat_map(|(start, end)| [start, end]) {
            grid[co.0][co.1] = true;
        }

        /*
        let mut grid = make_grid(&trace);
        flood_fill((1, 1), &mut grid);
        */
        draw_grid(&grid);
    }

    Ok((corner_count + line_area + center_area) as i64)
}

enum Interpret {
    Part1,
    Part2,
}

fn main() -> anyhow::Result<()> {
    println!("Checking part1 interpret 1 test input");
    assert_eq!(part1::solve(TEST_INPUT, Interpret::Part1)?, 62);
    /*println!("Checking part1 interpret 1 main input");
    assert_eq!(part1::solve(INPUT, Interpret::Part1)?, 42317);*/
    println!("Checking part2 interpret 1 test input");
    assert_eq!(part2(TEST_INPUT, Interpret::Part1)?, 62);
    println!("Checking part2 interpret 2 test input");
    assert_eq!(part2(TEST_INPUT, Interpret::Part2)?, 952408144115);
    println!("Checking part2 interpret 2 main  input");
    let p2 = part2(INPUT, Interpret::Part2)?;
    println!("{p2}");
    Ok(())
}

mod part1 {
    use crate::*;
    use itertools::Itertools;
    use std::collections::HashSet;

    pub(crate) fn interpret_part1_input(line: &str) -> (Dir, isize) {
        let mut cap = RE.captures_iter(line);
        let m = cap.next().unwrap();

        let dir = Dir::from_char(m[1].chars().next().unwrap());
        let len = m[2].parse::<isize>().unwrap();

        (dir, len)
    }

    pub(crate) fn solve(input: &str, interpret: Interpret) -> anyhow::Result<i64> {
        let instrs = input
            .lines()
            .map(|x| match interpret {
                Interpret::Part1 => interpret_part1_input(x),
                Interpret::Part2 => interpret_part2_input(x),
            })
            .collect_vec();

        let mut pos = (0, 0);
        let mut trace = vec![pos];

        for (dir, len) in instrs {
            pos = add(pos, scale(dir.as_tuple(), len));
            trace.push(pos);
        }

        let low_row = trace
            .iter()
            .min_by(|pos0, pos1| pos0.0.cmp(&pos1.0))
            .unwrap()
            .0;
        let low_col = trace
            .iter()
            .min_by(|pos0, pos1| pos0.1.cmp(&pos1.1))
            .unwrap()
            .1;

        let trace = trace
            .into_iter()
            .map(|pos| remap(pos, low_row, low_col))
            .collect_vec();

        let mut grid = make_grid(&trace);

        flood_fill((grid.len() / 4, grid[0].len() / 2), &mut grid);

        if VIZ {
            draw_grid(&grid);
        }

        let capacity: i64 = grid
            .into_iter()
            .map(|line| line.iter().filter(|&&x| x).count())
            .sum::<usize>() as i64;

        Ok(capacity)
    }

    pub(crate) fn flood_fill(start: (usize, usize), grid: &mut [Vec<bool>]) {
        let mut filled = HashSet::new();
        filled.insert(start);
        flood_map(start, grid, &mut filled);
        for co in filled.into_iter() {
            grid[co.0][co.1] = true;
        }
    }

    pub(crate) fn make_grid(trace: &[(usize, usize)]) -> Vec<Vec<bool>> {
        let rows = trace
            .iter()
            .max_by(|pos0, pos1| pos0.0.cmp(&pos1.0))
            .unwrap()
            .0
            + 1;
        let cols = trace
            .iter()
            .max_by(|pos0, pos1| pos0.1.cmp(&pos1.1))
            .unwrap()
            .1
            + 1;

        let mut grid = vec![vec![false; cols]; rows];
        for win in trace.windows(2) {
            let co0i = (win[0].0 as isize, win[0].1 as isize);
            let co1i = (win[1].0 as isize, win[1].1 as isize);
            let offset = (co1i.0 - co0i.0, co1i.1 - co0i.1);
            let noffset = norm(offset);

            let mut co = co0i;
            grid[co.0 as usize][co.1 as usize] = true;
            while co != co1i {
                co = add(co, noffset);
                grid[co.0 as usize][co.1 as usize] = true;
            }
        }
        grid
    }

    pub(crate) fn draw_grid(grid: &[Vec<bool>]) {
        for (_row, line) in grid.iter().enumerate() {
            for (_col, b) in line.iter().enumerate() {
                if *b {
                    print!("#");
                } else {
                    print!(".");
                }
            }
            println!()
        }
    }
}
