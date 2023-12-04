use itertools::Itertools;
use std::collections::HashSet;

const INPUT: &str = include_str!("inputs/day4.txt");

fn main() -> anyhow::Result<()> {
    let lines = INPUT.lines();
    let cards = lines
        .map(|line| {
            let nums = line.split_once(':').unwrap().1;
            let (winning, mine) = nums.split_once('|').unwrap();

            let winning = winning
                .split_whitespace()
                .map(|n| n.parse::<i64>().unwrap())
                .collect::<HashSet<_>>();
            let mine = mine
                .split_whitespace()
                .map(|n| n.parse::<i64>().unwrap())
                .collect::<HashSet<_>>();
            (winning, mine)
        })
        .collect_vec();

    println!("{}", part1::solve(&cards)?);
    println!("{}", part2::solve(&cards)?);
    Ok(())
}

mod part1 {
    use std::collections::HashSet;

    pub(crate) fn solve(cards: &[(HashSet<i64>, HashSet<i64>)]) -> anyhow::Result<i64> {
        let sum = cards
            .iter()
            .map(|(winning, mine)| {
                let count = winning.intersection(&mine).count() as i64;
                let points = match count {
                    n @ (0 | 1) => n,
                    n => 2i64.pow((n - 1) as u32),
                };
                points
            })
            .sum();
        Ok(sum)
    }
}

mod part2 {
    use std::collections::HashSet;

    pub(crate) fn solve(cards: &[(HashSet<i64>, HashSet<i64>)]) -> anyhow::Result<i64> {
        let mut pmemo: Vec<Option<i64>> = vec![None; cards.len()];
        let sum: i64 = (0..cards.len())
            .into_iter()
            .map(|idx| full_count(idx, &cards, &mut pmemo))
            .sum();

        Ok(sum)
    }

    fn full_count(
        idx: usize,
        cards: &[(HashSet<i64>, HashSet<i64>)],
        pmemo: &mut Vec<Option<i64>>,
    ) -> i64 {
        if let Some(points) = pmemo[idx] {
            points
        } else {
            let (winning, mine) = &cards[idx];
            let full_count = match winning.intersection(&mine).count() {
                0 => 1,
                n => {
                    1 + ((idx + 1)..(idx + 1 + n as usize))
                        .into_iter()
                        .map(|idx| full_count(idx, &cards, pmemo))
                        .sum::<i64>()
                }
            };
            pmemo[idx] = Some(full_count);
            full_count
        }
    }
}
