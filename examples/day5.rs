use itertools::Itertools;
use std::{collections::*, ops};

const INPUT: &str = include_str!("inputs/day5.txt");

struct Jump {
    dest: i64,
    src: i64,
    len: i64,
}

impl From<(i64, i64, i64)> for Jump {
    fn from(val: (i64, i64, i64)) -> Self {
        Self {
            dest: val.0,
            src: val.1,
            len: val.2,
        }
    }
}

struct Map(Vec<Jump>);
impl From<Vec<(i64, i64, i64)>> for Map {
    fn from(value: Vec<(i64, i64, i64)>) -> Self {
        Self(
            value
                .into_iter()
                .map(|(dest, src, len)| Jump::from((dest, src, len)))
                .collect_vec(),
        )
    }
}

impl Map {
    fn map(&self, val: i64) -> i64 {
        for jump in &self.0 {
            if (jump.src..(jump.src + jump.len)).contains(&val) {
                return val + jump.dest - jump.src;
            }
        }

        val
    }

    /*
    fn multimap(&self, range: ops::Range<i64>) -> Vec<ops::Range<i64>> {
        let mut ranges
    }
    */
}

fn main() -> anyhow::Result<()> {
    let mut lines = INPUT.lines();

    let ranges = lines
        .next()
        .unwrap()
        .split_once(':')
        .unwrap()
        .1
        .split_whitespace()
        .map(|s| s.parse::<i64>().unwrap())
        .chunks(2)
        .into_iter()
        .map(|mut chunk| (chunk.next().unwrap(), chunk.next().unwrap()))
        .collect_vec();

    let seeds: Vec<i64> = ranges
        .into_iter()
        .flat_map(|(start, len)| {
            let range = start..start + len;
            range.into_iter()
        })
        .collect_vec();

    let mut maps = vec![];
    let mut cur_map = None;
    for line in lines {
        if line.is_empty() {
            if let Some(map) = cur_map.take() {
                maps.push(Map::from(map));
            }
            continue;
        }
        if line.contains(':') {
            cur_map = Some(vec![]);
            continue;
        }
        if let Some(map) = cur_map.as_mut() {
            let mut elems = line.split_whitespace();
            map.push((
                elems.next().unwrap().parse::<i64>()?,
                elems.next().unwrap().parse::<i64>()?,
                elems.next().unwrap().parse::<i64>()?,
            ))
        }
    }
    if let Some(map) = cur_map.take() {
        maps.push(Map::from(map));
    }

    let locations = seeds
        .into_iter()
        .map(|seed| maps.iter().fold(seed, |val, map| map.map(val)));
    println!("{}", locations.min().unwrap());

    Ok(())
}

mod part1 {
    use itertools::Itertools;
    use std::collections::*;

    const INPUT: &str = include_str!("inputs/day5.txt");

    struct Jump {
        dest: i64,
        src: i64,
        len: i64,
    }

    impl From<(i64, i64, i64)> for Jump {
        fn from(val: (i64, i64, i64)) -> Self {
            Self {
                dest: val.0,
                src: val.1,
                len: val.2,
            }
        }
    }

    struct Map(Vec<Jump>);
    impl From<Vec<(i64, i64, i64)>> for Map {
        fn from(value: Vec<(i64, i64, i64)>) -> Self {
            Self(
                value
                    .into_iter()
                    .map(|(dest, src, len)| Jump::from((dest, src, len)))
                    .collect_vec(),
            )
        }
    }

    impl Map {
        fn map(&self, val: i64) -> i64 {
            for jump in &self.0 {
                if (jump.src..(jump.src + jump.len - 1)).contains(&val) {
                    return val + jump.dest - jump.src;
                }
            }

            val
        }
    }

    fn solve() -> anyhow::Result<()> {
        let mut lines = INPUT.lines();

        let seeds = lines
            .next()
            .unwrap()
            .split_once(':')
            .unwrap()
            .1
            .split_whitespace()
            .map(|s| s.parse::<i64>().unwrap())
            .collect_vec();

        let mut maps = vec![];
        let mut cur_map = None;
        for line in lines {
            if line.is_empty() {
                if let Some(map) = cur_map.take() {
                    maps.push(Map::from(map));
                }
                continue;
            }
            if line.contains(':') {
                cur_map = Some(vec![]);
                continue;
            }
            if let Some(map) = cur_map.as_mut() {
                let mut elems = line.split_whitespace();
                map.push((
                    elems.next().unwrap().parse::<i64>()?,
                    elems.next().unwrap().parse::<i64>()?,
                    elems.next().unwrap().parse::<i64>()?,
                ))
            }
        }
        if let Some(map) = cur_map.take() {
            maps.push(Map::from(map));
        }

        let mapped = seeds
            .into_iter()
            .map(|seed| maps.iter().fold(seed, |val, map| map.map(val)));
        println!("{}", mapped.min().unwrap());

        Ok(())
    }
}
