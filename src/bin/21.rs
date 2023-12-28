use std::collections::HashSet;

use aoc2023::Point;
use num::Integer;

fn main() {
    let input = aoc2023::readfile("inputs/21");
    println!("part1: {}", part1(&input, 64));
    println!("part2: {}", part2(&input, 26501365));
}

fn part1(input: &str, n: usize) -> usize {
    let mut board = aoc2023::to_board(input);
    let mut points = HashSet::from_iter(Point::find_multiple_in_board('S', &board));
    replace(&mut board, 'S', '.');
    for _ in 0..n {
        points = step(&board, &points);
    }
    points.len()
}

fn part2(input: &str, n: usize) -> usize {
    // Important observation:
    // first, last, and 'S' rows/columns are all empty.
    // This means it's easy to calculate where and when we get to the edge (and corners) of the map.
    // So the process of solving is
    // 1. count steps from center to edges and corners of the map, and cells occupied
    // 2. count how long does it takes to "fill" a map from each edge/corner, and cells occupied
    // 3. count how many maps are filled in every direction
    // 4. calculate filled maps * occupied, plus the remainder of the steps
    let mut board = aoc2023::to_board(input);
    let bw = board.len();
    let bwi = bw as i32 - 1;
    let points: HashSet<Point> = HashSet::from_iter(Point::find_multiple_in_board('S', &board));
    let p = Point::find_in_board('S', &board);
    replace(&mut board, 'S', '.');

    // Validation of assumptions.
    assert!(board.len() == board[0].len());
    assert!(points.len() == 1);
    assert!(p.x * 2 + 1 == board[0].len() as i32);
    assert!(p.y * 2 + 1 == board.len() as i32);
    assert!(board[0].iter().all(|cell| *cell == '.'));
    assert!(board[board.len() - 1].iter().all(|cell| *cell == '.'));
    assert!(board[p.y as usize].iter().all(|cell| *cell == '.'));
    for y in 0..board.len() {
        assert!(board[y][0] == '.');
        assert!(board[y][board[0].len() - 1] == '.');
        assert!(board[y][p.x as usize] == '.');
    }

    // count points in even and odd steps
    let even_steps = (board.len() + board[0].len()) * 2;
    let even_steps_points = get_locations_after(&board, &points, even_steps);
    let odd_steps_points = get_locations_after(&board, &even_steps_points, 1);

    // count even and odd steps whole maps
    let maps_traversed_radius = (n / bw) - 1;
    // maps at radius X = 4*(x-1) (e.g. 4, 8, 12, 16, 20, 24...)
    // even radii behave like the origin 1 + 8 + 16 + 24 + 32... = 1 + 8(1+2+..+R) = 1 + 4R(R+1)
    let even_radii = maps_traversed_radius / 2;
    let maps_like_origin = 1 + 4 * even_radii * (even_radii + 1);
    // odd radii behave opposite of origin (4 + 12 + 20... = 4*R + 8(1+2+...R-1) = 4RR
    let odd_radii = (maps_traversed_radius + 1) / 2;
    let maps_unlike_origin = 4 * odd_radii * odd_radii;

    let steps_in_full_maps = if n.is_even() {
        maps_like_origin * even_steps_points.len() + maps_unlike_origin * odd_steps_points.len()
    } else {
        maps_like_origin * odd_steps_points.len() + maps_unlike_origin * even_steps_points.len()
    };

    let left_n = (n - bw/2) % bw;
    let mut steps_part = 0;
    steps_part += get_locations_after_p(&board, Point::new(0, p.y), left_n).len();
    steps_part += get_locations_after_p(&board, Point::new(bwi, p.y), left_n).len();
    steps_part += get_locations_after_p(&board, Point::new(p.x, 0), left_n).len();
    steps_part += get_locations_after_p(&board, Point::new(p.x, bwi), left_n).len();
    if left_n * 2 < bw {
        let left_n2 = left_n + bw;
        steps_part += get_locations_after_p(&board, Point::new(0, p.y), left_n2).len();
        steps_part += get_locations_after_p(&board, Point::new(bwi, p.y), left_n2).len();
        steps_part += get_locations_after_p(&board, Point::new(p.x, 0), left_n2).len();
        steps_part += get_locations_after_p(&board, Point::new(p.x, bwi), left_n2).len();
    }
    let diag1_n = if left_n * 2 < bw {left_n - 1 + bw / 2} else {left_n - 1 - bw / 2};
    steps_part += get_locations_after_p(&board, Point::new(0, 0), diag1_n).len() * maps_traversed_radius;
    steps_part += get_locations_after_p(&board, Point::new(bwi, 0), diag1_n).len() * maps_traversed_radius;
    steps_part += get_locations_after_p(&board, Point::new(0, bwi), diag1_n).len() * maps_traversed_radius;
    steps_part += get_locations_after_p(&board, Point::new(bwi, bwi), diag1_n).len() * maps_traversed_radius;

    let diag2_n = diag1_n + bw;
    steps_part += get_locations_after_p(&board, Point::new(0, 0), diag2_n).len() * (maps_traversed_radius - 1);
    steps_part += get_locations_after_p(&board, Point::new(bwi, 0), diag2_n).len() * (maps_traversed_radius - 1);
    steps_part += get_locations_after_p(&board, Point::new(0, bwi), diag2_n).len() * (maps_traversed_radius - 1);
    steps_part += get_locations_after_p(&board, Point::new(bwi, bwi), diag2_n).len() * (maps_traversed_radius - 1);

    steps_in_full_maps + steps_part
}

fn replace(board: &mut [Vec<char>], old: char, new: char) {
    Point::find_in_board(old, board).set(board, new);
}

fn step(board: &[Vec<char>], points: &HashSet<Point>) -> HashSet<Point> {
    points
        .iter()
        .flat_map(|p| vec![p.up(), p.left(), p.down(), p.right()])
        .filter(|p| matches!(p.get(board), Some('.')))
        .collect()
}

fn get_locations_after(
    board: &[Vec<char>],
    points: &HashSet<Point>,
    steps: usize,
) -> HashSet<Point> {
    (0..steps).fold(points.clone(), |p, _| step(board, &p))
}

fn get_locations_after_p(board: &[Vec<char>], point: Point, steps: usize) -> HashSet<Point> {
    get_locations_after(board, &HashSet::from([point]), steps)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "...........
.....###.#.
.###.##..#.
..#.#...#..
....#.#....
.##..S####.
.##..#...#.
.......##..
.##.#.####.
.##..##.##.
...........";

    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT, 6), 16);
    }
}
