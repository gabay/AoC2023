fn main() {
    let input = aoc2023::readfile("inputs/1");
    println!("part1: {}", part1(&input));
    println!("part2: {}", part2(&input));
}

fn part1(input: &str) -> i32 {
    input
        .split('\n')
        .map(|line| line.chars().filter(|c| c.is_ascii_digit()).collect::<Vec<_>>())
        .map(|chars| {
            format!("{}{}", chars.first().unwrap(), chars.last().unwrap())
                .parse::<i32>()
                .unwrap()
        })
        .sum()
}

fn part2(input: &str) -> i32 {
    let numbers = ["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];
    input
        .split('\n')
        .map(|line| {
            numbers
                .iter()
                .enumerate()
                .fold(line.to_string(), |line, (i, number)| {
                    line.replace(*number, format!("{}{}{}", number, i + 1, number).as_str())
                })
        })
        .map(|line| line.chars().filter(|c| c.is_ascii_digit()).collect::<Vec<_>>())
        .map(|chars| {
            format!("{}{}", chars.first().unwrap(), chars.last().unwrap())
                .parse::<i32>()
                .unwrap()
        })
        .sum()
}
