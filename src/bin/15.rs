use itertools::enumerate;

fn main() {
    let input = aoc2023::readfile("inputs/15");
    println!("part1: {}", part1(&input));
    println!("part2: {}", part2(&input));
}

fn part1(input: &str) -> usize {
    input.split(',').map(hash).sum()
}

fn part2(input: &str) -> usize {
    let mut boxes = Boxes::new();
    input.split(',').for_each(|line| boxes.apply(line));
    boxes.get_power()
}

fn hash(s: &str) -> usize {
    s.chars()
        .fold(0, |acc, c| ((acc + (c as usize)) * 17) % 256)
}

struct Boxes<'a>(Vec<Box<'a>>);

impl<'a> Boxes<'a> {
    fn new() -> Self {
        Self(vec![Box::new(); 256])
    }

    fn apply(&mut self, cmd: &'a str) {
        if cmd.ends_with('-') {
            let label = &cmd[0..cmd.len() - 1];
            self.0[hash(label)].drop(label);
        } else {
            let (label, focus) = cmd.split_once('=').unwrap();
            let focus = aoc2023::parse(focus);
            self.0[hash(label)].set(label, focus);
        }
    }

    fn get_power(&self) -> usize {
        enumerate(&self.0)
            .map(|(i, bx)| (i + 1) * bx.get_power())
            .sum()
    }
}

#[derive(Clone, Default)]
struct Box<'a>(Vec<Lens<'a>>);

impl<'a> Box<'a> {
    fn new() -> Self {
        Self(vec![])
    }

    fn set(&mut self, label: &'a str, focus: usize) {
        match self.0.iter_mut().find(|lens| lens.label == label) {
            Some(lens) => lens.focus = focus,
            None => self.0.push(Lens::new(label, focus)),
        }
    }

    fn drop(&mut self, label: &str) {
        for index in 0..self.0.len() {
            if self.0[index].label == label {
                self.0.remove(index);
                break;
            }
        }
    }

    fn get_power(&self) -> usize {
        enumerate(&self.0)
            .map(|(i, lens)| (i + 1) * lens.focus)
            .sum()
    }
}

#[derive(Clone, Copy)]
struct Lens<'a> {
    pub label: &'a str,
    pub focus: usize,
}

impl<'a> Lens<'a> {
    fn new(label: &'a str, focus: usize) -> Self {
        Self { label, focus }
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";

    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT), 1320);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(INPUT), 145);
    }
}
