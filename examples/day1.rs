use std::str::Lines;

const INPUT: &str = include_str!("inputs/day1.txt");

fn main() -> anyhow::Result<()> {
    let p1 = part1(INPUT.lines());
    let p2 = part2(INPUT.lines());

    println!("Part 1: {p1}");
    println!("Part 2: {p2}");

    Ok(())
}

fn part1(mut lines: Lines<'_>) -> i64 {
    let mut sum = 0;

    while let Some(line) = lines.next() {
        let first = line.chars().find(is_digit).unwrap();
        let last = line.chars().rev().find(is_digit).unwrap();

        let word = String::from(first) + &String::from(last);
        let n = word.parse::<i64>().unwrap();
        sum += n;
    }

    sum
}

fn part2(mut lines: Lines<'_>) -> i64 {
    let mut sum = 0;

    while let Some(line) = lines.next() {
        let first = search(line, true);
        let last = search(line, false);

        let word = String::from(first) + &String::from(last);
        let n = word.parse::<i64>().unwrap();
        sum += n;
    }

    sum
}

fn search(line: &str, first: bool) -> char {
    for n in 0..line.len() {
        let range = if first { n.. } else { line.len() - n - 1.. };
        if let Some(n) = n_here(&line[range]) {
            return n;
        }
    }
    // Unreachable with the known input
    panic!()
}

fn n_here(line: &str) -> Option<char> {
    let fc = line.chars().nth(0).unwrap();
    if is_digit(&fc) {
        return Some(fc);
    };
    let mut acc = String::new();
    for ch in line.chars() {
        acc.push(ch);
        match acc.as_str() {
            "one" => return Some('1'),
            "two" => return Some('2'),
            "three" => return Some('3'),
            "four" => return Some('4'),
            "five" => return Some('5'),
            "six" => return Some('6'),
            "seven" => return Some('7'),
            "eight" => return Some('8'),
            "nine" => return Some('9'),
            _ => continue,
        }
    }
    None
}

fn is_digit(c: &char) -> bool {
    c.is_digit(10)
}
