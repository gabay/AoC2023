use std::collections::{HashMap, HashSet};

use aoc2023::Point;
use itertools::Itertools;

fn main() {
    let input = aoc2023::readfile("inputs/23");
    println!("part1: {}", part1(&input));
    println!("part2: {}", part2(&input));
}

fn part1(input: &str) -> usize {
    let board = aoc2023::to_board(input);
    let mut points = Point::find_multiple_in_board('.', &board);
    let mut up = Point::find_multiple_in_board('^', &board);
    let mut left = Point::find_multiple_in_board('<', &board);
    let mut down = Point::find_multiple_in_board('v', &board);
    let mut right = Point::find_multiple_in_board('>', &board);
    let (start, end) = get_start_end(&points);

    let valid = [
        points.clone(),
        up.clone(),
        left.clone(),
        down.clone(),
        right.clone(),
    ]
    .iter()
    .flatten()
    .cloned()
    .collect::<HashSet<_>>();

    let mut adj = HashMap::new();
    up.drain(..).for_each(|p| {
        adj.insert(p, vec![p.up()]);
    });
    left.drain(..).for_each(|p| {
        adj.insert(p, vec![p.left()]);
    });
    down.drain(..).for_each(|p| {
        adj.insert(p, vec![p.down()]);
    });
    right.drain(..).for_each(|p| {
        adj.insert(p, vec![p.right()]);
    });
    points.drain(..).for_each(|p| {
        insert_point(&mut adj, p, &valid);
    });

    get_longest_path(&adj, start, end, &mut HashSet::new())
}

fn part2(input: &str) -> usize {
    let inp = input.replace(['^', '<', '>', 'v'], ".");
    let board = aoc2023::to_board(&inp);
    let mut points = Point::find_multiple_in_board('.', &board);
    let (start, end) = get_start_end(&points);

    let valid = points.iter().cloned().collect::<HashSet<_>>();

    let mut adj = HashMap::new();
    points.drain(..).for_each(|p| {
        insert_point(&mut adj, p, &valid);
    });

    get_longest_path(&adj, start, end, &mut HashSet::new())
}

fn get_start_end(points: &[Point]) -> (Point, Point) {
    let start = points.first().unwrap().to_owned();
    let end = points.last().unwrap().to_owned();
    (start, end)
}

fn insert_point(adj: &mut HashMap<Point, Vec<Point>>, p: Point, valid: &HashSet<Point>) {
    adj.insert(
        p,
        [p.up(), p.left(), p.down(), p.right()]
            .iter()
            .filter(|pp| valid.contains(*pp))
            .cloned()
            .collect_vec(),
    );
}

fn get_longest_path(
    adj: &HashMap<Point, Vec<Point>>,
    start: Point,
    end: Point,
    seen: &mut HashSet<Point>,
) -> usize {
    if start == end {
        seen.len()
    } else if seen.insert(start) {
        let result = adj
            .get(&start)
            .unwrap()
            .iter()
            .map(|neighbor| get_longest_path(adj, *neighbor, end, seen))
            .max()
            .unwrap_or(0);
        seen.remove(&start);
        result
    } else {
        0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "#.#####################
#.......#########...###
#######.#########.#.###
###.....#.>.>.###.#.###
###v#####.#v#.###.#.###
###.>...#.#.#.....#...#
###v###.#.#.#########.#
###...#.#.#.......#...#
#####.#.#.#######.#.###
#.....#.#.#.......#...#
#.#####.#.#.#########v#
#.#...#...#...###...>.#
#.#.#v#######v###.###v#
#...#.>.#...>.>.#.###.#
#####v#.#.###v#.#.###.#
#.....#...#...#.#.#...#
#.#########.###.#.#.###
#...###...#...#...#.###
###.###.#.###v#####v###
#...#...#.#.>.>.#.>.###
#.###.###.#.###.#.#v###
#.....###...###...#...#
#####################.#";

    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT), 94);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(INPUT), 154);
    }
}
