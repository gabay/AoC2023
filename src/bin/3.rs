use lazy_static::lazy_static;
use regex::{Match, Regex};

fn main() {
    let input = aoc2023::readfile("inputs/3");
    println!("part1: {}", part1(&input));
    println!("part2: {}", part2(&input));
}

lazy_static! {
    static ref NUMBERS_RE: Regex = Regex::new(r"\d+").unwrap();
    static ref SYMBOL_RE: Regex = Regex::new(r"[^.0-9]").unwrap();
    static ref GEAR_RE: Regex = Regex::new(r"\*").unwrap();
}

fn part1(input: &str) -> i32 {
    let lines = input.split('\n').collect::<Vec<_>>();
    let symbol_bboxes = lines
        .iter()
        .enumerate()
        .flat_map(|(y, line)| {
            SYMBOL_RE
                .find_iter(line)
                .map(move |match_| BBox::from_y_match(y as i32, match_, 1))
        })
        .collect::<Vec<_>>();

    lines
        .iter()
        .enumerate()
        .map(|(y, line)| {
            NUMBERS_RE
                .find_iter(line)
                .filter(|match_| {
                    BBox::from_y_match(y as i32, *match_, 0).intersect_list(&symbol_bboxes)
                })
                .map(|match_| match_.as_str().parse::<i32>().unwrap())
                .sum::<i32>()
        })
        .sum()
}

fn part2(input: &str) -> i32 {
    let lines = input.split('\n').collect::<Vec<_>>();
    let numbers_with_bboxes = lines
        .iter()
        .enumerate()
        .flat_map(|(y, line)| {
            NUMBERS_RE.find_iter(line).map(move |m| {
                (
                    m.as_str().parse::<i32>().unwrap(),
                    BBox::from_y_match(y as i32, m, 1),
                )
            })
        })
        .collect::<Vec<_>>();

    lines
        .iter()
        .enumerate()
        .flat_map(|(y, line)| {
            GEAR_RE
                .find_iter(line)
                .map(move |match_| BBox::from_y_match(y as i32, match_, 0))
                .map(|gear_bbox| {
                    numbers_with_bboxes
                        .iter()
                        .filter(|(_, bbox)| bbox.intersect(&gear_bbox))
                        .collect::<Vec<_>>()
                })
        })
        .filter(|v| v.len() == 2)
        .map(|v| v[0].0 * v[1].0)
        .sum()
}

struct BBox {
    top: i32,
    left: i32,
    bottom: i32,
    right: i32,
}

impl BBox {
    fn from_y_match(y: i32, match_: Match, pad: i32) -> Self {
        BBox {
            top: y - pad,
            left: match_.start() as i32 - pad,
            bottom: y + pad,
            right: match_.end() as i32 + pad - 1,
        }
    }

    fn intersect(&self, other: &BBox) -> bool {
        self.top <= other.bottom
            && self.left <= other.right
            && self.bottom >= other.top
            && self.right >= other.left
    }

    fn intersect_list(&self, other: &[BBox]) -> bool {
        other.iter().any(|o| self.intersect(o))
    }
}
