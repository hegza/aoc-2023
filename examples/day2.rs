use regex::Regex;

const INPUT: &str = include_str!("inputs/day2.txt");

const RED: i64 = 12;
const GREEN: i64 = 13;
const BLUE: i64 = 14;

fn main() -> anyhow::Result<()> {
    println!("Part 1: {}", part1(INPUT.lines())?);
    println!("Part 2: {}", part2(INPUT.lines())?);

    Ok(())
}

fn part1(mut lines: std::str::Lines<'_>) -> anyhow::Result<i64> {
    let mut sum = 0;
    while let Some(line) = lines.next() {
        let (game_s, rest) = line.split_once(':').unwrap();
        let game_n = &Regex::new(r"Game (\d+)")?
            .captures_iter(game_s)
            .next()
            .unwrap()[1]
            .parse::<i64>()?;

        let mut rmax = 0;
        let mut bmax = 0;
        let mut gmax = 0;

        let mut it = rest.split(';');
        while let Some(set) = it.next() {
            let r = match Regex::new(r"(\d+) red")?.captures_iter(set).next() {
                Some(x) => x[1].parse::<i64>()?,
                None => 0,
            };
            if r > rmax {
                rmax = r;
            }
            let b = match Regex::new(r"(\d+) blue")?.captures_iter(set).next() {
                Some(x) => x[1].parse::<i64>()?,
                None => 0,
            };
            if b > bmax {
                bmax = b;
            }
            let g = match Regex::new(r"(\d+) green")?.captures_iter(set).next() {
                Some(x) => x[1].parse::<i64>()?,
                None => 0,
            };
            if g > gmax {
                gmax = g;
            }
        }
        if rmax <= RED && gmax <= GREEN && bmax <= BLUE {
            sum += game_n;
        }
    }

    Ok(sum)
}

fn part2(mut lines: std::str::Lines<'_>) -> anyhow::Result<i64> {
    let mut sum = 0;
    while let Some(line) = lines.next() {
        let (_, rest) = line.split_once(':').unwrap();

        let mut rmax = 0;
        let mut bmax = 0;
        let mut gmax = 0;

        let mut it = rest.split(';');
        while let Some(set) = it.next() {
            let r = match Regex::new(r"(\d+) red")?.captures_iter(set).next() {
                Some(x) => x[1].parse::<i64>()?,
                None => 0,
            };
            if r > rmax {
                rmax = r;
            }
            let b = match Regex::new(r"(\d+) blue")?.captures_iter(set).next() {
                Some(x) => x[1].parse::<i64>()?,
                None => 0,
            };
            if b > bmax {
                bmax = b;
            }
            let g = match Regex::new(r"(\d+) green")?.captures_iter(set).next() {
                Some(x) => x[1].parse::<i64>()?,
                None => 0,
            };
            if g > gmax {
                gmax = g;
            }
        }
        let p = rmax * gmax * bmax;
        sum += p;
    }

    Ok(sum)
}
