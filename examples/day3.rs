use std::collections::HashSet;

const INPUT: &str = include_str!("inputs/day3.txt");

fn main() -> anyhow::Result<()> {
    let syms: HashSet<char> = HashSet::from(['$', '&', '=', '*', '#', '@', '%', '/', '+', '-']);

    let lines = INPUT.lines();

    let map: Vec<Vec<_>> = lines
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<_>>();

    #[derive(PartialEq)]
    enum State {
        None,
        FormNum((String, Vec<(isize, isize)>)),
    }

    let mut parts: Vec<(i64, Vec<(isize, isize)>)> = Vec::new();
    let mut s = State::None;

    for (row, line) in map.iter().enumerate() {
        for (col, c) in line.iter().enumerate() {
            use State as S;
            match (c, &mut s) {
                (c, S::None) if c.is_numeric() => {
                    s = S::FormNum((String::from(*c), vec![(row as isize, col as isize)]));
                }
                (c, S::FormNum((cs, pl))) if c.is_numeric() => {
                    cs.push(*c);
                    pl.push((row as isize, col as isize));
                }
                (c, S::None) if *c == '.' || syms.contains(c) => {}
                (c, S::FormNum((cs, pl))) if *c == '.' || syms.contains(c) => {
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

    let parts = parts
        .into_iter()
        .filter_map(|(part_num, pl)| {
            if pl
                .iter()
                .any(|pos| is_sym_adjacent(*pos, syms.iter(), &map))
            {
                Some(part_num)
            } else {
                None
            }
        })
        .collect::<Vec<_>>();

    println!("{}", parts.into_iter().sum::<i64>());

    Ok(())
}

fn is_sym_adjacent<'a>(
    pos: (isize, isize),
    syms: impl Iterator<Item = &'a char> + Clone,
    map: &[Vec<char>],
) -> bool {
    [
        (-1isize, -1isize),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ]
    .iter()
    .any(|(row, col)| {
        syms.clone()
            .find(|&sym| {
                let r = pos.0 + row;
                let c = pos.1 + col;
                if r == -1 || c == -1 || r as usize == map.len() || c as usize == map[0].len() {
                    return false;
                }
                map[r as usize][c as usize] == *sym
            })
            .is_some()
    })
}
