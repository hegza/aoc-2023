const INPUT: &str = include_str!("inputs/day6.txt");

fn main() -> anyhow::Result<()> {
    let mut lines = INPUT.lines();

    let times = lines
        .next()
        .unwrap()
        .split_once(':')
        .unwrap()
        .1
        .split_whitespace();
    let distances = lines
        .next()
        .unwrap()
        .split_once(':')
        .unwrap()
        .1
        .split_whitespace();

    println!(
        "Part 1: {}",
        part1::solve(times.clone(), distances.clone())?
    );
    println!("Part 2: {}", part2::solve(times, distances)?);

    Ok(())
}

mod part1 {
    pub(crate) fn solve(
        times: impl Iterator<Item = impl AsRef<str>>,
        distances: impl Iterator<Item = impl AsRef<str>>,
    ) -> anyhow::Result<i64> {
        let times = times.map(|s| s.as_ref().parse::<i64>().unwrap());
        let distances = distances.map(|s| s.as_ref().parse::<i64>().unwrap());

        let s: i64 = times
            .zip(distances)
            .map(|(t, m)| {
                let held_count = (1..t - 1)
                    .filter(|&h| {
                        let dist = h * (t - h + 1) - h;
                        dist > m
                    })
                    .count();
                held_count as i64
            })
            .product();

        Ok(s)
    }
}

mod part2 {
    use crate::part1;
    use std::iter;

    pub(crate) fn solve<'a>(
        times: impl Iterator<Item = &'a str>,
        distances: impl Iterator<Item = &'a str>,
    ) -> anyhow::Result<i64> {
        part1::solve(
            iter::once(times.collect::<String>()),
            iter::once(distances.collect::<String>()),
        )
    }
}
