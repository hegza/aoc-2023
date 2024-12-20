use itertools::Itertools;

const INPUT: &str = include_str!("inputs/day24.txt");
const _TEST_INPUT: &str = include_str!("inputs/day24_test.txt");

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct V2((i64, i64));

#[derive(Debug)]
struct F2((f64, f64));

impl std::ops::Sub for V2 {
    type Output = V2;

    fn sub(self, rhs: Self) -> Self::Output {
        V2((self.0 .0 - rhs.0 .0, self.0 .1 - rhs.0 .1))
    }
}

impl V2 {
    fn as_tuple(&self) -> (i64, i64) {
        self.0
    }
}

impl From<V2> for F2 {
    fn from(value: V2) -> Self {
        F2((value.0 .0 as f64, value.0 .1 as f64))
    }
}

impl std::ops::Mul<f64> for F2 {
    type Output = F2;

    fn mul(self, rhs: f64) -> Self::Output {
        F2((self.0 .0 * rhs, self.0 .1 * rhs))
    }
}

impl std::ops::Add for F2 {
    type Output = F2;

    fn add(self, rhs: Self) -> Self::Output {
        F2((self.0 .0 + rhs.0 .0, self.0 .1 + rhs.0 .1))
    }
}

impl std::ops::Mul<f64> for V2 {
    type Output = F2;

    fn mul(self, rhs: f64) -> Self::Output {
        F2::from(self) * rhs
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct V3((i64, i64, i64));

impl V3 {
    fn trunc(self) -> V2 {
        V2((self.0 .0, self.0 .1))
    }
}

enum Intersect {
    Future((f64, f64)),
    Never,
    Past((f64, f64)),
}

fn future_intersect(x0: V2, v0: V2, x1: V2, v1: V2) -> Intersect {
    let (x0_x, x0_y) = x0.as_tuple();
    let (v0_x, v0_y) = v0.as_tuple();
    let (x1_x, x1_y) = x1.as_tuple();
    let (v1_x, v1_y) = v1.as_tuple();

    // Check for parallel
    let dvx = v0_x - v1_x;
    let dvy = v0_y - v1_y;
    if dvx == dvy {
        return Intersect::Never;
    }

    // Calculate time of intersection along each axis
    let tx = (x1_x - x0_x) as f64 / dvx as f64;
    let ty = (x1_y - x0_y) as f64 / dvy as f64;

    // Point of intersect
    let px = x0_x as f64 + tx * (v0_x as f64);
    let py = x0_y as f64 + ty * (v0_y as f64);

    // Intersection happened in the past
    if tx < 0. || ty < 0. {
        return Intersect::Past((px, py));
    }

    Intersect::Future((px, py))
}

fn main() -> anyhow::Result<()> {
    let lines = _TEST_INPUT.lines();

    let hailstones = lines
        .map(|line| {
            let (pos, vel) = line.split_once('@').unwrap();

            let mut pos_it = pos.split(',').map(|s| s.trim().parse::<i64>().unwrap());
            let (x, y, z) = (
                pos_it.next().unwrap(),
                pos_it.next().unwrap(),
                pos_it.next().unwrap(),
            );
            let mut vel_it = vel.split(',').map(|s| s.trim().parse::<i64>().unwrap());
            let (vx, vy, vz) = (
                vel_it.next().unwrap(),
                vel_it.next().unwrap(),
                vel_it.next().unwrap(),
            );

            (V3((x, y, z)), V3((vx, vy, vz)))
        })
        .collect_vec();

    let min: f64 = 7f64; //200000000000000f64;
    let max: f64 = 27f64; //400000000000000f64;

    let hailstones_2d = hailstones
        .iter()
        .map(|(x0, v0)| (x0.trunc(), v0.trunc()))
        .collect_vec();

    let mut count = 0;
    for idx0 in 0..hailstones_2d.len() {
        let (x0, v0) = hailstones_2d[idx0];
        for idx1 in idx0 + 1..hailstones_2d.len() {
            let (x1, v1) = hailstones_2d[idx1];
            match future_intersect(x0, v0, x1, v1) {
                Intersect::Future((px, py)) => {
                    if px >= min && px <= max && py >= min && py <= max {
                        println!(
                            "Hailstones {:?} and {:?} will cross inside the test area (at {:?})",
                            x0,
                            x1,
                            (px, py)
                        );
                        count += 1;
                    } else {
                        println!(
                            "Hailstones {:?} and {:?} will cross outside the test area (at {:?})",
                            x0,
                            x1,
                            (px, py)
                        );
                    }
                }
                Intersect::Never => {
                    println!(
                        "Hailstones' {:?} and {:?} paths are parallel; they never intersect",
                        x0, x1,
                    );
                }
                Intersect::Past((px, py)) => {
                    println!(
                        "Hailstones {:?} and {:?} crossed in the past at {:?}",
                        x0,
                        x1,
                        (px, py)
                    );
                }
            }
        }
    }

    println!("{count}");

    Ok(())
}
