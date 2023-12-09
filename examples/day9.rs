use itertools::Itertools;
use std::collections::*;

const INPUT: &str = include_str!("inputs/day9.txt");

fn main() -> anyhow::Result<()> {
    let lines = INPUT.lines();

    let histories = lines
        .map(|line| {
            line.split_whitespace()
                .map(|n| n.parse::<i64>())
                .collect::<Result<Vec<_>, _>>()
        })
        .collect::<Result<Vec<_>, _>>()?;

    let predictions = histories.into_iter().map(|hist| {
        let mut cur_diff = &hist;
        let mut diffs = vec![hist.clone()];
        loop {
            let diff = cur_diff.windows(2).map(|win| win[1] - win[0]);
            if diff.clone().all(|x| x == 0) {
                break;
            }
            let diff = diff.collect_vec();
            diffs.push(diff);
            cur_diff = &diffs.last().unwrap();
        }
        println!("diffs for {:?}", &hist);
        for d in &diffs {
            println!("\t{:?}:", &d);
        }
        // Prediction
        let mut it = diffs.into_iter().rev();
        let mut cur = 0;
        let mut prev = 0;
        while let Some(diff) = it.next() {
            cur = diff.first().unwrap() - prev;
            prev = cur;
            println!("\t{}", cur);
        }
        cur
    });

    println!("{}", predictions.sum::<i64>());

    Ok(())
}

mod part1 {
    use itertools::Itertools;

    const INPUT: &str = include_str!("inputs/day9.txt");

    fn solve() -> anyhow::Result<()> {
        let lines = INPUT.lines();

        let histories = lines
            .map(|line| {
                line.split_whitespace()
                    .map(|n| n.parse::<i64>())
                    .collect::<Result<Vec<_>, _>>()
            })
            .collect::<Result<Vec<_>, _>>()?;

        let predictions = histories.into_iter().map(|hist| {
            let mut cur_diff = &hist;
            let mut diffs = vec![];
            loop {
                let diff = cur_diff.windows(2).map(|win| win[1] - win[0]);
                if diff.clone().all(|x| x == 0) {
                    break;
                }
                let diff = diff.collect_vec();
                diffs.push(diff);
                cur_diff = &diffs.last().unwrap();
            }
            diffs.push(hist);
            for d in &diffs {
                println!("\t{:?}:", &d);
            }
            // Prediction
            let mut it = diffs.into_iter().rev();
            let mut cur = 0;
            let mut prev = 0;
            while let Some(diff) = it.next() {
                cur = diff.last().unwrap() + prev;
                prev = cur;
                println!("\t{}", cur);
            }
            cur
        });

        println!("{}", predictions.sum::<i64>());

        Ok(())
    }
}
