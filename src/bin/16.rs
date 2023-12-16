use std::collections::HashSet;

use aoc2023::{Direction, Point};

fn main() {
    let input = aoc2023::readfile("inputs/16");
    println!("part1: {}", part1(&input));
    println!("part2: {}", part2(&input));
}

fn part1(input: &str) -> usize {
    let board = aoc2023::to_board(input);

    count_energized_tiles(&board, Beam::new(Point { x: 0, y: 0 }, Direction::Right))
}

fn part2(input: &str) -> usize {
    let board = aoc2023::to_board(input);
    let height = board.len() as i32;
    let width = board[0].len() as i32;

    (0..width)
        .map(|x| Beam::new(Point { x, y: 0 }, Direction::Down))
        .chain((0..height).map(|y| Beam::new(Point { x: 0, y }, Direction::Right)))
        .chain((0..width).map(|x| Beam::new(Point { x, y: height - 1 }, Direction::Up)))
        .chain((0..height).map(|y| Beam::new(Point { x: width - 1, y }, Direction::Left)))
        .map(|beam| count_energized_tiles(&board, beam))
        .max()
        .unwrap()
}

fn count_energized_tiles(board: &[Vec<char>], initial_beam: Beam) -> usize {
    let mut beams = vec![initial_beam];
    let mut seen: HashSet<Beam> = HashSet::new();

    while let Some(beam) = beams.pop() {
        if beam.pos.is_in_board(board) && !seen.contains(&beam) {
            beams.append(&mut beam.step(board));
            seen.insert(beam);
        }
    }

    seen.iter()
        .map(|beam| beam.pos)
        .collect::<HashSet<_>>()
        .len()
}

#[derive(Hash, Clone, Copy, PartialEq, Eq)]
struct Beam {
    pos: Point,
    dir: Direction,
}

impl Beam {
    fn new(pos: Point, dir: Direction) -> Self {
        Self { pos, dir }
    }

    fn step(&self, board: &[Vec<char>]) -> Vec<Self> {
        match self.pos.get(board).unwrap() {
            '.' => vec![self._simple_step()],
            '/' => {
                let new_dir = match self.dir {
                    Direction::Up => Direction::Right,
                    Direction::Right => Direction::Up,
                    Direction::Down => Direction::Left,
                    Direction::Left => Direction::Down,
                };
                vec![Beam::new(self.pos.step(new_dir), new_dir)]
            }
            '\\' => {
                let new_dir = match self.dir {
                    Direction::Up => Direction::Left,
                    Direction::Left => Direction::Up,
                    Direction::Down => Direction::Right,
                    Direction::Right => Direction::Down,
                };
                vec![Beam::new(self.pos.step(new_dir), new_dir)]
            }
            '|' => match self.dir {
                Direction::Up | Direction::Down => vec![self._simple_step()],
                Direction::Left | Direction::Right => {
                    vec![
                        Beam::new(self.pos.up(), Direction::Up),
                        Beam::new(self.pos.down(), Direction::Down),
                    ]
                }
            },
            '-' => match self.dir {
                Direction::Left | Direction::Right => vec![self._simple_step()],
                Direction::Up | Direction::Down => vec![
                    Beam::new(self.pos.left(), Direction::Left),
                    Beam::new(self.pos.right(), Direction::Right),
                ],
            },
            c => panic!("Inavlid character '{}'", c),
        }
    }

    fn _simple_step(&self) -> Self {
        Self::new(self.pos.step(self.dir), self.dir)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r".|...\....
|.-.\.....
.....|-...
........|.
..........
.........\
..../.\\..
.-.-/..|..
.|....-|.\
..//.|....";

    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT), 46);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(INPUT), 51);
    }
}
