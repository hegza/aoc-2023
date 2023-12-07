use itertools::Itertools;
use std::{cmp::Ordering, collections::*, str::FromStr};

const INPUT: &str = include_str!("inputs/day7.txt");

#[derive(PartialEq, Eq, Hash, Debug)]
struct Card(char);

impl Card {
    fn value(&self) -> usize {
        match self.0 {
            c if c.is_numeric() => c.to_digit(10).unwrap() as usize,
            'T' => 10,
            'J' => 11,
            'Q' => 12,
            'K' => 13,
            'A' => 14,
            _ => panic!("not a card"),
        }
    }
}

impl PartialOrd for Card {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let v = self.value();
        let v2 = other.value();
        v.partial_cmp(&v2)
    }
}

#[derive(PartialEq, Debug)]
struct Hand(Vec<Card>);

impl FromStr for Hand {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(s.chars().map(|c| Card(c)).collect_vec()))
    }
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

impl Hand {
    fn kind(&self) -> Kind {
        let cs = &self.0;

        let counts = cs
            .iter()
            .unique()
            .map(|c1| cs.iter().filter(|&c2| c2 == c1).count())
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

    fn value(&self) -> usize {
        self.kind() as usize
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match self.value().partial_cmp(&other.value()).unwrap() {
            ord @ (Ordering::Less | Ordering::Greater) => Some(ord),
            Ordering::Equal => {
                for (c1, c2) in self.0.iter().zip(other.0.iter()) {
                    match c1.partial_cmp(c2).unwrap() {
                        Ordering::Equal => continue,
                        ord => {
                            return Some(ord);
                        }
                    }
                }
                return None;
            }
        }
    }
}

fn main() -> anyhow::Result<()> {
    let lines = INPUT.lines();

    let mut hands = lines
        .into_iter()
        .map(|line| {
            let (hand, bid) = line.split_once(' ').unwrap();

            let hand = Hand::from_str(hand).unwrap();
            let bid = bid.parse::<usize>().unwrap();

            (hand, bid)
        })
        .collect_vec();

    hands.sort_by(|(hand1, _), (hand2, _)| {
        hand1.partial_cmp(&hand2).expect("cannot order elements")
    });

    /*
    for (rank, hand) in hands
        .iter()
        .map(|(h, bid)| (h.0.iter().map(|c| c.0).collect_vec(), h.kind(), bid))
        .enumerate()
    {
        println!("{}: {:?}", rank + 1, &hand);
    }*/

    let winnings: usize = hands
        .into_iter()
        .enumerate()
        .map(|(idx, (_hand, bid))| {
            let rank = idx + 1;
            bid * rank
        })
        .sum::<usize>();

    println!("{winnings}");
    Ok(())
}

mod part1 {
    use crate::{Hand, INPUT};
    use itertools::Itertools;
    use std::str::FromStr;

    fn solve() -> anyhow::Result<()> {
        let lines = INPUT.lines();

        let mut hands = lines
            .into_iter()
            .map(|line| {
                let (hand, bid) = line.split_once(' ').unwrap();

                let hand = Hand::from_str(hand).unwrap();
                let bid = bid.parse::<usize>().unwrap();

                (hand, bid)
            })
            .collect_vec();

        hands.sort_by(|(hand1, _), (hand2, _)| {
            hand1.partial_cmp(&hand2).expect("cannot order elements")
        });

        /*
        for (rank, hand) in hands
            .iter()
            .map(|(h, bid)| (h.0.iter().map(|c| c.0).collect_vec(), h.kind(), bid))
            .enumerate()
        {
            println!("{}: {:?}", rank + 1, &hand);
        }*/

        let winnings: usize = hands
            .into_iter()
            .enumerate()
            .map(|(idx, (_hand, bid))| {
                let rank = idx + 1;
                bid * rank
            })
            .sum::<usize>();

        println!("{winnings}");
        Ok(())
    }
}
