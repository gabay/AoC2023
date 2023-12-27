use std::ops::{RangeInclusive, Sub};

use itertools::Itertools;

fn main() {
    let input = aoc2023::readfile("inputs/24");
    println!(
        "part1: {}",
        part1(&input, 200000000000000f64..=400000000000000f64)
    );
    println!("part2: {}", part2(&input));
}

fn part1(input: &str, range: RangeInclusive<f64>) -> usize {
    let hailstones = input.lines().map(Hailstone::from).collect_vec();

    hailstones
        .iter()
        .tuple_combinations()
        .filter(|(a, b)| a.intersect_in_range(b, &range))
        .count()
}

fn part2(input: &str) -> usize {
    let hailstones = input.lines().map(Hailstone::from).collect_vec();
    let n = 200;
    let a = hailstones[0];
    let b = hailstones[1];
    for vx in (-n)..=n {
        for vy in (-n)..=n {
            let rv1 = Hailstone::from_vs(vx as f64, vy as f64, 0f64);
            let a1 = a - rv1;
            let (x, _, _, _) = hailstones
                .iter()
                .skip(1)
                .find_map(|hs: &Hailstone| a1.get_intersection(&(*hs - rv1)))
                .unwrap();
            if hailstones
                .iter()
                .all(|hs| a1.does_intersect_at_x(&(*hs - rv1), x))
            {
                for vz in (-n)..=n {
                    let rv2 = Hailstone::from_vs(vx as f64, vy as f64, vz as f64);
                    let a2 = a - rv2;
                    let b2 = b - rv2;
                    let (_, _, t1, t2) = a2.get_intersection(&b2).unwrap();
                    if ((a2.z + a2.vz * t1) - (b2.z + b2.vz * t2)).abs() < 0.5 {
                        let x = a.x + a2.vx * t1;
                        let y = a.y + a2.vy * t1;
                        let z = a.z + a2.vz * t1;
                        return (x + y + z) as usize;
                    }
                }
            }
            if vx == 196 && vy == -109 {
                panic!();
            }
        }
    }
    panic!("no solution...");
}

#[derive(Debug, Clone, Copy)]
struct Hailstone {
    x: f64,
    y: f64,
    z: f64,
    vx: f64,
    vy: f64,
    vz: f64,
}

impl Sub for Hailstone {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
            vx: self.vx - rhs.vx,
            vy: self.vy - rhs.vy,
            vz: self.vz - rhs.vz,
        }
    }
}

impl From<&str> for Hailstone {
    fn from(value: &str) -> Self {
        let (p, v) = value.split_once(" @ ").unwrap();
        let (x, y, z) = p.split(", ").map(aoc2023::parse).collect_tuple().unwrap();
        let (vx, vy, vz) = v.split(", ").map(aoc2023::parse).collect_tuple().unwrap();
        Self {
            x,
            y,
            z,
            vx,
            vy,
            vz,
        }
    }
}

impl Hailstone {
    fn from_vs(vx: f64, vy: f64, vz: f64) -> Self {
        Self {
            x: 0f64,
            y: 0f64,
            z: 0f64,
            vx,
            vy,
            vz,
        }
    }
    fn intersect_in_range(&self, other: &Hailstone, range: &RangeInclusive<f64>) -> bool {
        if let Some((x, y, t1, t2)) = self.get_intersection(other) {
            t1 >= 0f64 && t2 >= 0f64 && range.contains(&x) && range.contains(&y)
        } else {
            false
        }
    }

    fn get_intersection(&self, other: &Hailstone) -> Option<(f64, f64, f64, f64)> {
        get_intersection_t1_t2(
            self.x, self.y, self.vx, self.vy, other.x, other.y, other.vx, other.vy,
        )
    }

    fn does_intersect_at_x(&self, other: &Hailstone, x: f64) -> bool {
        if self.vx == 0f64 {
            return self.x == x;
        }
        if other.vx == 0f64 {
            return other.x == x;
        }
        let t1 = (x - self.x) / self.vx;
        let y1 = self.y + self.vy * t1;
        let t2 = (x - other.x) / other.vx;
        let y2 = other.y + other.vy * t2;
        t1 >= 0f64 && t2 >= 0f64 && (y1 - y2).abs() < 0.001
    }
}

#[allow(clippy::too_many_arguments)]
fn get_intersection_t1_t2(
    x1: f64,
    y1: f64,
    vx1: f64,
    vy1: f64,
    x2: f64,
    y2: f64,
    vx2: f64,
    vy2: f64,
) -> Option<(f64, f64, f64, f64)> {
    if vx1 == 0f64 && vx2 == 0f64 {
        None
    } else if vx1 == 0f64 {
        let t2 = (x1 - x2) / vx2;
        let y = y2 + vy2 * t2;
        let t1 = (y - y1) / vy1;
        Some((x1, y, t1, t2))
    } else if vx2 == 0f64 {
        let t1 = (x2 - x1) / vx1;
        let y = y1 + vy1 * t1;
        let t2 = (y - y2) / vy2;
        Some((x2, y, t1, t2))
    } else {
        // m1 = vy1 / vx1
        let m1 = vy1 / vx1;
        let m2 = vy2 / vx2;
        if m1 == m2 {
            None
        } else {
            // y1 - m1*x1 + m1*x == y2 - m2*x2 + m2*x
            // x = (y2 - y1 + m1*x1 - m2*x2) / (m1-m2)
            let x = ((y2 - y1) + m1 * x1 - m2 * x2) / (m1 - m2);
            // y = y1 - m1*x1 + m1*x
            let y = y1 - (m1 * x1) + (m1 * x);
            // x = x1 + vx1*t1
            // t1 = (x - x1) / vx
            let t1 = (x - x1) / vx1;
            let t2 = (x - x2) / vx2;
            Some((x.round(), y.round(), t1, t2))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "19, 13, 30 @ -2,  1, -2
18, 19, 22 @ -1, -1, -2
20, 25, 34 @ -2, -2, -4
12, 31, 28 @ -1, -2, -1
20, 19, 15 @  1, -5, -3";

    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT, 7f64..=27f64), 2);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(INPUT), 47);
    }
}
