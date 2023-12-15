use itertools::Itertools;
use std::collections::*;

const INPUT: &str = include_str!("inputs/day15.txt");

fn main() -> anyhow::Result<()> {
    let mut lines = INPUT.lines();
    let mut line = lines.next().unwrap();

    let hashes = line.split(',').map(|s| {
        let mut cur = 0 as u32;
        for c in s.chars() {
            let ascii = c as u8;
            cur += ascii as u32;
            cur *= 17;
            cur %= 256;
        }
        cur
    });

    let sum: i64 = hashes.sum::<u32>() as i64;
    println!("{sum}");

    Ok(())
}
