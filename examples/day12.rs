use itertools::Itertools;
use std::{fmt, iter};

const _TEST_INPUT: &str = include_str!("inputs/day12_test.txt");
const INPUT: &str = include_str!("inputs/day12.txt");

#[derive(PartialEq, Clone)]
enum Status {
    Unknown,
    Active,
    Inactive,
}

impl fmt::Debug for Status {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Unknown => write!(f, "?"),
            Self::Active => write!(f, "#"),
            Self::Inactive => write!(f, "."),
        }
    }
}

impl Status {
    fn from_char(c: char) -> Status {
        match c {
            '?' => Status::Unknown,
            '#' => Status::Active,
            '.' => Status::Inactive,
            _ => panic!(),
        }
    }
}

/*
fn arrangements(v: &[Status]) -> Vec<Vec<Status>> {
    if !v.contains(&Status::Unknown) {
        vec![v.to_vec()]
    } else {
        let mut ret = vec![];
        for (idx, s) in v.iter().enumerate() {
            if s == &Status::Unknown {
                let mut resolved_active = v.to_vec();
                resolved_active[idx] = Status::Active;
                ret.extend(arrangements(&resolved_active));

                let mut resolved_inactive = v.to_vec();
                resolved_inactive[idx] = Status::Inactive;
                ret.extend(arrangements(&resolved_inactive));
            }
        }
        //println!("{v:?} -> {ret:?}");
        ret
    }
}

fn to_config(v: &[Status]) -> Vec<usize> {
    let mut c = vec![];
    let mut state = Status::Inactive;
    let mut it = v.iter();
    while let Some(s) = it.next() {
        match s {
            Status::Active => {
                if state == Status::Inactive {
                    c.push(1);
                    state = Status::Active;
                } else {
                    *c.last_mut().unwrap() += 1;
                }
            }
            Status::Inactive => {
                state = Status::Inactive;
            }
            Status::Unknown => panic!(),
        }
    }
    c
}
*/

fn permutations<'s>(springs: &'s [Status]) -> Box<dyn Iterator<Item = Vec<usize>> + 's> {
    // If the solution contains a '.', the solutions is always cartesian_product(perm(left), perm(right))
    if let Some(dot_pos) = springs.iter().position(|s| s == &Status::Inactive) {
        let left = &springs[0..dot_pos];
        let right = &springs[dot_pos + 1..];

        let pleft = permutations(left).collect_vec();
        let pright = permutations(right).collect_vec();

        let solutions = pleft
            .into_iter()
            .cartesian_product(pright)
            .map(|(left, right)| left.into_iter().chain(right).collect());

        Box::new(solutions)
    }
    // If the solution contains any '?', the solution is all permutations where ? is swapped for either '.' or '#'
    else if springs.contains(&Status::Unknown) {
        let unknown_indices = springs
            .iter()
            .enumerate()
            .filter_map(|(idx, status)| (status == &Status::Unknown).then_some(idx));
        let solutions = unknown_indices.flat_map(|idx| {
            let mut resolved_active = springs.to_vec();
            resolved_active[idx] = Status::Active;
            let pactive = permutations(&resolved_active).collect_vec();

            let mut resolved_inactive = springs.to_vec();
            resolved_inactive[idx] = Status::Inactive;
            let pinactive = permutations(&resolved_inactive).collect_vec();

            pactive.into_iter().chain(pinactive)
        });

        Box::new(solutions)
    }
    // If the solution is only "###" then the solution is len("###")
    else {
        Box::new(iter::once(vec![springs.len()]))
    }
}

fn part1(input: &str) -> i64 {
    let lines = input.lines();

    let rows = lines.map(|line| {
        let (left, right) = line.split_once(' ').unwrap();
        let springs = left.chars().map(Status::from_char).collect_vec();
        let config = right
            .split(',')
            .map(|n| n.parse::<usize>().unwrap())
            .collect_vec();
        (springs, config)
    });

    let counts = rows.map(|(springs, config)| {
        let count = permutations(&springs).filter(|p| p == &config).count();
        println!("{springs:?} -> {count}");
        count
    });

    counts.sum::<usize>() as i64
}

fn main() -> anyhow::Result<()> {
    let p1_test = part1(INPUT);
    assert_eq!(p1_test, 21);

    /*
    let p1 = part1(INPUT);
    println!("Part 1: {}", p1);
    */
    Ok(())
}

/*
fn arrangements(v: &[Status], depth: usize) -> Vec<Vec<usize>> {
    for _ in 0..depth {
        print!("\t");
    }
    println!("> {:?}", v);

    if !v.contains(&Status::Inactive) {
        if !v.contains(&Status::Unknown) {
            if v.is_empty() {
                vec![]
            } else {
                let len = v.len();
                for _ in 0..depth {
                    print!("\t");
                }
                println!("-> {len}");
                vec![vec![len]]
            }
        } else {
            let mut a = vec![];
            for (idx, s) in v.iter().enumerate() {
                if s == &Status::Unknown {
                    let mut resolved_active = v.to_vec();
                    resolved_active[idx] = Status::Active;
                    a.extend(arrangements(&resolved_active, depth + 1));

                    let mut resolved_inactive = v.to_vec();
                    resolved_inactive[idx] = Status::Inactive;
                    a.extend(arrangements(&resolved_inactive, depth + 1));
                }
            }
            for _ in 0..depth {
                print!("\t");
            }
            let a = a
                .iter()
                .filter(|&x| {
                    let sum = x.iter().sum::<usize>();
                    sum < v.len() || x.len() == 1
                })
                .cloned()
                .collect_vec();
            println!("-> {:?}", a);
            a
        }
    }
    // Split on 'Inactive' and return the sides
    else {
        let (left_stati, right_stati) =
            v.split_at(v.iter().position(|s| s == &Status::Inactive).unwrap());
        // Skip the included inactive sym
        let right_stati = &right_stati[1..];

        //println!("left: {left_stati:?}, right: {right_stati:?}");

        let left_arrangements = arrangements(&left_stati, depth + 1)
            .into_iter()
            .flat_map(|left| left.into_iter());
        let right_arrangements = arrangements(&right_stati, depth + 1)
            .into_iter()
            .flat_map(|right| right.into_iter());

        let result = match (
            left_arrangements.clone().count(),
            right_arrangements.clone().count(),
        ) {
            (0, 0) => vec![],
            (0, _) => vec![right_arrangements.collect_vec()],
            (_, 0) => vec![left_arrangements.collect_vec()],
            (_, _) => left_arrangements
                .cartesian_product(right_arrangements)
                .map(|(l, r)| vec![l, r])
                .collect_vec(),
        };
        for _ in 0..depth {
            print!("\t");
        }
        println!("-> {:?}", result);

        result
    }
}
*/

// This returns OK but takes too much time ':D
/*
#[test]
fn config_interpret_works() {
    assert!(
        arrangements(&"???.###".chars().map(Status::from_char).collect_vec())
            .into_iter()
            .map(|a| to_config(&a))
            .collect_vec()
            .contains(&vec![1usize, 1, 3])
    );
    assert!(arrangements(
        &".??..??...?##."
            .chars()
            .map(Status::from_char)
            .collect_vec()
    )
    .into_iter()
    .map(|a| to_config(&a))
    .collect_vec()
    .contains(&vec![1usize, 1, 3]));
    assert!(arrangements(
        &"?#?#?#?#?#?#?#?"
            .chars()
            .map(Status::from_char)
            .collect_vec()
    )
    .into_iter()
    .map(|a| to_config(&a))
    .collect_vec()
    .contains(&vec![1usize, 3, 1, 6]));
    assert!(
        arrangements(&"????.#...#...".chars().map(Status::from_char).collect_vec())
            .into_iter()
            .map(|a| to_config(&a))
            .collect_vec()
            .contains(&vec![4usize, 1, 1])
    );
    assert!(arrangements(
        &"????.######..#####."
            .chars()
            .map(Status::from_char)
            .collect_vec()
    )
    .into_iter()
    .map(|a| to_config(&a))
    .collect_vec()
    .contains(&vec![1usize, 6, 5]));
    assert!(
        arrangements(&"?###????????".chars().map(Status::from_char).collect_vec())
            .into_iter()
            .map(|a| to_config(&a))
            .collect_vec()
            .contains(&vec![3usize, 2, 1])
    );
}
*/
