use std::fs;
use std::path::Path;

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

pub fn to_i32(v: &str) -> i32 {
    v.parse::<i32>().unwrap()
}

pub fn to_u32(v: &str) -> u32 {
    v.parse::<u32>().unwrap()
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
}
