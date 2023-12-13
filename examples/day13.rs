use itertools::Itertools;

const INPUT: &str = include_str!("inputs/day13.txt");

fn transpose<T: Clone>(v: &Vec<Vec<T>>) -> Vec<Vec<T>> {
    let mut ret = vec![Vec::new(); v[0].len()];
    v.iter().for_each(|line| {
        line.iter().enumerate().for_each(|(idx, c)| {
            ret[idx].push(c.clone());
        })
    });
    ret
}

/// Find seams where two adjacent lines match -> potential mirrors
fn seams(lines: &[Vec<bool>]) -> impl Iterator<Item = usize> + Clone + '_ {
    lines
        .iter()
        .tuple_windows()
        .positions(|(l1, l2)| l1 == l2)
        // The seam is between the lines, +1 is more intuitive
        .map(|x| x + 1)
}

/// Check if given line refracts a mirror in pattern
fn is_mirror(line: usize, pat: &[Vec<bool>]) -> bool {
    let must_match_rows = line.min(pat.len() - line);
    (0..must_match_rows).all(|row| pat[line - row - 1] == pat[line + row])
}

/// Finds the first mirror in the pattern
fn find_mirror(pat: &[Vec<bool>]) -> Option<usize> {
    seams(&pat).find(|line| is_mirror(*line, pat))
}

fn parse(input: &str) -> Vec<Vec<Vec<bool>>> {
    let mut lines = input.lines();

    let mut patterns = vec![];
    let mut cur_pat = vec![];
    while let Some(line) = lines.next() {
        if line.is_empty() {
            patterns.push(cur_pat);
            cur_pat = vec![];
        } else {
            cur_pat.push(
                line.chars()
                    .map(|c| match c {
                        '#' => true,
                        '.' => false,
                        _ => panic!(),
                    })
                    .collect_vec(),
            )
        }
    }
    patterns.push(cur_pat);
    patterns
}

fn main() -> anyhow::Result<()> {
    let p1 = part1::solve(INPUT);
    assert_eq!(p1, 35210);
    println!("Part 1: {p1}");

    let p2 = part2::solve(INPUT);
    assert_eq!(p2, 31974);
    println!("Part 2: {p2}");

    Ok(())
}

mod part1 {
    use crate::{find_mirror, parse, transpose};

    pub(crate) fn solve(input: &str) -> i64 {
        let patterns = parse(input);
        let tpatterns = patterns.iter().map(|pattern| transpose(pattern));

        let sum: usize = patterns
            .iter()
            .zip(tpatterns)
            .map(|(pat, tpat)| {
                if let Some(line) = find_mirror(pat) {
                    line * 100
                } else {
                    find_mirror(&tpat).unwrap()
                }
            })
            .sum();
        sum as i64
    }
}

mod part2 {
    use crate::{find_mirror, is_mirror, parse, seams, transpose};
    use itertools::Itertools;

    pub(crate) fn solve(input: &str) -> i64 {
        let patterns = parse(input);

        let combinations = patterns.iter().map(|pat| {
            let w = pat[0].len();
            let h = pat.len();
            (0..h).cartesian_product(0..w).map(move |(row, col)| {
                let mut npat = pat.clone();
                // Flip one char
                npat[row][col] = !npat[row][col];
                npat
            })
        });

        patterns
            .iter()
            .zip(combinations)
            .map(|(orig, many_pats)| {
                let orig_hline = find_mirror(&orig);
                let orig_vline = find_mirror(&transpose(&orig));
                many_pats
                    .map(|pat| (pat.clone(), transpose(&pat)))
                    .find_map(|(pat, tpat)| {
                        let potential_hline = seams(&pat).find(|line| {
                            is_mirror(*line, &pat) && !orig_hline.is_some_and(|orig| orig == *line)
                        });
                        if let Some(line) = potential_hline {
                            Some(line * 100)
                        } else {
                            let potential_vline = seams(&tpat).find(|line| {
                                is_mirror(*line, &tpat)
                                    && !orig_vline.is_some_and(|orig| orig == *line)
                            });
                            potential_vline
                        }
                    })
                    .unwrap()
            })
            .sum::<usize>() as i64
    }
}
