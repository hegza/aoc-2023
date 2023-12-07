use itertools::Itertools;
use std::cmp::{self, Ordering};

const INPUT: &str = include_str!("inputs/day7.txt");

fn main() -> anyhow::Result<()> {
    let p1 = part1::solve()?;
    println!("Part 1: {}", p1);
    assert_eq!(p1, 246912307);

    let p2 = part2::solve()?;
    println!("Part 2: {}", p2);
    assert_eq!(p2, 246894760);

    Ok(())
}

fn card_value(card: &char, joker_rule: bool) -> usize {
    match card {
        c if c.is_numeric() => c.to_digit(10).unwrap() as usize,
        'T' => 10,
        'J' => {
            if joker_rule {
                1
            } else {
                11
            }
        }
        'Q' => 12,
        'K' => 13,
        'A' => 14,
        _ => panic!("not a card"),
    }
}

fn card_cmp(left: &char, right: &char, joker_rule: bool) -> cmp::Ordering {
    card_value(left, joker_rule).cmp(&card_value(right, joker_rule))
}

#[derive(PartialEq, PartialOrd, Debug)]
enum Kind {
    Five = 7,
    Four = 6,
    House = 5,
    Three = 4,
    TwoPair = 3,
    OnePair = 2,
    High = 1,
    None = 0,
}

fn can_be_house(hand: &[char]) -> bool {
    let nonj_counts = hand
        .iter()
        .unique()
        .filter(|&&x| x != 'J')
        .map(|c1| hand.iter().filter(|&c2| c2 == c1).count())
        .collect_vec();
    let pairs = nonj_counts.iter().filter(|&&count| count == 2).count();

    let jcount = hand.iter().filter(|&&c| c == 'J').count();
    match jcount {
        5 | 4 | 3 => true,
        2 => !hand
            .iter()
            .unique()
            .filter(|&&x| x != 'J')
            .all(|c1| hand.iter().filter(|&c2| (c1 == c2)).count() == 1),
        1 => pairs >= 2,
        0 => nonj_counts.contains(&3) && nonj_counts.contains(&2),
        _ => panic!(),
    }
}

fn kind_with_joker_rule(hand: &[char]) -> Kind {
    let counts = hand
        .iter()
        .unique()
        .map(|&c1| hand.iter().filter(|&&c2| (c2 == c1 || c2 == 'J')).count())
        .collect_vec();
    let of_kind = *counts.iter().max().unwrap();
    let pairs = counts.iter().filter(|&&count| count == 2).count();

    let jcount = hand.iter().filter(|&&c| c == 'J').count();

    if of_kind == 5 {
        Kind::Five
    } else if of_kind == 4 {
        Kind::Four
    } else if can_be_house(hand) {
        Kind::House
    } else if of_kind == 3 {
        Kind::Three
    } else if pairs == 2 || jcount >= 2 {
        Kind::TwoPair
    } else if jcount != 0 {
        Kind::OnePair
    } else if hand
        .iter()
        .unique()
        .filter(|&&x| x != 'J')
        .all(|c1| hand.iter().filter(|&c2| (c1 == c2)).count() == 1)
    {
        Kind::High
    } else {
        Kind::None
    }
}

fn kind_without_joker_rule(hand: &[char]) -> Kind {
    let counts = hand
        .iter()
        .unique()
        .map(|c1| hand.iter().filter(|&c2| c2 == c1).count())
        .collect_vec();
    let of_kind = *counts.iter().max().unwrap();
    let pairs = counts.iter().filter(|&&count| count == 2).count();

    if of_kind == 5 {
        Kind::Five
    } else if of_kind == 4 {
        Kind::Four
    } else if counts.contains(&2) && counts.contains(&3) {
        Kind::House
    } else if of_kind == 3 {
        Kind::Three
    } else if pairs == 2 {
        Kind::TwoPair
    } else if pairs == 1 {
        Kind::OnePair
    } else if counts
        .iter()
        .all(|c1| counts.iter().filter(|&c2| c1 == c2).count() == 1)
    {
        Kind::High
    } else {
        Kind::None
    }
}

fn value(hand: &[char], joker_rule: bool) -> usize {
    if joker_rule {
        (kind_with_joker_rule(hand) as usize).max(kind_without_joker_rule(hand) as usize)
    } else {
        kind_without_joker_rule(hand) as usize
    }
}

fn hand_cmp(left: &[char], right: &[char], joker_rule: bool) -> cmp::Ordering {
    match value(left, joker_rule).cmp(&value(right, joker_rule)) {
        ord @ (Ordering::Less | Ordering::Greater) => ord,
        Ordering::Equal => {
            for (c1, c2) in left.iter().zip(right.iter()) {
                match card_cmp(c1, c2, joker_rule) {
                    Ordering::Equal => continue,
                    ord => {
                        return ord;
                    }
                }
            }
            Ordering::Equal
        }
    }
}

mod part1 {
    use crate::{hand_cmp, INPUT};
    use itertools::Itertools;

    pub(crate) fn solve() -> anyhow::Result<i64> {
        let lines = INPUT.lines();
        let mut hands = lines
            .into_iter()
            .map(|line| {
                let (hand, bid) = line.split_once(' ').unwrap();

                let hand = hand.chars().collect_vec();
                let bid = bid.parse::<usize>().unwrap();

                (hand, bid)
            })
            .collect_vec();

        hands.sort_by(|(hand1, _), (hand2, _)| hand_cmp(hand1, hand2, false));

        let hands = hands
            .into_iter()
            .enumerate()
            .map(|(idx, rest)| (idx + 1, rest));

        let winnings: usize = hands.map(|(rank, (_, bid))| bid * rank).sum::<usize>();

        Ok(winnings as i64)
    }
}

mod part2 {
    use crate::{hand_cmp, INPUT};
    use itertools::Itertools;

    pub(crate) fn solve() -> anyhow::Result<i64> {
        let lines = INPUT.lines();

        let mut hands = lines
            .into_iter()
            .map(|line| {
                let (hand, bid) = line.split_once(' ').unwrap();

                let hand = hand.chars().collect_vec();
                let bid = bid.parse::<usize>().unwrap();

                (hand, bid)
            })
            .collect_vec();

        hands.sort_by(|(hand1, _), (hand2, _)| hand_cmp(hand1, hand2, true));

        let hands = hands
            .into_iter()
            .enumerate()
            .map(|(idx, rest)| (idx + 1, rest));

        let winnings: usize = hands.map(|(rank, (_, bid))| bid * rank).sum::<usize>();

        Ok(winnings as i64)
    }
}
