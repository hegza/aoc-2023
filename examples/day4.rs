use std::collections::HashSet;

use itertools::Itertools;

const INPUT: &str = include_str!("inputs/day4.txt");

fn main() -> anyhow::Result<()> {
    let lines = INPUT.lines();
    let originals = lines
        .map(|line| {
            let (_title, nums) = line.split_once(':').expect(&format!("{:?}", line));

            let (winning, mine) = nums.split_once('|').unwrap();

            let winning = winning
                .split_whitespace()
                .map(|n| n.parse::<i64>().expect(&format!("{:?}", n)))
                .collect::<HashSet<_>>();
            let mine = mine
                .split_whitespace()
                .map(|n| n.parse::<i64>().unwrap())
                .collect::<HashSet<_>>();
            (winning, mine)
        })
        .collect_vec();

    let mut cmemo: Vec<Option<i64>> = vec![None; originals.len()];
    let mut count = |idx: usize, winning: &HashSet<i64>, mine: &HashSet<i64>| -> i64 {
        if let Some(count) = cmemo[idx] {
            count
        } else {
            let count = winning.intersection(&mine).count() as i64;
            cmemo[idx] = Some(count);
            count
        }
    };

    /*
    let mut points = |idx: usize, winning: &HashSet<i64>, mine: &HashSet<i64>| -> i64 {
        if let Some(points) = pmemo[idx] {
            points
        } else {
            let count = winning.intersection(&mine).count();
            let points = match count {
                0 => 0,
                1 => 1,
                n => 2i32.pow((n - 1) as u32) as i64,
            };
            pmemo[idx] = Some(points);
            points
        }
    };*/
    let mut pmemo: Vec<Option<i64>> = vec![None; originals.len()];
    fn points_rec(
        idx: usize,
        cards: &[(HashSet<i64>, HashSet<i64>)],
        pmemo: &mut Vec<Option<i64>>,
    ) -> i64 {
        if let Some(points) = pmemo[idx] {
            println!("Card {} scores you {}", idx + 1, points);
            points
        } else {
            let (winning, mine) = &cards[idx];
            let count = winning.intersection(&mine).count();
            match count {
                0 => {
                    pmemo[idx] = Some(1);
                    println!("Card {} scores you {}", idx + 1, 1);
                    1
                }
                n => {
                    let mut sum = 1;
                    let range = (idx + 1)..(idx + 1 + n as usize);
                    println!(
                        "Card {} has {} matching numbers, so you win {:?}",
                        idx + 1,
                        n,
                        range.clone().map(|x| x + 1).collect_vec()
                    );
                    for card_idx in range {
                        sum += points_rec(card_idx, &cards, pmemo);
                    }
                    pmemo[idx] = Some(sum);
                    println!("Card {} evaluates to {}", idx + 1, sum);
                    sum
                }
            }
        }
    };

    let sum: i64 = (0..originals.len())
        .into_iter()
        .map(|idx| points_rec(idx, &originals, &mut pmemo))
        .sum();

    println!("{:?}", sum);

    Ok(())
}

/*
fn part1() -> anyhow::Result<()> {
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
}
*/
