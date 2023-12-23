use std::collections::{HashMap, HashSet};

use aoc2023::Point;
use itertools::Itertools;

fn main() {
    let input = aoc2023::readfile("inputs/22");
    println!("part1: {}", part1(&input));
    println!("part2: {}", part2(&input));
}

fn part1(input: &str) -> usize {
    let mut heights = Heights::new();
    let bricks = input
        .lines()
        .map(Brick::from)
        .sorted_by_key(|brick| brick.z1)
        .map(|brick| {
            let mut b = brick;
            while !b.down().collides_with_heights(&heights) {
                b = b.down();
            }
            b.xys().iter().for_each(|p| heights.set(*p, b.z2));
            b
        })
        .collect_vec();

    let supported_by = bricks
        .iter()
        .map(|b1| {
            bricks
                .iter()
                .filter(|b2| !b1.collides_with_brick(b2) && b1.down().collides_with_brick(b2))
                .collect_vec()
        })
        .collect_vec();
    let sole_supporting_bricks = supported_by
        .iter()
        .filter(|supporters| supporters.len() == 1)
        .flatten()
        .collect::<HashSet<_>>();

    bricks.len() - sole_supporting_bricks.len()
}

fn part2(input: &str) -> usize {
    let mut heights = Heights::new();
    let bricks = input
        .lines()
        .map(Brick::from)
        .sorted_by_key(|brick| brick.z1)
        .map(|brick| {
            let mut b = brick;
            while !b.down().collides_with_heights(&heights) {
                b = b.down();
            }
            b.xys().iter().for_each(|p| heights.set(*p, b.z2));
            b
        })
        .collect_vec();

    let supported_by = bricks
        .iter()
        .map(|b1| {
            bricks
                .iter()
                .filter(|b2| !b1.collides_with_brick(b2) && b1.down().collides_with_brick(b2))
                .collect::<HashSet<_>>()
        })
        .collect_vec();
    let sole_supporting_bricks = supported_by
        .iter()
        .filter(|supporters| supporters.len() == 1)
        .flatten()
        .collect::<HashSet<_>>();

    sole_supporting_bricks
        .iter()
        .map(|brick| {
            let mut bs = HashSet::new();
            bs.insert(**brick);
            loop {
                let mut stop = true;
                for (b, supported_by) in bricks.iter().zip(&supported_by) {
                    if !bs.contains(b) && !supported_by.is_empty() && supported_by.is_subset(&bs) {
                        bs.insert(b);
                        stop = false;
                    }
                }
                if stop {
                    break;
                }
            }
            bs.len()
        })
        .sum::<usize>()
        - sole_supporting_bricks.len()
}

#[derive(Debug, PartialEq, Eq, Hash)]
struct Brick {
    x1: usize,
    y1: usize,
    z1: usize,
    x2: usize,
    y2: usize,
    z2: usize,
}

impl From<&str> for Brick {
    fn from(value: &str) -> Self {
        let (a, b) = value.split_once('~').unwrap();
        let (x1, y1, z1) = a
            .split(',')
            .map(aoc2023::parse::<usize>)
            .collect_tuple()
            .unwrap();
        let (x2, y2, z2) = b
            .split(',')
            .map(aoc2023::parse::<usize>)
            .collect_tuple()
            .unwrap();
        let (x1, x2) = aoc2023::minmax(x1, x2);
        let (y1, y2) = aoc2023::minmax(y1, y2);
        let (z1, z2) = aoc2023::minmax(z1, z2);
        Self {
            x1,
            y1,
            z1,
            x2,
            y2,
            z2,
        }
    }
}

impl Brick {
    fn xys(&self) -> Vec<Point> {
        (self.x1..=self.x2)
            .flat_map(|x| {
                (self.y1..=self.y2)
                    .map(|y| Point::new(x as i32, y as i32))
                    .collect_vec()
            })
            .collect()
    }

    fn down(&self) -> Self {
        Self {
            x1: self.x1,
            y1: self.y1,
            z1: self.z1 - 1,
            x2: self.x2,
            y2: self.y2,
            z2: self.z2 - 1,
        }
    }

    fn collides_with_brick(&self, other: &Self) -> bool {
        self.x1 <= other.x2
            && self.y1 <= other.y2
            && self.z1 <= other.z2
            && self.x2 >= other.x1
            && self.y2 >= other.y1
            && self.z2 >= other.z1
    }

    fn collides_with_heights(&self, heights: &Heights) -> bool {
        self.xys().iter().any(|p| self.z1 <= heights.get(*p))
    }
}

struct Heights(HashMap<Point, usize>);

impl Heights {
    fn new() -> Self {
        Self(HashMap::new())
    }

    fn get(&self, p: Point) -> usize {
        *self.0.get(&p).unwrap_or(&0)
    }

    fn set(&mut self, p: Point, height: usize) {
        self.0.insert(p, height);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "1,0,1~1,2,1
0,0,2~2,0,2
0,2,3~2,2,3
0,0,4~0,2,4
2,0,5~2,2,5
0,1,6~2,1,6
1,1,8~1,1,9";

    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT), 5);
    }
    #[test]
    fn test_part2() {
        assert_eq!(part2(INPUT), 7);
    }
}
