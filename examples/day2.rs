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

fn find_num_re(re: &str, haystack: &str) -> anyhow::Result<i64> {
    match Regex::new(re)?.captures_iter(haystack).next() {
        Some(x) => Ok(x[1].parse::<i64>()?),
        None => Ok(0),
    }
}

fn part1(mut lines: std::str::Lines<'_>) -> anyhow::Result<i64> {
    let mut sum = 0;
    while let Some(line) = lines.next() {
        let (game_s, rest) = line.split_once(':').unwrap();

        let mut rmax = 0;
        let mut bmax = 0;
        let mut gmax = 0;

        let mut it = rest.split(';');
        while let Some(set) = it.next() {
            let r = find_num_re(r"(\d+) red", set)?;
            if r > rmax {
                rmax = r;
            }
            let b = find_num_re(r"(\d+) blue", set)?;
            if b > bmax {
                bmax = b;
            }
            let g = find_num_re(r"(\d+) green", set)?;
            if g > gmax {
                gmax = g;
            }
        }
        if rmax <= RED && gmax <= GREEN && bmax <= BLUE {
            let game_n = find_num_re(r"Game (\d+)", game_s)?;
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
            let r = find_num_re(r"(\d+) red", set)?;
            if r > rmax {
                rmax = r;
            }
            let b = find_num_re(r"(\d+) blue", set)?;
            if b > bmax {
                bmax = b;
            }
            let g = find_num_re(r"(\d+) green", set)?;
            if g > gmax {
                gmax = g;
            }
        }
        let p = rmax * gmax * bmax;
        sum += p;
    }

    Ok(sum)
}

#[test]
fn day2_part1() {
    assert_eq!(part1(INPUT.lines()).unwrap(), 2776)
}

#[test]
fn day2_part2() {
    assert_eq!(part2(INPUT.lines()).unwrap(), 68638)
}
