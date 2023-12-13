use itertools::Itertools;

const INPUT: &str = include_str!("inputs/day13.txt");

fn transpose(v: &Vec<String>) -> Vec<String> {
    assert!(!v.is_empty());

    let mut ret = vec![String::new(); v[0].len()];
    for line in v {
        line.chars().enumerate().for_each(|(idx, c)| {
            ret[idx].push(c);
        });
    }
    ret
}

fn candidates(lines: &[String]) -> impl Iterator<Item = usize> + Clone + '_ {
    lines
        .iter()
        .tuple_windows()
        .positions(|(l1, l2)| l1 == l2)
        .map(|x| x + 1)
}

fn is_mirror(line: usize, pat: &[String]) -> bool {
    let rowcount = line.min(pat.len() - line);

    (0..rowcount).all(|row| pat[line - row - 1] == pat[line + row])
}

fn main() -> anyhow::Result<()> {
    let mut lines = INPUT.lines();

    let mut patterns = vec![];
    let mut cur_pat = vec![];
    while let Some(line) = lines.next() {
        if line.is_empty() {
            patterns.push(cur_pat);
            cur_pat = vec![];
        } else {
            cur_pat.push(line.to_owned())
        }
    }
    patterns.push(cur_pat);

    let tpatterns = patterns.iter().map(|pattern| transpose(pattern));

    let sum: usize = patterns
        .iter()
        .zip(tpatterns)
        .enumerate()
        .map(|(pat_idx, (pat, tpat))| {
            let mut _candidates = candidates(&pat);
            /*println!(
                "hlines in pattern {pat_idx} at {:?}",
                &_candidates.clone().collect_vec()
            );*/
            if let Some(line) = _candidates.find(|line| is_mirror(*line, &pat)) {
                println!("hmirror in pattern {pat_idx} at {line}");
                line * 100
            } else {
                let mut _candidates = candidates(&tpat);
                /*println!(
                    "vlines in pattern {pat_idx} at {:?}",
                    &_candidates.clone().collect_vec()
                );*/
                let line = _candidates.find(|line| is_mirror(*line, &tpat)).unwrap();
                println!("vmirror in pattern {pat_idx} at {line}");
                line
            }
        })
        .sum();

    println!("{sum}");

    Ok(())
}
