use itertools::Itertools;
use std::cmp;

fn main() {
    let input = aoc2023::readfile("inputs/11");
    println!("part1: {}", part1(&input));
    println!("part2: {}", part2(&input));
}

fn part1(input: &str) -> usize {
    let board = input
        .lines()
        .map(|row| row.chars().collect_vec())
        .collect_vec();
    count_distances_between_all_pairs(&board, 1)
}

fn part2(input: &str) -> usize {
    let board = input
        .lines()
        .map(|row| row.chars().collect_vec())
        .collect_vec();
    count_distances_between_all_pairs(&board, 999999)
}

fn count_distances_between_all_pairs(board: &[Vec<char>], empty_cell_weight: usize) -> usize {
    let empty_rows = get_empty_rows(board);
    let empty_cols = get_empty_cols(board);
    let points = aoc2023::Point::find_multiple_in_board('#', board);
    let mut result = 0;
    for i in 0..points.len() {
        for j in 0..i {
            let (p1, p2) = (points[i], points[j]);
            let x1 = cmp::min(p1.x, p2.x) as usize;
            let x2 = cmp::max(p1.x, p2.x) as usize;
            let y1 = cmp::min(p1.y, p2.y) as usize;
            let y2 = cmp::max(p1.y, p2.y) as usize;
            let d = (x2 - x1) as usize
                + (y2 - y1) as usize
                + (empty_cell_weight * empty_rows.iter().filter(|y| **y > y1 && **y < y2).count())
                + (empty_cell_weight * empty_cols.iter().filter(|x| **x > x1 && **x < x2).count());
            result += d
        }
    }
    result
}

fn get_empty_rows(board: &[Vec<char>]) -> Vec<usize> {
    board
        .iter()
        .enumerate()
        .filter_map(|(y, row)| if row.contains(&'#') { None } else { Some(y) })
        .collect()
}

fn get_empty_cols(board: &[Vec<char>]) -> Vec<usize> {
    (0..board[0].len())
        .filter(|x| board.iter().all(|row| row[*x] != '#'))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....";

    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT), 374);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(INPUT), 82000210);
    }
}
