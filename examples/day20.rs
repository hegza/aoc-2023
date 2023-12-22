use itertools::Itertools;
use std::{collections::*, fmt};

const INPUT: &str = include_str!("inputs/day20.txt");
const _TEST_INPUT: &str = include_str!("inputs/day20_test.txt");

#[derive(Debug, PartialEq)]
enum Mod {
    Flip(bool),
    UninitCon,
    Con(HashMap<String, Pulse>),
    Inv,
    Broad,
}

impl Mod {
    fn from_char(c: char) -> Mod {
        match c {
            '%' => Mod::Flip(false),
            '&' => Mod::UninitCon,
            _ => panic!(),
        }
    }

    fn resolve_pulse(&mut self, pulse_src: &str, in_pulse: Pulse) -> Option<Pulse> {
        match self {
            Mod::Flip(state) => {
                match in_pulse {
                    // High pulse is ignored
                    Pulse::Hi => None,
                    // Low pulse flips and sends
                    Pulse::Lo => {
                        *state = !*state;
                        Some(Pulse::from_hi(*state))
                    }
                }
            }
            Mod::Con(inputs) => {
                *inputs.get_mut(pulse_src).unwrap() = in_pulse;
                if inputs.iter().all(|(_, p)| *p == Pulse::Hi) {
                    Some(Pulse::Lo)
                } else {
                    Some(Pulse::Hi)
                }
            }
            Mod::Inv => Some(in_pulse.not()),
            Mod::Broad => Some(in_pulse),
            _ => panic!(),
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Pulse {
    Lo,
    Hi,
}

impl Pulse {
    fn from_hi(b: bool) -> Pulse {
        if b {
            Pulse::Hi
        } else {
            Pulse::Lo
        }
    }
    fn not(&self) -> Pulse {
        match self {
            Pulse::Lo => Pulse::Hi,
            Pulse::Hi => Pulse::Lo,
        }
    }
}

impl fmt::Debug for Pulse {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Lo => write!(f, "-low"),
            Self::Hi => write!(f, "-high"),
        }
    }
}

/// Returns if rx low
fn push_button(mods: &mut HashMap<String, (Mod, Vec<String>)>) -> bool {
    let mut q = VecDeque::new();

    q.push_back((
        "button".to_string(),
        Pulse::Lo,
        vec!["broadcaster".to_string()],
    ));

    while let Some((pulse_src, pulse, pulse_target_names)) = q.pop_front() {
        if pulse == Pulse::Lo && pulse_target_names.contains(&"rx".to_string()) {
            return true;
        }

        // Eliminate non-existing modules before dispatching subsequent pulses from them
        let pulse_target_names = pulse_target_names
            .into_iter()
            .filter(|in_tgt_name| mods.contains_key(in_tgt_name))
            .collect_vec();

        /*
        for tgt in &pulse_target_names {
            println!("{pulse_src} {pulse:?} -> {tgt}");
        }
        */

        let out_pulses = pulse_target_names.into_iter().filter_map(|pulse_tgt_name| {
            let (pulse_tgt_mod, subsequent_targets) = mods.get_mut(&pulse_tgt_name).unwrap();
            pulse_tgt_mod
                .resolve_pulse(&pulse_src, pulse)
                .and_then(|out_pulse| {
                    Some((
                        pulse_tgt_name.clone(),
                        out_pulse,
                        subsequent_targets.clone(),
                    ))
                })
        });
        q.extend(out_pulses);
    }

    false
}

fn main() -> anyhow::Result<()> {
    assert_eq!(part1::solve(INPUT), 919383692);

    println!("Part 2...");
    let p2 = part2(INPUT);

    println!("Part 2: {p2}");

    Ok(())
}

fn part2(input: &str) -> i64 {
    let mut mods = parse_input(input);
    for n in 1.. {
        if push_button(&mut mods) {
            return n;
        }
    }
    panic!()
}

fn parse_input(input: &str) -> HashMap<String, (Mod, Vec<String>)> {
    let mut mods: HashMap<_, _> = input
        .lines()
        .map(|line| {
            let (lefts, rights) = line.split_once("->").unwrap();
            let lefts = lefts.trim();
            let rights = rights.trim();

            let (mod_, name) = if lefts == "broadcaster" {
                (Mod::Broad, lefts.to_string())
            } else {
                let mut it = lefts.chars();
                let c = it.next().unwrap();
                (Mod::from_char(c), it.collect::<String>())
            };

            let targets = rights
                .split(',')
                .map(str::trim)
                .map(str::to_owned)
                .collect_vec();

            (name, (mod_, targets))
        })
        .collect();

    let mut inputs = HashMap::new();
    for (mod_name, (_, mod_targets)) in &mods {
        for tgt in mod_targets {
            inputs
                .entry(tgt.clone())
                .and_modify(|inputs: &mut Vec<_>| inputs.push(mod_name.clone()))
                .or_insert(vec![mod_name.clone()]);
        }
    }

    for (name, inputs) in inputs.into_iter() {
        if let Some(mod_) = mods.get(&name) {
            if mod_.0 == Mod::UninitCon {
                let con = mods.get_mut(&name).unwrap();
                if inputs.len() > 1 {
                    con.0 = Mod::Con(inputs.into_iter().map(|in_| (in_, Pulse::Lo)).collect());
                } else {
                    con.0 = Mod::Inv;
                }
            }
        }
    }

    mods
}

mod part1 {
    use itertools::Itertools;

    use crate::{parse_input, Mod, Pulse};
    use std::collections::{HashMap, VecDeque};

    /// Returns (low pulses, high pulses)
    fn push_button(mods: &mut HashMap<String, (Mod, Vec<String>)>) -> (usize, usize) {
        let mut q = VecDeque::new();

        q.push_back((
            "button".to_string(),
            Pulse::Lo,
            vec!["broadcaster".to_string()],
        ));

        simulate(q, mods)
    }

    fn simulate(
        mut q: VecDeque<(String, Pulse, Vec<String>)>,
        mods: &mut HashMap<String, (Mod, Vec<String>)>,
    ) -> (usize, usize) {
        let mut lo_pulses = 0;
        let mut hi_pulses = 0;

        use Pulse as P;
        while let Some((pulse_src, pulse, pulse_target_names)) = q.pop_front() {
            // Pulse sent
            *match pulse {
                P::Lo => &mut lo_pulses,
                P::Hi => &mut hi_pulses,
            } += pulse_target_names.len();

            // Eliminate non-existing modules before dispatching subsequent pulses from them
            let pulse_target_names = pulse_target_names
                .into_iter()
                .filter(|in_tgt_name| mods.contains_key(in_tgt_name))
                .collect_vec();

            /*
            for tgt in &pulse_target_names {
                println!("{pulse_src} {pulse:?} -> {tgt}");
            }
            */

            let out_pulses = pulse_target_names.into_iter().filter_map(|pulse_tgt_name| {
                let (pulse_tgt_mod, subsequent_targets) = mods.get_mut(&pulse_tgt_name).unwrap();
                pulse_tgt_mod
                    .resolve_pulse(&pulse_src, pulse)
                    .and_then(|out_pulse| {
                        Some((
                            pulse_tgt_name.clone(),
                            out_pulse,
                            subsequent_targets.clone(),
                        ))
                    })
            });
            q.extend(out_pulses);
        }

        (lo_pulses, hi_pulses)
    }

    pub(crate) fn solve(input: &str) -> i64 {
        let mut mods: HashMap<String, (Mod, Vec<String>)> = parse_input(input);

        let (lo_pulses, hi_pulses) = (0..1000)
            .map(|_| push_button(&mut mods))
            .fold((0, 0), |acc, (x0, x1)| (acc.0 + x0, acc.1 + x1));

        let mul = lo_pulses * hi_pulses;

        mul as i64
    }
}
