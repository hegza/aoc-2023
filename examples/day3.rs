use itertools::Itertools;

const INPUT: &str = include_str!("inputs/day3.txt");
const SYMS: &[char] = &['$', '&', '=', '*', '#', '@', '%', '/', '+', '-'];

#[derive(PartialEq)]
enum State {
    None,
    FormNum((String, Vec<(usize, usize)>)),
}

fn main() -> anyhow::Result<()> {
    let lines = INPUT.lines();

    let cmap = lines
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<_>>();

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
    if let State::FormNum((cs, pl)) = s {
        parts.push((cs.parse::<i64>()?, pl));
    }

    println!("{}", part1(&cmap, &parts)?);
    println!("{}", part2(&cmap, &parts)?);
    Ok(())
}

fn adjacents(
    pos: (usize, usize),
    rows: usize,
    cols: usize,
) -> impl Iterator<Item = (usize, usize)> {
    [
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ]
    .into_iter()
    .filter_map(move |(ro, co)| {
        let r = pos.0 as isize + ro;
        let c = pos.1 as isize + co;
        if r == -1 || c == -1 || r as usize == rows || c as usize == cols {
            return None;
        }
        Some((r as usize, c as usize))
    })
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

fn is_sym_adjacent<'a>(pos: (usize, usize), syms: &[char], map: &[Vec<char>]) -> bool {
    adjacents((pos.0, pos.1), map.len(), map[0].len())
        .any(|(row, col)| syms.contains(&map[row][col]))
}

fn part1(cmap: &[Vec<char>], parts: &[(i64, Vec<(usize, usize)>)]) -> anyhow::Result<i64> {
    let parts = parts.into_iter().filter_map(|(part_num, pl)| {
        if pl.iter().any(|&pos| is_sym_adjacent(pos, SYMS, &cmap)) {
            Some(part_num)
        } else {
            None
        }
    });

    let part_sum = parts.sum::<i64>();

    Ok(part_sum)
}

fn part2(cmap: &[Vec<char>], parts: &[(i64, Vec<(usize, usize)>)]) -> anyhow::Result<i64> {
    let stars = cmap.iter().enumerate().flat_map(|(row, line)| {
        line.iter()
            .enumerate()
            .filter_map(move |(col, &ch)| (ch == '*').then_some((row, col)))
    });
    let gears = stars.filter_map(|(row, col)| {
        let ns = adjacent_numbers((row, col), cmap.len(), cmap[0].len(), &parts);
        (ns.len() == 2).then_some(ns)
    });
    let gear_ratios = gears.map(|ns| ns.into_iter().product::<i64>()).sum::<i64>();

    Ok(gear_ratios)
}
