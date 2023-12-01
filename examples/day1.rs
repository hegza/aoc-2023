const INPUT: &str = include_str!("inputs/day1.txt");

fn search(line: &str, first: bool) -> char {
    if first {
        for n in 0..line.len() {
            if let Some(n) = n_here(&line[n..]) {
                return n;
            }
        }
    } else {
        for n in 0..line.len() {
            if let Some(n) = n_here(&line[line.len() - n - 1..]) {
                return n;
            }
        }
    }
    panic!()
}

fn n_here(line: &str) -> Option<char> {
    let fc = line.chars().nth(0).unwrap();
    if fc.is_digit(10) {
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

fn main() -> anyhow::Result<()> {
    let mut lines = INPUT.lines();
    let mut sum = 0;
    while let Some(line) = lines.next() {
        println!("{line}");
        let mut c = line.chars();
        let first = search(line, true);
        let last = search(line, false);

        let word = String::from(first) + &String::from(last);
        let n = word.parse::<i32>().unwrap();
        sum += n;
    }
    println!("{sum}");

    Ok(())
}
