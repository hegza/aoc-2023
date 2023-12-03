use std::num::ParseIntError;

const INPUT: &str = include_str!("inputs/day3.txt");
const SYMS: &[char] = &['$', '&', '=', '*', '#', '@', '%', '/', '+', '-'];

#[derive(PartialEq)]
enum State {
    None,
    FormNum((String, Vec<(usize, usize)>)),
}

fn parts_with_positions(
    cmap: &[Vec<char>],
) -> Result<Vec<(i64, Vec<(usize, usize)>)>, ParseIntError> {
    // Use a state machine to process the map into parts with a list of associated positions
    let mut parts = Vec::new();
    let mut s = State::None;

    for (row, line) in cmap.iter().enumerate() {
        for (col, c) in line.iter().enumerate() {
            use State as S;
            match (c, &mut s) {
                (c, S::None) if c.is_numeric() => {
                    s = S::FormNum((String::from(*c), vec![(row, col)]));
                }
                (c, S::FormNum((cs, pl))) if c.is_numeric() => {
                    cs.push(*c);
                    pl.push((row, col));
                }
                (c, S::None) if *c == '.' || SYMS.contains(c) => {}
                (c, S::FormNum((cs, pl))) if *c == '.' || SYMS.contains(c) => {
                    parts.push((cs.parse::<i64>()?, pl.clone()));
                    s = S::None;
                }
                (c, _) => panic!("{c}"),
            }
        }
    }
    // Push the final number if there was still one being constructed
    if let State::FormNum((cs, pl)) = s {
        parts.push((cs.parse::<i64>()?, pl));
    }
    Ok(parts)
}

fn main() -> anyhow::Result<()> {
    let lines = INPUT.lines();

    let cmap = lines
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<_>>();

    let parts = parts_with_positions(&cmap)?;

    println!("{}", part1::solve(&cmap, &parts)?);
    println!("{}", part2::solve(&cmap, &parts)?);
    Ok(())
}

mod part1 {
    use crate::SYMS;
    use aoc_2023::adjacents;

    pub(crate) fn solve(
        cmap: &[Vec<char>],
        parts: &[(i64, Vec<(usize, usize)>)],
    ) -> anyhow::Result<i64> {
        let parts = parts.into_iter().filter_map(|(part_num, pl)| {
            pl.iter()
                .any(|&pos| is_sym_adjacent(pos, SYMS, cmap))
                .then_some(part_num)
        });

        let part_sum = parts.sum::<i64>();

        Ok(part_sum)
    }

    fn is_sym_adjacent<'a>(pos: (usize, usize), syms: &[char], map: &[Vec<char>]) -> bool {
        adjacents((pos.0, pos.1), map.len(), map[0].len())
            .any(|(row, col)| syms.contains(&map[row][col]))
    }
}

mod part2 {
    use aoc_2023::adjacents;
    use itertools::Itertools;

    pub(crate) fn solve(
        cmap: &[Vec<char>],
        parts: &[(i64, Vec<(usize, usize)>)],
    ) -> anyhow::Result<i64> {
        let stars = cmap.iter().enumerate().flat_map(|(row, line)| {
            line.iter()
                .enumerate()
                .filter_map(move |(col, &ch)| (ch == '*').then_some((row, col)))
        });
        let gears = stars.filter_map(|(row, col)| {
            let ns = adjacent_numbers((row, col), cmap.len(), cmap[0].len(), parts);
            (ns.len() == 2).then_some(ns)
        });
        let gear_ratios = gears.map(|ns| ns.into_iter().product::<i64>()).sum::<i64>();

        Ok(gear_ratios)
    }

    /// Returns part numbers adjacent to given position
    fn adjacent_numbers(
        pos: (usize, usize),
        rows: usize,
        cols: usize,
        parts: &[(i64, Vec<(usize, usize)>)],
    ) -> Vec<i64> {
        let adjacents = adjacents(pos, rows, cols).collect_vec();
        parts
            .iter()
            .filter_map(|(part_num, part_positions)| {
                part_positions
                    .iter()
                    .any(|part_pos| adjacents.contains(part_pos))
                    .then_some(*part_num)
            })
            .collect()
    }
}
