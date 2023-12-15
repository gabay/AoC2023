use regex::Regex;
use lazy_static::lazy_static;

fn main() {
    let input = aoc2023::readfile("inputs/2");
    println!("part1: {}", part1(&input));
    println!("part2: {}", part2(&input));
}

fn part1(input: &str) -> i32 {
    input
        .split('\n')
        .map(Game::from)
        .filter(Game::is_feasible)
        .map(|game| game.id)
        .sum()
}

fn part2(input: &str) -> i32 {
    input
        .split('\n')
        .map(|line| Game::from(line).get_minimum_set_power())
        .sum()
}

struct Game {
    id: i32,
    sets: Vec<Set>,
}

lazy_static! {
    static ref GAME_RE: Regex = Regex::new(r"Game (\d+):(.*)").unwrap();
}

impl From<&str> for Game {
    fn from(value: &str) -> Self {
        let capture = GAME_RE.captures(value).unwrap();
        let id = aoc2023::parse(capture.get(1).unwrap().as_str());
        let sets: Vec<Set> = capture
            .get(2)
            .unwrap()
            .as_str()
            .split(';')
            .map(Set::from)
            .collect();
        Self { id, sets }
    }
}

impl Game {
    fn is_feasible(&self) -> bool {
        self.sets.iter().all(Set::is_feasible)
    }

    fn get_minimum_set_power(&self) -> i32 {
        let red = self.sets.iter().map(|set| set.red).max().unwrap();
        let green = self.sets.iter().map(|set| set.green).max().unwrap();
        let blue = self.sets.iter().map(|set| set.blue).max().unwrap();
        red * green * blue
    }
}

struct Set {
    red: i32,
    green: i32,
    blue: i32,
}

lazy_static! {
    static ref SET_RE: Regex = Regex::new(r"(\d+) (\w+)").unwrap();
}

impl From<&str> for Set {
    fn from(value: &str) -> Self {
        let (mut red, mut green, mut blue) = (0, 0, 0);
        for capture in SET_RE.captures_iter(value) {
            let number = aoc2023::parse(capture.get(1).unwrap().as_str());
            match capture.get(2).unwrap().as_str() {
                "red" => red = number,
                "green" => green = number,
                "blue" => blue = number,
                _ => panic!("Unknown text {}", capture.get(0).unwrap().as_str()),
            }
        }
        Self { red, green, blue }
    }
}

impl Set {
    fn is_feasible(&self) -> bool {
        self.red <= 12 && self.green <= 13 && self.blue <= 14
    }
}
