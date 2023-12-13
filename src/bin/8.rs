use std::collections::HashMap;

use itertools::Itertools;
use lazy_static::lazy_static;
use num::integer::lcm;
use regex::Regex;

lazy_static! {
    static ref LOCATION_RE: Regex = Regex::new(r"\w{3}").unwrap();
}

fn main() {
    let input = aoc2023::readfile("inputs/8");
    println!("part1: {}", part1(&input));
    println!("part2: {}", part2(&input));
}

fn part1(input: &str) -> i128 {
    let pattern = Pattern::new(input.lines().next().unwrap());
    let location_map = LocationMap::from_data(input);
    location_map.count_steps(Location("AAA"), Location("ZZZ"), pattern.clone())
}

fn part2(input: &str) -> i128 {
    let pattern = Pattern::new(input.lines().next().unwrap());
    let location_map = LocationMap::from_data(input);
    location_map.count_steps2(pattern.clone())
}

#[derive(Clone)]
struct Pattern {
    chars: Vec<char>,
    index: usize,
}

impl Pattern {
    fn new(s: &str) -> Self {
        Pattern {
            chars: s.chars().collect(),
            index: 0,
        }
    }
}

impl Iterator for Pattern {
    type Item = char;

    fn next(&mut self) -> Option<Self::Item> {
        let item = self.chars[self.index];
        self.index = (self.index + 1) % self.chars.len();
        Some(item)
    }
}

#[derive(Eq, PartialEq, Hash, Debug)]
struct Location<'a>(&'a str);

impl<'a> Location<'a> {
    fn is_source(&self) -> bool {
        self.0.chars().last() == Some('A')
    }

    fn is_dest(&self) -> bool {
        self.0.chars().last() == Some('Z')
    }
}
struct LocationMap<'a> {
    left: HashMap<Location<'a>, Location<'a>>,
    right: HashMap<Location<'a>, Location<'a>>,
}

impl<'a> LocationMap<'a> {
    fn from_data(data: &'a str) -> Self {
        let items = data
            .lines()
            .skip(2)
            .map(|line| {
                LOCATION_RE
                    .find_iter(line)
                    .map(|loc| loc.as_str())
                    .collect_tuple()
                    .unwrap()
            })
            .collect_vec();
        let left = items
            .iter()
            .map(|(loc, left, _)| (Location(loc), Location(left)))
            .collect::<HashMap<_, _>>();
        let right = items
            .iter()
            .map(|(loc, _, right)| (Location(loc), Location(right)))
            .collect::<HashMap<_, _>>();
        Self { left, right }
    }

    fn count_steps(&self, source: Location, destination: Location, pattern: Pattern) -> i128 {
        let mut current = &source;
        let mut steps = 0;
        for char in pattern {
            if *current == destination {
                break;
            }
            current = self.get_next_location(current, char);
            steps += 1;
        }
        steps
    }

    fn get_next_location(&self, loc: &'a Location, c: char) -> &'a Location {
        if c == 'L' {
            &self.left[loc]
        } else {
            &self.right[loc]
        }
    }

    fn count_steps2(&self, pattern: Pattern) -> i128 {
        let locations = self.get_initial_locations();
        let solution_steps: Vec<_> = locations
            .iter()
            .map(|loc| self.get_solution_steps(loc, pattern.clone()))
            .collect();
        let mut result = 0;
        let mut total_modulus = 1;
        for solution in solution_steps {
            let modulus = solution.modulus();
            let remainder = solution.remainder();
            let (_, x, y) = extended_gcd(total_modulus, modulus);
            result = (result * modulus * y) + (remainder * total_modulus * x);
            total_modulus = lcm(total_modulus, modulus);
            result = result % total_modulus;
        }
        result % total_modulus
    }

    fn get_initial_locations(&self) -> Vec<&Location> {
        self.left.keys().filter(|loc| loc.is_source()).collect()
    }

    fn get_solution_steps(&self, loc: &Location, mut pattern: Pattern) -> SolutionSteps {
        let mut curr = loc;
        let mut state_to_steps: HashMap<(&Location, usize), i128> = HashMap::new();
        let mut solution_steps = vec![];
        let mut steps = 0;
        loop {
            let c: char = pattern.next().unwrap();
            if curr.is_dest() {
                solution_steps.push(steps);
            }
            if state_to_steps.contains_key(&(curr, pattern.index)) {
                // Done
                let period_start = state_to_steps[&(curr, pattern.index)];
                let period_solution_steps = solution_steps
                    .iter()
                    .filter(|i| **i >= period_start)
                    .map(|i| i + 1 - period_start)
                    .collect();
                return SolutionSteps {
                    next: solution_steps,
                    period: period_solution_steps,
                };
            }
            state_to_steps.insert((curr, pattern.index), steps);
            curr = self.get_next_location(curr, c);
            steps += 1;
        }
    }
}

#[derive(Debug)]
struct SolutionSteps {
    next: Vec<i128>,
    period: Vec<i128>,
}

impl SolutionSteps {
    fn remainder(&self) -> i128 {
        self.next.first().unwrap() % self.modulus()
    }

    fn modulus(&self) -> i128 {
        *self.period.first().unwrap()
    }

    fn intersect_solutions(&self, other: &SolutionSteps) -> SolutionSteps {
        assert!(self.next.len() == 1);
        assert!(self.period.len() == 1);
        assert!(other.next.len() == 1);
        assert!(other.period.len() == 1);
        let period = lcm(self.period[0], other.period[0]);
        let mut a = self.next[0];
        let mut b = other.next[0];
        while a != b {
            if a < b {
                let steps = (b - a + self.period[0] - 1) / self.period[0];
                a += self.period[0] * steps;
                assert!(a >= b);
            } else {
                let steps = 1 + ((a - b + other.period[0] - 1) / other.period[0]);
                b += other.period[0] * steps;
                assert!(b >= a);
            }
        }
        println!("{:?} {:?}", self, other);
        SolutionSteps {
            next: vec![a],
            period: vec![period],
        }
    }
}

impl Iterator for SolutionSteps {
    type Item = i128;
    fn next(&mut self) -> Option<Self::Item> {
        let value = self.next.pop().unwrap();
        if self.next.is_empty() {
            for p in self.period.iter() {
                self.next.push(value + p);
            }
        }
        Some(value)
    }
}

fn extended_gcd(mut a: i128, mut b: i128) -> (i128, i128, i128) {
    if a > b {
        let (gcd, m1, m2) = extended_gcd(b, a);
        return (gcd, m2, m1);
    }
    let (mut x, mut y, mut u, mut v) = (0, 1, 1, 0);
    while a != 0 {
        let quotient = b / a;
        let remainder = b % a;
        let m1 = x - u * quotient;
        let m2 = y - v * quotient;
        b = a;
        a = remainder;
        x = u;
        y = v;
        u = m1;
        v = m2;
    }
    let gcd = b;
    return (gcd, x, y);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_1() {
        let input = "RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)";
        assert_eq!(part1(input), 2);
    }

    #[test]
    fn test_part1_2() {
        let input = "LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)";
        assert_eq!(part1(input), 6);
    }
}
