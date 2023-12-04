use std::collections::HashSet;

use regex::Regex;

const INPUT: &str = include_str!("inputs/day4.txt");

fn main() -> anyhow::Result<()> {
    let mut lines = INPUT.lines();

    let title_re = Regex::new(r"Card +(\d)+")?;
    let mut sum = 0;
    while let Some(line) = lines.next() {
        let (title, nums) = line.split_once(':').expect(&format!("{:?}", line));
        let title = &title_re.captures_iter(title).next().unwrap()[0];

        let (winning, mine) = nums.split_once('|').unwrap();

        let winning = winning
            .split_whitespace()
            .map(|n| n.parse::<i64>().expect(&format!("{:?}", n)))
            .collect::<HashSet<_>>();
        let mine = mine
            .split_whitespace()
            .map(|n| n.parse::<i64>().unwrap())
            .collect::<HashSet<_>>();

        let count = winning.intersection(&mine).count();
        let points = match count {
            0 => 0,
            1 => 1,
            n => 2i32.pow((n - 1) as u32),
        };
        println!("{}: {} matches, {} points", title, count, points);
        sum += points;
    }
    println!("{:?}", sum);

    Ok(())
}
