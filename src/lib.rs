use std::fmt::Debug;
use std::fs;
use std::path::Path;
use std::str::FromStr;

// Common functionality for AoC

pub fn hello_world() {
    println!("Hello, world!");
}

pub fn readfile(path: &str) -> String {
    fs::read_to_string(Path::new(path))
        .unwrap()
        .trim_matches('\n')
        .to_string()
}

pub fn parse<T: FromStr>(v: &str) -> T
where
    T::Err: Debug,
{
    v.parse::<T>().unwrap()
}

pub fn minmax<T: Ord>(a: T, b: T) -> (T, T) {
    if a <= b {
        (a, b)
    } else {
        (b, a)
    }
}

#[derive(Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl Point {
    pub fn new(x: i32, y: i32) -> Self {
        Point { x, y }
    }
    pub fn up(&self) -> Self {
        Self {
            x: self.x,
            y: self.y - 1,
        }
    }
    pub fn down(&self) -> Self {
        Self {
            x: self.x,
            y: self.y + 1,
        }
    }
    pub fn left(&self) -> Self {
        Self {
            x: self.x - 1,
            y: self.y,
        }
    }
    pub fn right(&self) -> Self {
        Self {
            x: self.x + 1,
            y: self.y,
        }
    }
    pub fn is_in_board<T>(&self, board: &[Vec<T>]) -> bool {
        self.y >= 0
            && (self.y as usize) < board.len()
            && self.x >= 0
            && (self.x as usize) < board[self.y as usize].len()
    }

    pub fn find_in_board<T: Eq>(item: T, board: &[Vec<T>]) -> Self {
        Self::find_multiple_in_board(item, board).pop().unwrap()
    }

    pub fn find_multiple_in_board<T: Eq>(item: T, board: &[Vec<T>]) -> Vec<Self> {
        let mut result = vec![];
        for (y, row) in board.iter().enumerate() {
            for (x, cell) in row.iter().enumerate() {
                if *cell == item {
                    result.push(Point {
                        x: x as i32,
                        y: y as i32,
                    });
                }
            }
        }
        result
    }

    pub fn get<T: Copy>(&self, board: &[Vec<T>]) -> Option<T> {
        if self.is_in_board(board) {
            Some(board[self.y as usize][self.x as usize])
        } else {
            None
        }
    }

    pub fn set<T: Copy>(&self, board: &mut [Vec<T>], value: T) {
        if self.is_in_board(board) {
            board[self.y as usize][self.x as usize] = value;
        }
    }

    pub fn step(&self, d: Direction) -> Self {
        match d {
            Direction::Up => self.up(),
            Direction::Left => self.left(),
            Direction::Down => self.down(),
            Direction::Right => self.right(),
        }
    }

    pub fn steps(&self, d: Direction, n: i32) -> Self {
        match d {
            Direction::Up => Self::new(self.x, self.y - n),
            Direction::Left => Self::new(self.x - n, self.y),
            Direction::Down => Self::new(self.x, self.y + n),
            Direction::Right => Self::new(self.x + n, self.y),
        }
    }
}

pub fn to_board(s: &str) -> Vec<Vec<char>> {
    s.lines().map(|line| line.chars().collect()).collect()
}

pub fn to_num_board(s: &str) -> Vec<Vec<i32>> {
    s.lines()
        .map(|line| line.chars().map(|c| (c as i32) - ('0' as i32)).collect())
        .collect()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Direction {
    Up,
    Left,
    Down,
    Right,
}

impl Direction {
    pub fn rotate_left(&self) -> Self {
        match self {
            Self::Up => Self::Left,
            Self::Left => Self::Down,
            Self::Down => Self::Right,
            Self::Right => Self::Up,
        }
    }

    pub fn rotate_right(&self) -> Self {
        match self {
            Self::Up => Self::Right,
            Self::Left => Self::Up,
            Self::Down => Self::Left,
            Self::Right => Self::Down,
        }
    }

    pub fn opposite(&self) -> Self {
        match self {
            Self::Up => Self::Down,
            Self::Left => Self::Right,
            Self::Down => Self::Up,
            Self::Right => Self::Left,
        }
    }
}

impl From<&str> for Direction {
    fn from(value: &str) -> Self {
        match value.chars().next().unwrap() {
            'u' | 'U' => Direction::Up,
            'd' | 'D' => Direction::Down,
            'l' | 'L' => Direction::Left,
            'r' | 'R' => Direction::Right,
            _ => panic!("Cannot crete direction from '{}'", value),
        }
    }
}
