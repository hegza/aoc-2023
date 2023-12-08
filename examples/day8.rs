use itertools::Itertools;
use lcmx::lcmx;
use regex::Regex;
use std::collections::*;

const INPUT: &str = include_str!("inputs/day8.txt");

fn main() -> anyhow::Result<()> {
    let mut lines = INPUT.lines();
    let instr = lines.next().unwrap().chars();

    // Skip whitespace
    lines.next().unwrap();

    let nodes = lines
        .map(|s| {
            let (left, right) = s.split_once('=').unwrap();
            let src = left.trim().to_owned();
            let re = Regex::new(r"[A-Z]+").unwrap();
            let mut dests = re.captures_iter(right);
            let dest1 = dests.next().unwrap()[0].to_owned();
            let dest2 = dests.next().unwrap()[0].to_owned();
            (src, (dest1, dest2))
        })
        .collect::<HashMap<String, (String, String)>>();

    let starts = nodes
        .keys()
        .map(|s| s.to_string())
        .filter(|s| s.ends_with('A'));

    let cycles = starts
        .map(|start| {
            let mut cur = &start;
            let mut n: u64 = 0;
            let mut instr = instr.clone().cycle();

            while let Some(i) = instr.next() {
                if cur.ends_with('Z') {
                    break;
                }
                if i == 'L' {
                    cur = &nodes[cur].0;
                } else {
                    cur = &nodes[cur].1;
                }
                n += 1;
            }
            n
        })
        .collect_vec();
    let lcm = lcmx(&cycles).unwrap();
    println!("{}", lcm);

    Ok(())
}

mod part1 {
    use regex::Regex;
    use std::collections::*;

    const INPUT: &str = include_str!("inputs/day8.txt");

    pub(crate) fn solve() -> anyhow::Result<()> {
        let mut lines = INPUT.lines();

        let mut instr = lines.next().unwrap().chars();

        // Skip whitespace
        lines.next().unwrap();

        let nodes = lines
            .map(|s| {
                let (left, right) = s.split_once('=').unwrap();
                let src = left.trim().to_owned();
                let re = Regex::new(r"[A-Z]+").unwrap();
                let mut dests = re.captures_iter(right);
                let dest1 = dests.next().unwrap()[0].to_owned();
                let dest2 = dests.next().unwrap()[0].to_owned();
                (src, (dest1, dest2))
            })
            .collect::<HashMap<String, (String, String)>>();

        for node in &nodes {
            println!("{:?}", &node);
        }

        let mut n = 0;
        let mut cur = "AAA";
        let mut instr = instr.cycle();
        while let Some(i) = instr.next() {
            if cur == "ZZZ" {
                break;
            }
            if i == 'L' {
                cur = &nodes[cur].0;
            } else {
                cur = &nodes[cur].1;
            }
            n += 1;
        }
        println!("{n}");

        Ok(())
    }
}
