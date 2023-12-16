use itertools::Itertools;

const INPUT: &str = include_str!("inputs/day11.txt");
fn transpose<T>(v: Vec<Vec<T>>) -> Vec<Vec<T>> {
    assert!(!v.is_empty());
    let len = v[0].len();
    let mut iters: Vec<_> = v.into_iter().map(|n| n.into_iter()).collect();
    (0..len)
        .map(|_| {
            iters
                .iter_mut()
                .map(|n| n.next().unwrap())
                .collect::<Vec<T>>()
        })
        .collect()
}

fn is_between(x: usize, low: usize, high: usize) -> bool {
    (x > low && x < high) || (x < low && x > high)
}

fn dist(
    a: (usize, usize),
    b: (usize, usize),
    exp_factor: i64,
    expy: &[usize],
    expx: &[usize],
) -> i64 {
    let ydist = (b.0 as i64 - a.0 as i64).abs();
    let xdist = (b.1 as i64 - a.1 as i64).abs();
    let dist = ydist
        + xdist
        + (exp_factor - 1)
            * expy
                .iter()
                .filter(|exp| is_between(**exp, b.0, a.0))
                .count() as i64
        + (exp_factor - 1)
            * expx
                .iter()
                .filter(|exp| is_between(**exp, b.1, a.1))
                .count() as i64;
    dist
}

fn pairwise_dist<'a>(
    galaxies: &'a [(usize, usize)],
    exp_factor: i64,
    expy: &'a [usize],
    expx: &'a [usize],
) -> impl Iterator<Item = i64> + 'a {
    galaxies.iter().combinations(2).map(move |pair| {
        let (a, b) = (pair[0], pair[1]);
        dist(*a, *b, exp_factor, expy, expx)
    })
}

fn main() {
    let lines = INPUT.lines();

    let univ = lines.map(|line| line.chars().collect_vec()).collect_vec();

    let expy = univ
        .iter()
        .enumerate()
        .filter_map(|(row, line)| line.iter().all(|c| *c == '.').then_some(row))
        .collect_vec();
    let expx = transpose(univ.clone())
        .iter()
        .enumerate()
        .filter_map(|(row, line)| line.iter().all(|c| *c == '.').then_some(row))
        .collect_vec();

    let galaxies = univ
        .iter()
        .enumerate()
        .flat_map(|(row, line)| {
            line.iter()
                .enumerate()
                .filter_map(move |(col, c)| (*c == '#').then_some((row, col)))
        })
        .collect_vec();

    println!(
        "Part 1: {}",
        pairwise_dist(&galaxies, 2, &expy, &expx).sum::<i64>()
    );
    println!(
        "Part 2: {}",
        pairwise_dist(&galaxies, 1_000_000, &expy, &expx).sum::<i64>()
    );
}
