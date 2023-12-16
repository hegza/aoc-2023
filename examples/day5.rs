use itertools::Itertools;
use std::{num::ParseIntError, str};

const INPUT: &str = include_str!("inputs/day5.txt");

fn main() -> anyhow::Result<()> {
    let mut lines = INPUT.lines();
    let seed_nums = lines
        .next()
        .unwrap()
        .split_once(':')
        .unwrap()
        .1
        .split_whitespace()
        .map(|s| s.parse::<i64>().unwrap());

    println!(
        "Part 1: {}",
        part1::solve(seed_nums.clone(), lines.clone())?
    );
    println!("Part 2: {}", part2::solve(seed_nums, lines)?);
    Ok(())
}

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
}

fn parse_maps(lines: str::Lines) -> Result<Vec<Map>, ParseIntError> {
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
    Ok(maps)
}

mod part1 {
    use std::str;

    use crate::parse_maps;

    pub(crate) fn solve(
        seed_nums: impl Iterator<Item = i64>,
        lines: str::Lines,
    ) -> anyhow::Result<i64> {
        let maps = parse_maps(lines)?;
        let locations = seed_nums.map(|seed| maps.iter().fold(seed, |val, map| map.map(val)));
        let min = locations.min().unwrap();

        Ok(min)
    }
}

mod part2 {
    use crate::parse_maps;
    use itertools::Itertools;
    use std::str;

    pub(crate) fn solve(
        seed_nums: impl Iterator<Item = i64>,
        lines: str::Lines,
    ) -> anyhow::Result<i64> {
        let ranges = seed_nums
            .chunks(2)
            .into_iter()
            .map(|mut chunk| (chunk.next().unwrap(), chunk.next().unwrap()))
            .collect_vec();

        let seeds = ranges
            .into_iter()
            .flat_map(|(start, len)| (start..start + len));

        let maps = parse_maps(lines)?;
        let locations = seeds.map(|seed| maps.iter().fold(seed, |val, map| map.map(val)));
        let min = locations.min().unwrap();

        Ok(min)
    }
}
