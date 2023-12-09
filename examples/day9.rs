use itertools::Itertools;

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

    let diff_trees = histories
        .into_iter()
        .map(|hist| {
            let mut diff_tree = vec![hist.clone()];
            loop {
                let diffs = diff_tree
                    .last()
                    .unwrap()
                    .windows(2)
                    .map(|win| win[1] - win[0]);
                if diffs.clone().all(|x| x == 0) {
                    break;
                }
                diff_tree.push(diffs.collect_vec());
            }
            diff_tree
        })
        .collect_vec();

    println!("Part 1: {}", part1::solve(&diff_trees));
    println!("Part 2: {}", part2::solve(&diff_trees));

    Ok(())
}

mod part1 {
    pub(crate) fn solve(diff_trees: &[Vec<Vec<i64>>]) -> i64 {
        let preds = diff_trees.into_iter().map(|diffs| {
            let mut it = diffs.into_iter().rev();
            let mut cur = 0;
            let mut prev = 0;
            while let Some(diff) = it.next() {
                cur = diff.last().unwrap() + prev;
                prev = cur;
            }
            cur
        });

        preds.sum::<i64>()
    }
}

mod part2 {
    pub(crate) fn solve(diff_trees: &[Vec<Vec<i64>>]) -> i64 {
        let preds = diff_trees.into_iter().map(|diffs| {
            let mut it = diffs.into_iter().rev();
            let mut cur = 0;
            let mut prev = 0;
            while let Some(diff) = it.next() {
                cur = diff.first().unwrap() - prev;
                prev = cur;
            }
            cur
        });

        preds.sum::<i64>()
    }
}
