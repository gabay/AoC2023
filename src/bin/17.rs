use std::collections::{BinaryHeap, HashSet};

use aoc2023::{Direction, Point};

fn main() {
    let input = aoc2023::readfile("inputs/17");
    println!("part1: {}", part1(&input));
    println!("part2: {}", part2(&input));
}

fn part1(input: &str) -> i32 {
    let board = aoc2023::to_num_board(input);
    let start = Point::new(0, 0);
    let end = Point::new(board[0].len() as i32 - 1, board.len() as i32 - 1);
    shortest(&board, start, end, 1, 3)
}

fn part2(input: &str) -> i32 {
    let board = aoc2023::to_num_board(input);
    let start = Point::new(0, 0);
    let end = Point::new(board[0].len() as i32 - 1, board.len() as i32 - 1);
    shortest(&board, start, end, 4, 10)
}

fn shortest(
    board: &[Vec<i32>],
    start: Point,
    end: Point,
    min_consecutive_steps: i32,
    max_consecutive_steps: i32,
) -> i32 {
    let mut heap = BinaryHeap::new();
    let mut seen = HashSet::new();
    heap.push((0, start, Direction::Right, 0));
    heap.push((0, start, Direction::Down, 0));
    while let Some((heat_loss, pos, direction, consecutive_steps_in_direction)) = heap.pop() {
        // If reached goal - return
        if pos == end {
            return -heat_loss;
        }

        // if already seen - skip
        if !seen.insert((pos, direction, consecutive_steps_in_direction)) {
            continue;
        }

        if consecutive_steps_in_direction < max_consecutive_steps {
            let np = pos.step(direction);
            if let Some(n) = np.get(board) {
                heap.push((
                    heat_loss - n,
                    np,
                    direction,
                    consecutive_steps_in_direction + 1,
                ));
            }
        }

        if consecutive_steps_in_direction >= min_consecutive_steps {
            let np = pos.step(direction.rotate_left());
            if let Some(n) = np.get(board) {
                heap.push((heat_loss - n, np, direction.rotate_left(), 1));
            }
            let np = pos.step(direction.rotate_right());
            if let Some(n) = np.get(board) {
                heap.push((heat_loss - n, np, direction.rotate_right(), 1));
            }
        }
    }
    panic!("No path from {:?} to {:?}", start, end);
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "2413432311323
3215453535623
3255245654254
3446585845452
4546657867536
1438598798454
4457876987766
3637877979653
4654967986887
4564679986453
1224686865563
2546548887735
4322674655533";
    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT), 102);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(INPUT), 94);
    }
}
