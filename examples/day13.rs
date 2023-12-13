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

fn seams(lines: &[Vec<bool>]) -> impl Iterator<Item = usize> + Clone + '_ {
    lines
        .iter()
        .tuple_windows()
        .positions(|(l1, l2)| l1 == l2)
        .map(|x| x + 1)
}

fn is_mirror(line: usize, pat: &[Vec<bool>]) -> bool {
    let rowcount = line.min(pat.len() - line);

    (0..rowcount).all(|row| pat[line - row - 1] == pat[line + row])
}

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
    let patterns = parse(&INPUT);

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

    let line_scores = patterns.iter().zip(combinations).map(|(orig, many_pats)| {
        let torig = transpose(&orig);
        many_pats
            .map(|pat| (pat.clone(), transpose(&pat)))
            .find_map(|(pat, tpat)| {
                let orig_hline = find_mirror(&orig);
                let potential_hline = seams(&pat).find(|line| {
                    is_mirror(*line, &pat) && !orig_hline.is_some_and(|orig| orig == *line)
                });
                if potential_hline.is_some() && orig_hline != potential_hline {
                    Some(potential_hline.unwrap() * 100)
                } else {
                    let orig_vline = find_mirror(&torig);
                    let potential_vline = seams(&tpat).find(|line| {
                        is_mirror(*line, &tpat) && !orig_vline.is_some_and(|orig| orig == *line)
                    });
                    if potential_vline.is_some() && orig_vline != potential_vline {
                        potential_vline
                    } else {
                        None
                    }
                }
            })
            .unwrap()
    });

    let sum: usize = line_scores.sum();

    println!("{sum}");
    assert_eq!(sum, 31974);

    Ok(())
}

mod part1 {
    use crate::{find_mirror, parse, transpose, INPUT};

    pub(crate) fn solve() -> i64 {
        let patterns = parse(&INPUT);
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
