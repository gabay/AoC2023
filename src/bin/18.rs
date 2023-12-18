use aoc2023::{parse, Direction, Point};
use itertools::Itertools;

use std::collections::HashSet;

fn main() {
    let input = aoc2023::readfile("inputs/18");
    println!("part1: {}", part1(&input));
    println!("part2: {}", part2(&input));
}

fn part1(input: &str) -> usize {
    let moves = input
        .lines()
        .map(|l| {
            let (dir, len, _) = l.split(' ').collect_tuple().unwrap();
            (Direction::from(dir), parse::<usize>(len))
        })
        .collect_vec();
    let points = get_points(&moves);
    let area = get_area(&points);
    points.len() + area
}
fn part2(input: &str) -> i64 {
    let moves = input
        .lines()
        .map(|l| {
            let (_, _, rgb) = l.split(' ').collect_tuple().unwrap();
            (rgb_to_direction(rgb), rgb_to_length(rgb))
        })
        .collect_vec();
    let corners = get_corners(&moves);
    let mut active_xs = HashSet::new();
    let mut result = 0;
    let mut prev_y = corners.first().unwrap().p.y;
    for chunk in &corners.iter().chunks(2) {
        let (c1, c2) = chunk.collect_tuple().unwrap();
        assert_eq!(c1.p.y, c2.p.y);
        assert!(c1.p.x < c2.p.x);

        if c1.p.y >= prev_y {
            let height = (c1.p.y - prev_y + 1) as i64;
            let width = active_xs.len() as i64;
            result += height * width;
            prev_y = c1.p.y + 1;
        }

        let was_inside = active_xs.contains(&(c1.p.x + 1));

        // flip columns in the middle.
        for x in c1.p.x + 1..c2.p.x {
            if active_xs.insert(x) {
                result += 1;
            } else {
                active_xs.remove(&x);
            }
        }

        match c1.ct {
            CornerType::DownRight => {
                if active_xs.insert(c1.p.x) {
                    result += 1;
                }
            }
            CornerType::UpRight => {
                if was_inside {
                    active_xs.remove(&c1.p.x);
                }
            }
            _ => panic!(
                "c1 type should be UpRight or DownRight: c1={:?} c2={:?}",
                c1, c2
            ),
        }

        match c2.ct {
            CornerType::DownLeft => {
                if active_xs.insert(c2.p.x) {
                    result += 1;
                }
            }
            CornerType::UpLeft => {
                if was_inside {
                    active_xs.remove(&c2.p.x);
                }
            }
            _ => panic!(
                "c2 type should be UpLeft or DownLeft: c1={:?} c2={:?}",
                c1, c2
            ),
        }
    }
    assert_eq!(active_xs.len(), 0);

    result
}

fn get_points(moves: &[(Direction, usize)]) -> HashSet<Point> {
    let mut p = Point::new(0, 0);
    let mut points = HashSet::new();
    points.insert(p);
    for (direction, length) in moves {
        for _ in 0..*length {
            p = p.step(*direction);
            points.insert(p);
        }
    }
    assert_eq!(p, Point::new(0, 0));
    points
}

fn get_area(points: &HashSet<Point>) -> usize {
    // assume center is inside
    let x0 = {
        let xs = points.iter().map(|p| p.x);
        (xs.clone().min().unwrap() + xs.max().unwrap()) / 2
    };
    let y0 = {
        let ys = points.iter().map(|p| p.y);
        (ys.clone().min().unwrap() + ys.max().unwrap()) / 2
    };
    let mut area_points = HashSet::new();
    let mut queue = vec![Point::new(x0, y0)];
    while let Some(p) = queue.pop() {
        if points.contains(&p) {
            continue;
        }
        if area_points.insert(p) {
            for np in [p.up(), p.down(), p.left(), p.right()] {
                queue.push(np);
            }
        }
    }
    area_points.len()
}

fn rgb_to_direction(rgb: &str) -> Direction {
    match rgb.chars().nth(7).unwrap() {
        '0' => Direction::Right,
        '1' => Direction::Down,
        '2' => Direction::Left,
        '3' => Direction::Up,
        _ => panic!("Invalid rgb value '{}'", rgb),
    }
}

fn rgb_to_length(rgb: &str) -> usize {
    usize::from_str_radix(&rgb[2..7], 16).unwrap()
}

fn get_corners(moves: &[(Direction, usize)]) -> Vec<Corner> {
    let mut p = Point::new(0, 0);
    let mut prev_direction = moves.last().unwrap().0.opposite();
    let mut corners = vec![];
    for (direction, length) in moves {
        corners.push(Corner::new(p, CornerType::from(prev_direction, *direction)));
        p = p.steps(*direction, *length as i32);
        prev_direction = direction.opposite();
    }
    assert_eq!(p, Point::new(0, 0));
    corners.sort_by_key(|c| (c.p.y, c.p.x, c.ct));
    corners
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy, Debug)]
enum CornerType {
    DownLeft,
    DownRight,
    UpLeft,
    UpRight,
}

impl CornerType {
    fn from(d1: Direction, d2: Direction) -> Self {
        match (d1, d2) {
            (Direction::Down, Direction::Left) | (Direction::Left, Direction::Down) => {
                Self::DownLeft
            }
            (Direction::Down, Direction::Right) | (Direction::Right, Direction::Down) => {
                Self::DownRight
            }
            (Direction::Up, Direction::Left) | (Direction::Left, Direction::Up) => Self::UpLeft,
            (Direction::Up, Direction::Right) | (Direction::Right, Direction::Up) => Self::UpRight,
            _ => panic!("Invalid corner {:?} {:?}", d1, d2),
        }
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
struct Corner {
    p: Point,
    ct: CornerType,
}

impl Corner {
    fn new(p: Point, ct: CornerType) -> Self {
        Self { p, ct }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "R 6 (#70c710)
D 5 (#0dc571)
L 2 (#5713f0)
D 2 (#d2c081)
R 2 (#59c680)
D 2 (#411b91)
L 5 (#8ceee2)
U 2 (#caa173)
L 1 (#1b58a2)
U 2 (#caa171)
R 2 (#7807d2)
U 3 (#a77fa3)
L 2 (#015232)
U 2 (#7a21e3)";

    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT), 62);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(INPUT), 952408144115);
    }
}
