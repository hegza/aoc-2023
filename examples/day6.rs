use itertools::Itertools;
use std::iter;

const INPUT: &str = include_str!("inputs/day6.txt");

fn main() -> anyhow::Result<()> {
    let mut lines = INPUT.lines();

    let times = lines
        .next()
        .unwrap()
        .split_once(':')
        .unwrap()
        .1
        .split_whitespace()
        .collect_vec();
    let time = times.concat().parse::<i64>()?;
    let distances = lines
        .next()
        .unwrap()
        .split_once(':')
        .unwrap()
        .1
        .split_whitespace()
        .collect_vec();
    let distance = distances.concat().parse::<i64>()?;

    println!("Times: {:?}\n\nDistances: {:?}", time, distance);

    let s: usize = iter::once(time)
        .zip(iter::once(distance))
        .map(|(t, m)| {
            let held_count = (1..t - 1)
                .filter(|&h| {
                    let dist = h * (t - h + 1) - h;
                    dist > m
                })
                .count();
            held_count
        })
        .product();

    println!("{s}");

    Ok(())
}

/*
mod part1 {
    use crate::INPUT;

    fn solve() -> anyhow::Result<()> {
        let mut lines = INPUT.lines();

        let times = lines
            .next()
            .unwrap()
            .split_once(':')
            .unwrap()
            .1
            .split_whitespace()
            .map(|s| s.parse::<i64>().unwrap())
            .collect_vec();
        let distances = lines
            .next()
            .unwrap()
            .split_once(':')
            .unwrap()
            .1
            .split_whitespace()
            .map(|s| s.parse::<i64>().unwrap())
            .collect_vec();

        println!("Times: {:?}\n\nDistances: {:?}", times, distances);

        let s: usize = times
            .into_iter()
            .zip(distances)
            .map(|(t, m)| {
                let held_count = (1..t - 1)
                    .filter(|&h| {
                        let dist = h * (t - h + 1) - h;
                        dist > m
                    })
                    .count();
                held_count
            })
            .product();

        println!("{s}");

        Ok(())
    }
}
*/
