use itertools::enumerate;

fn main() {
    let input = aoc2023::readfile("inputs/12");
    println!("part1: {}", part1(&input));
    println!("part2: {}", part2(&input));
}

fn part1(input: &str) -> usize {
    input
        .lines()
        .map(Row::from)
        .map(Row::count_arrangements)
        .sum()
}

fn part2(input: &str) -> usize {
    input
        .lines()
        .map(unfold)
        .map(Row::from)
        .map(Row::count_arrangements)
        .sum()
}

enum Spring {
    Operational,
    Damaged,
    Unknown,
}

impl From<char> for Spring {
    fn from(value: char) -> Self {
        match value {
            '.' => Self::Operational,
            '#' => Self::Damaged,
            '?' => Self::Unknown,
            _ => panic!(),
        }
    }
}

impl Spring {
    fn can_be_damaged(&self) -> bool {
        !matches!(self, Self::Operational)
    }
    fn can_be_operational(&self) -> bool {
        !matches!(self, Self::Damaged)
    }
}

struct Row {
    springs: Vec<Spring>,
    summary: Vec<i32>,
}

impl From<&str> for Row {
    fn from(value: &str) -> Self {
        let (springs_str, summary_str) = value.split_once(' ').unwrap();
        let springs = springs_str.chars().map(Spring::from).collect();
        let summary = summary_str.split(',').map(aoc2023::to_i32).collect();
        Self { springs, summary }
    }
}

impl From<String> for Row {
    fn from(value: String) -> Self {
        Row::from(value.as_str())
    }
}

impl Row {
    fn count_arrangements(self) -> usize {
        let mut arrangements = vec![vec![0; self.springs.len() + 1]; self.summary.len() + 1];

        arrangements[0][0] = 1;
        for (i, spring) in enumerate(&self.springs) {
            if let Spring::Damaged = spring {
                break;
            }
            arrangements[0][i + 1] = 1;
        }

        for (summary_index, item_length) in enumerate(&self.summary) {
            let summary_length = *item_length as usize;
            for (spring_index, spring) in enumerate(&self.springs) {
                let arrangements_with_spring_operational = if spring.can_be_operational() {
                    arrangements[summary_index + 1][spring_index]
                } else {
                    0
                };
                let arrangements_with_spring_damaged_at_start = if summary_index == 0
                    && spring_index + 1 == summary_length
                    && self.springs[0..=spring_index]
                        .iter()
                        .all(Spring::can_be_damaged)
                {
                    arrangements[summary_index][0]
                } else {
                    0
                };
                let arrangements_with_spring_damaged = if spring_index + 1 > summary_length
                    && self.springs[spring_index - summary_length].can_be_operational()
                    && self.springs[spring_index + 1 - summary_length..=spring_index]
                        .iter()
                        .all(Spring::can_be_damaged)
                {
                    arrangements[summary_index][spring_index - summary_length]
                } else {
                    0
                };
                arrangements[summary_index + 1][spring_index + 1] =
                    arrangements_with_spring_operational
                        + arrangements_with_spring_damaged_at_start
                        + arrangements_with_spring_damaged;
            }
        }
        // arrangements.iter().for_each(|row| eprintln!("{:?}", row));
        // eprintln!("{}", arrangements[self.summary.len()][self.springs.len()]);
        arrangements[self.summary.len()][self.springs.len()]
    }
}

fn unfold(s: &str) -> String {
    let (springs, summary) = s.split_once(' ').unwrap();
    format!(
        "{}?{}?{}?{}?{} {},{},{},{},{}",
        springs, springs, springs, springs, springs, summary, summary, summary, summary, summary
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1";

    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT), 21);
    }

    #[test]
    fn test_part1_2() {
        assert_eq!(part1("#?#????????.?#. 4,1,2,1"), 6);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(INPUT), 525152);
    }
}
