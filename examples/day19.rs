use itertools::Itertools;
use lazy_static::lazy_static;
use regex::Regex;
use std::{cmp, collections::*, iter};

const INPUT: &str = include_str!("inputs/day19.txt");
const TEST_INPUT: &str = include_str!("inputs/day19_test.txt");

lazy_static! {
    pub static ref WORKFLOW_RE: Regex = Regex::new(r"^(\w+)\{(.+),(\w+)\}$").unwrap();
    pub static ref RULE_RE: Regex = Regex::new(r"([[:alpha:]])([><])(\d+):(\w+)").unwrap();
    pub static ref PARTS_RE: Regex = Regex::new(r"([[:alpha:]])=(\d+)").unwrap();
}

#[derive(Debug)]
struct Rule {
    rating: char,
    ord: cmp::Ordering,
    rhs: usize,
    target_container: String,
}

struct Workflow {
    name: String,
    rules: Vec<Rule>,
    default: String,
}

impl Workflow {
    fn apply(&self, part: &Part) -> Option<String> {
        for rule in &self.rules {
            if part[&rule.rating].cmp(&rule.rhs) == rule.ord {
                return Some(rule.target_container.clone());
            }
        }
        None
    }

    fn partitions(&self, mut part: Partition) -> Vec<(String, Partition)> {
        let mut result = vec![];

        // Break off partitions into matching containers
        for rule in &self.rules {
            let (a, b) = part.break_off(rule);
            result.push((rule.target_container.clone(), a));
            part = b;
        }

        // Put the rest in the default bin
        result.push((self.default.clone(), part));

        result
    }
}

#[derive(Clone, Debug)]
struct Partition(HashMap<char, (usize, usize)>);

impl Default for Partition {
    fn default() -> Self {
        Self(
            [
                ('x', (1, 4000)),
                ('m', (1, 4000)),
                ('a', (1, 4000)),
                ('s', (1, 4000)),
            ]
            .into_iter()
            .collect(),
        )
    }
}

impl Partition {
    /// Returns (rule, not rule)
    fn break_off(self, rule: &Rule) -> (Partition, Partition) {
        print!("{:?} was broken into ", &self);

        let mut a = self.clone();
        let mut b = self.clone();

        match rule.ord {
            cmp::Ordering::Less => {
                // Set max value as determined by rule
                let amax = &mut a.0.get_mut(&rule.rating).unwrap().1;
                *amax = rule.rhs - 1;

                let bmin = &mut b.0.get_mut(&rule.rating).unwrap().0;
                *bmin = rule.rhs;
            }
            cmp::Ordering::Greater => {
                // Set min value as determined by rule
                let amin = &mut a.0.get_mut(&rule.rating).unwrap().0;
                *amin = rule.rhs + 1;

                let bmax = &mut b.0.get_mut(&rule.rating).unwrap().1;
                *bmax = rule.rhs;
            }
            _ => panic!(),
        }

        println!("{a:?} + {b:?}");

        (a, b)
    }

    fn magnitude(&self) -> usize {
        (self.0[&'x'].1 - self.0[&'x'].0 + 1)
            * (self.0[&'m'].1 - self.0[&'m'].0 + 1)
            * (self.0[&'a'].1 - self.0[&'a'].0 + 1)
            * (self.0[&'s'].1 - self.0[&'s'].0 + 1)
    }
}

type Part = HashMap<char, usize>;

fn build_dag(workflows: &HashMap<String, Workflow>) -> HashMap<String, Vec<String>> {
    let mut graph = HashMap::new();

    for (name, w) in workflows {
        let targets = iter::once(w.default.clone())
            .chain(w.rules.iter().map(|rule| rule.target_container.clone()))
            .collect_vec();
        graph.insert(name.clone(), targets);
    }

    graph
}

fn resolve_partitions(init: Partition, workflows: &HashMap<String, Workflow>) -> Vec<Partition> {
    let mut out = Vec::new();

    let mut q = VecDeque::new();
    q.push_back(("in".to_string(), init));

    while let Some((cur, in_part)) = q.pop_front() {
        let w = &workflows[&cur];
        for (tgt, out_part) in w.partitions(in_part) {
            // Skip rejects
            if tgt.as_str() == "R" {
                continue;
            }
            // Store accepted separately
            else if tgt.as_str() == "A" {
                out.push(out_part);
            }
            // Push out partitions into next container
            else {
                q.push_back((tgt.clone(), out_part));
            }
        }
    }

    out
}

/*
struct UnionPartition(HashMap<char, Vec<(usize, usize)>>);

impl UnionPartition {
    fn from_vec(parts: &[Partition]) -> UnionPartition {
        let mut union = [('x', vec![]), ('m', vec![]), ('a', vec![]), ('s', vec![])];

        for part in parts {

        }

        union
    }
}
*/

fn union_magnitude(parts: &[Partition]) -> usize {
    (1..=4000)
        .cartesian_product(1..=4000)
        .cartesian_product(1..=4000)
        .cartesian_product(1..=4000)
        // Count this instance only if there is a partition that contains this value combination
        .filter(|(((x, m), a), s)| {
            parts.iter().any(|part| {
                let (xmin, xmax) = part.0.get(&'x').unwrap();
                let (mmin, mmax) = part.0.get(&'m').unwrap();
                let (amin, amax) = part.0.get(&'a').unwrap();
                let (smin, smax) = part.0.get(&'s').unwrap();

                x >= xmin
                    && x <= xmax
                    && m >= mmin
                    && m <= mmax
                    && a >= amin
                    && a <= amax
                    && s >= smin
                    && s <= smax
            })
        })
        .count()
}

fn main() -> anyhow::Result<()> {
    let (workflows, parts) = INPUT.split_once("\n\n").unwrap();
    let workflows: HashMap<String, Workflow> = workflows
        .lines()
        .map(|line| {
            let mut cap = WORKFLOW_RE.captures_iter(line);
            let m = cap.next().unwrap();

            let name = m[1].to_string();
            let rules = m[2].to_string();

            let rules_cap = RULE_RE.captures_iter(&rules);

            let rules = rules_cap
                .map(|rule| {
                    let rating = rule[1].chars().next().unwrap();
                    let ord = match rule[2].chars().next().unwrap() {
                        '<' => cmp::Ordering::Less,
                        '>' => cmp::Ordering::Greater,
                        _ => panic!(),
                    };
                    let target = rule[3].parse::<usize>().unwrap();
                    let target_container = rule[4].to_string();

                    Rule {
                        rating,
                        ord,
                        rhs: target,
                        target_container,
                    }
                })
                .collect_vec();

            let default = m[3].to_string();

            (
                name.clone(),
                Workflow {
                    name,
                    rules,
                    default,
                },
            )
        })
        .collect();

    let init = Partition::default();

    let accepted = resolve_partitions(init, &workflows);
    /*
    for part in &accepted {
        println!("{:?}", part);
    }
    let accepted = union_magnitude(&accepted);
    */
    let mag = accepted.into_iter().map(|p| p.magnitude()).sum::<usize>();
    println!("p2: {mag}");

    Ok(())
}

fn part1(workflows: &HashMap<String, Workflow>, parts_str: &str) -> i64 {
    let parts = parts_str
        .lines()
        .map(|line| {
            let mut cap = PARTS_RE.captures_iter(line);
            let x = {
                let c = cap.next().unwrap();
                assert_eq!(&c[1], "x");
                c[2].parse::<usize>().unwrap()
            };
            let m = {
                let c = cap.next().unwrap();
                assert_eq!(&c[1], "m");
                c[2].parse::<usize>().unwrap()
            };
            let a = {
                let c = cap.next().unwrap();
                assert_eq!(&c[1], "a");
                c[2].parse::<usize>().unwrap()
            };
            let s = {
                let c = cap.next().unwrap();
                assert_eq!(&c[1], "s");
                c[2].parse::<usize>().unwrap()
            };

            [('x', x), ('m', m), ('a', a), ('s', s)]
                .into_iter()
                .collect::<Part>()
        })
        .collect_vec();

    let mut qs: HashMap<String, VecDeque<Part>> = workflows
        .iter()
        .map(|(name, _)| (name.clone(), VecDeque::new()))
        .collect();

    qs.insert("A".to_string(), VecDeque::new());
    qs.insert("R".to_string(), VecDeque::new());

    for part in parts {
        let q = qs.get_mut("in").unwrap();
        q.push_back(part.clone());
    }

    let keys = qs.keys().cloned().collect_vec();
    loop {
        let mut any_left = false;
        for qname in &keys {
            let q = qs.get_mut(qname).unwrap();
            if qname == "A" || qname == "R" {
                continue;
            }
            if let Some(part) = q.pop_front() {
                any_left = true;
                let w = &workflows[qname];
                let next_q = if let Some(target) = w.apply(&part) {
                    target.clone()
                } else {
                    w.default.clone()
                };
                qs.get_mut(&next_q).unwrap().push_back(part);
            }
        }
        if !any_left {
            break;
        }
    }

    let accepted = qs.get("A").unwrap();
    for part in accepted {
        println!("Accepted: {part:?}");
    }

    let p1: usize = accepted
        .iter()
        .map(|part| part.iter().map(|(_, val)| *val).sum::<usize>())
        .sum();

    p1 as i64
}
