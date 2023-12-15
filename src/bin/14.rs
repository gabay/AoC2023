use std::collections::HashMap;

use aoc2023::Point;
use itertools::enumerate;

fn main() {
    let input = aoc2023::readfile("inputs/14");
    println!("part1: {}", part1(&input));
    println!("part2: {}", part2(&input));
}

fn part1(input: &str) -> usize {
    let mut board = aoc2023::to_board(input);
    let movable_rocks = aoc2023::Point::find_multiple_in_board('O', &board);
    movable_rocks
        .iter()
        .for_each(|pos| move_up(pos, &mut board));
    get_total_load(&board)
}

fn part2(input: &str) -> usize {
    let mut board = aoc2023::to_board(input);
    let mut seen = HashMap::new();
    let mut steps = 0;
    let goal = 1_000_000_000;
    while !seen.contains_key(&board) {
        seen.insert(board.clone(), steps);
        cycle(&mut board);
        steps += 1;
    }

    let loop_size = steps - seen.get(&board).unwrap();
    let loops = (goal - steps) / loop_size;
    steps += loops * loop_size;

    while steps < goal {
        cycle(&mut board);
        steps += 1;
    }
    get_total_load(&board)
}

fn move_up(point: &Point, board: &mut [Vec<char>]) {
    let np = point.up();
    if np.get(board).is_some_and(|c| c == '.') {
        np.set(board, point.get(board).unwrap());
        point.set(board, '.');
        move_up(&np, board);
    }
}

fn move_left(point: &Point, board: &mut [Vec<char>]) {
    let np = point.left();
    if np.get(board).is_some_and(|c| c == '.') {
        np.set(board, point.get(board).unwrap());
        point.set(board, '.');
        move_left(&np, board);
    }
}

fn move_down(point: &Point, board: &mut [Vec<char>]) {
    let np = point.down();
    if np.get(board).is_some_and(|c| c == '.') {
        np.set(board, point.get(board).unwrap());
        point.set(board, '.');
        move_down(&np, board);
    }
}

fn move_right(point: &Point, board: &mut [Vec<char>]) {
    let np = point.right();
    if np.get(board).is_some_and(|c| c == '.') {
        np.set(board, point.get(board).unwrap());
        point.set(board, '.');
        move_right(&np, board);
    }
}

fn cycle(board: &mut [Vec<char>]) {
    aoc2023::Point::find_multiple_in_board('O', board)
        .iter()
        .for_each(|pos| move_up(pos, board));
    aoc2023::Point::find_multiple_in_board('O', board)
        .iter()
        .for_each(|pos| move_left(pos, board));
    aoc2023::Point::find_multiple_in_board('O', board)
        .iter()
        .rev()
        .for_each(|pos| move_down(pos, board));
    aoc2023::Point::find_multiple_in_board('O', board)
        .iter()
        .rev()
        .for_each(|pos| move_right(pos, board));
}

fn get_total_load(board: &[Vec<char>]) -> usize {
    let board_len = board.len();
    enumerate(board)
        .map(|(i, row)| row.iter().filter(|c| **c == 'O').count() * (board_len - i))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....";

    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT), 136);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(INPUT), 64);
    }
}
