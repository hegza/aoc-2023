use itertools::Itertools;
use regex::Regex;
use std::collections::*;

const INPUT: &str = include_str!("inputs/day8.txt");

fn step(cur: impl AsRef<str>, i: char, nodes: &HashMap<String, (String, String)>) -> &String {
    if i == 'L' {
        &nodes[cur.as_ref()].0
    } else {
        &nodes[cur.as_ref()].1
    }
}

fn count_until_cond(
    start: &str,
    instr: &[char],
    nodes: &HashMap<String, (String, String)>,
    cond: impl Fn(&str) -> bool,
) -> i64 {
    let mut n = 0;
    let mut cur = start;
    let instr = instr.iter().cycle();

    for &i in instr {
        if cond(cur) {
            break;
        }
        cur = step(cur, i, nodes);
        n += 1;
    }

    n
}

fn main() -> anyhow::Result<()> {
    let mut lines = INPUT.lines();
    let instr = lines.next().unwrap().chars().collect_vec();

    // Skip whitespace
    lines.next().unwrap();

    let alpha_re = Regex::new(r"[A-Z]+")?;
    let nodes = lines
        .map(|s| {
            let (left, right) = s.split_once('=').unwrap();

            let src = left.trim().to_owned();

            let mut dests = alpha_re.captures_iter(right);
            let dest1 = dests.next().unwrap()[0].to_owned();
            let dest2 = dests.next().unwrap()[0].to_owned();

            (src, (dest1, dest2))
        })
        .collect();

    println!("Part 1: {}", part1::solve(&instr, &nodes));
    println!("Part 2: {}", part2::solve(&instr, &nodes));

    Ok(())
}

mod part1 {
    use crate::count_until_cond;
    use std::collections::*;

    pub(crate) fn solve(instr: &[char], nodes: &HashMap<String, (String, String)>) -> i64 {
        count_until_cond("AAA", instr, nodes, |s| s == "ZZZ")
    }
}

mod part2 {
    use crate::count_until_cond;
    use itertools::Itertools;
    use lcmx::lcmx;
    use std::collections::HashMap;

    pub(crate) fn solve(instr: &[char], nodes: &HashMap<String, (String, String)>) -> i64 {
        let starts = nodes.keys().filter(|s| s.ends_with('A'));

        let cycles = starts
            .map(|start| count_until_cond(start, instr, nodes, |s| s.ends_with('Z')) as u64)
            .collect_vec();

        let lcm = lcmx(&cycles).unwrap();

        lcm as i64
    }
}
