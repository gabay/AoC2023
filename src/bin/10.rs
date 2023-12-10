use std::collections::{HashMap, VecDeque, HashSet};

use aoc2023::Point;
use itertools::Itertools;

fn main() {
    let input = aoc2023::readfile("inputs/10");
    println!("part1: {}", part1(&input));
    println!("part2: {}", part2(&input));
}

fn part1(input: &str) -> i32 {
    let cells = input
        .lines()
        .map(|line| line.chars().map(Cell).collect_vec())
        .collect_vec();

    get_loop_greatest_distance(&cells)
}

fn part2(input: &str) -> i32 {
    let cells = input
        .lines()
        .map(|line| line.chars().map(Cell).collect_vec())
        .collect_vec();
    get_enclosed_size(&cells)
}

#[derive(Clone, Copy, PartialEq, Eq)]
struct Cell(char);

impl Cell {
    fn is_connected_up(&self) -> bool {
        "S|LJ".contains(self.0)
    }
    fn is_connected_down(&self) -> bool {
        "S|F7".contains(self.0)
    }
    fn is_connected_left(&self) -> bool {
        "S-J7".contains(self.0)
    }
    fn is_connected_right(&self) -> bool {
        "S-LF".contains(self.0)
    }
}

fn get_loop_greatest_distance(cells: &Vec<Vec<Cell>>) -> i32 {
    *get_loop(cells).values().max().unwrap()
}

fn get_loop(cells: &Vec<Vec<Cell>>) -> HashMap<Point, i32> {
    let start = Point::find_in_board(Cell('S'), &cells);
    let mut distances = HashMap::from([(start, 0)]);
    let mut queue = VecDeque::from([start]);
    while !queue.is_empty() {
        let point = queue.pop_front().unwrap();
        let cell = point.get(cells).unwrap();
        if cell.is_connected_up() {
            let np = point.up();
            if !distances.contains_key(&np) && np.get(cells).is_some_and(|c| c.is_connected_down())
            {
                distances.insert(np, distances[&point] + 1);
                queue.push_back(np);
            }
        }
        if cell.is_connected_left() {
            let np = point.left();
            if !distances.contains_key(&np) && np.get(cells).is_some_and(|c| c.is_connected_right())
            {
                distances.insert(np, distances[&point] + 1);
                queue.push_back(np);
            }
        }
        if cell.is_connected_down() {
            let np = point.down();
            if !distances.contains_key(&np) && np.get(&cells).is_some_and(|c| c.is_connected_up()) {
                distances.insert(np, distances[&point] + 1);
                queue.push_back(np);
            }
        }
        if cell.is_connected_right() {
            let np = point.right();
            if !distances.contains_key(&np) && np.get(&cells).is_some_and(|c| c.is_connected_left())
            {
                distances.insert(np, distances[&point] + 1);
                queue.push_back(np);
            }
        }
    }
    distances
}

fn get_enclosed_size(cells: &Vec<Vec<Cell>>) -> i32 {
    let mut enclosed_size = 0;
    let loop_points = get_loop(cells).into_keys().collect::<HashSet<_>>();
    for (y, line) in cells.iter().enumerate() {
        let mut crossings = vec![];
        for (x, cell) in line.iter().enumerate() {
            let p = Point::new(x as i32, y as i32);
            if loop_points.contains(&p) {
                crossings.push(*cell);
            } else if count_crossings_horizontal(&crossings) % 2 == 1 {
                enclosed_size += 1;
            }
        }
    }
    enclosed_size
}

fn count_crossings_horizontal(line: &Vec<Cell>) -> usize {
    let mut count = 0;
    let mut line_came_from_up = false;
    for cell in line {
        match cell.0 {
            '|' => count += 1,
            'L' => line_came_from_up = true,
            'F' => line_came_from_up = false,
            '7' => if line_came_from_up {count += 1},
            'J' => if !line_came_from_up {count += 1},
            _ => (),
        }
    }
    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_1() {
        let input = "-L|F7
7S-7|
L|7||
-L-J|
L|-JF";
        assert_eq!(part1(input), 4);
    }

    #[test]
    fn test_part1_2() {
        let input = "7-F7-
.FJ|7
SJLL7
|F--J
LJ.LJ";
        assert_eq!(part1(input), 8);
    }

    #[test]
    fn test_part2_1() {
        let input = "...........
.S-------7.
.|F-----7|.
.||.....||.
.||.....||.
.|L-7.F-J|.
.|..|.|..|.
.L--J.L--J.
...........";
        assert_eq!(part2(input), 4);
    }

    #[test]
    fn test_part2_2() {
        let input = "FF7FSF7F7F7F7F7F---7
L|LJ||||||||||||F--J
FL-7LJLJ||||||LJL-77
F--JF--7||LJLJ7F7FJ-
L---JF-JLJ.||-FJLJJ7
|F|F-JF---7F7-L7L|7|
|FFJF7L7F-JF7|JL---7
7-L-JL7||F7|L7F-7F7|
L.L7LFJ|||||FJL7||LJ
L7JLJL-JLJLJL--JLJ.L";
        assert_eq!(part2(input), 10);
    }
}
