use regex::Regex;

const INPUT: &str = include_str!("inputs/day15.txt");
const _TEST_INPUT: &str = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";

fn hash(s: &str) -> u8 {
    let mut cur: u8 = 0;
    for c in s.chars() {
        let ascii = c as u8;
        cur = cur.wrapping_add(ascii);
        cur = cur.wrapping_mul(17);
        // cur %= 256;
    }
    cur as u8
}

fn to_instr<'s>(s: &'s str, re: &Regex) -> (&'s str, usize, char, Option<u8>) {
    let mut m = re.captures_iter(s);
    let m1 = m.next().unwrap();

    let label = m1.get(1).unwrap().as_str();
    let hs = hash(label);
    let op = m1.get(2).unwrap().as_str().chars().nth(0).unwrap();
    let num_opt = m1.get(3).map(|n| n.as_str().parse::<u8>().unwrap());

    (label, hs as usize, op, num_opt)
}

fn _to_instr2<'s>(s: &'s str) -> (&'s str, usize, char, Option<u8>) {
    let split_idx = s.find(&['=', '-']).unwrap();
    let label = &s[0..split_idx];
    let hs = hash(label);
    let op = s.chars().nth(split_idx).unwrap();
    let num_opt = (s.len() - split_idx - 1 != 0)
        .then_some(s[split_idx + 1..s.len()].to_owned().parse::<u8>().unwrap());

    (label, hs as usize, op, num_opt)
}

fn main() -> anyhow::Result<()> {
    let p1 = part1::solve();
    println!("Part 1: {p1}");
    let p2 = part2::solve()?;
    println!("Part 2: {p2}");

    Ok(())
}

mod part1 {
    use crate::{hash, INPUT};

    pub(crate) fn solve() -> i64 {
        let mut lines = INPUT.lines();
        let line = lines.next().unwrap();

        let hashes = line.split(',').map(|s| hash(s) as u32);

        let sum: i64 = hashes.sum::<u32>() as i64;
        sum
    }
}

mod part2 {
    use crate::{to_instr, INPUT};
    use itertools::Itertools;
    use regex::Regex;

    pub(crate) fn solve() -> anyhow::Result<i64> {
        let mut lines = INPUT.lines();
        let line = lines.next().unwrap();

        let re = Regex::new(r"([a-z]+)([=|-])([0-9])?")?;
        let instrs = line.split(',').map(|s| to_instr(s, &re));

        let mut boxes: Vec<Vec<(&str, u8)>> = vec![vec![]; 256];
        for (label, hash, op, n) in instrs {
            if op == '-' {
                let lenses = &mut boxes[hash];
                if let Some(idx) = lenses
                    .iter()
                    .position(|(lens_label, _)| lens_label == &label)
                {
                    lenses.remove(idx);
                }
            } else if op == '=' {
                let focal = n.unwrap();
                let lenses = &mut boxes[hash];
                if let Some(idx) = lenses
                    .iter()
                    .position(|(lens_label, _)| lens_label == &label)
                {
                    lenses[idx] = (label, focal);
                } else {
                    lenses.push((label, focal));
                }
            }

            print!("After: {label}{op}");
            if let Some(n) = n {
                println!("{n}");
            } else {
                println!();
            }
            for (idx, box_) in boxes.iter().enumerate() {
                if !box_.is_empty() {
                    print!("Box {idx}: ");
                    println!(
                        "[{}]",
                        box_.iter().map(|(s, u)| format!("{s} {u}")).join(" ")
                    );
                }
            }
            println!();
        }

        let power = boxes
            .iter()
            .enumerate()
            .map(|(box_num, box_)| {
                box_.iter()
                    .enumerate()
                    .map(|(slot_idx, (_, lens))| (slot_idx + 1, lens))
                    .map(|(slot_num, lens)| (1 + box_num) * slot_num * *lens as usize)
                    .sum::<usize>()
            })
            .sum::<usize>();

        Ok(power as i64)
    }
}
