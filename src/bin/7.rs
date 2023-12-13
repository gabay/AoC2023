use itertools::Itertools;
use lazy_static::lazy_static;
use regex::Regex;
use std::hash::Hash;
use std::{cmp::Ordering, collections::HashMap};

lazy_static! {
    static ref NUMBER_RE: Regex = Regex::new(r"\d+").unwrap();
}

fn main() {
    let input = aoc2023::readfile("inputs/7");
    println!("part1: {}", part1(&input));
    println!("part2: {}", part2(&input));
}

fn part1(input: &str) -> u32 {
    input
        .lines()
        .map(Hand::fromline)
        .sorted()
        .enumerate()
        .map(|(i, hand)| (i as u32 + 1) * hand.bid)
        .sum()
}

fn part2(input: &str) -> u32 {
    input
        .lines()
        .map(Hand2::fromline)
        .sorted()
        .enumerate()
        .map(|(i, hand)| (i as u32 + 1) * hand.bid)
        .sum()
}

trait FromChar {
    #[must_use]
    fn fromchar(c: char) -> Self;
}

#[derive(PartialOrd, Ord, PartialEq, Eq, Hash, Clone, Copy, Debug)]
enum Card {
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}

impl FromChar for Card {
    fn fromchar(c: char) -> Self {
        match c {
            'A' => Card::Ace,
            'K' => Card::King,
            'Q' => Card::Queen,
            'J' => Card::Jack,
            'T' => Card::Ten,
            '9' => Card::Nine,
            '8' => Card::Eight,
            '7' => Card::Seven,
            '6' => Card::Six,
            '5' => Card::Five,
            '4' => Card::Four,
            '3' => Card::Three,
            '2' => Card::Two,
            _ => panic!("Invalid card {}", c),
        }
    }
}

struct Hand {
    cards: Vec<Card>,
    bid: u32,
}

impl Hand {
    fn fromline(line: &str) -> Self {
        let Some((cards, bid)) = line.split(' ').collect_tuple() else {panic!()};
        Self {
            cards: cards.chars().map(Card::fromchar).collect(),
            bid: bid.parse::<u32>().unwrap(),
        }
    }

    fn hand_type(&self) -> HandType {
        let mut counter: HashMap<Card, i32> = HashMap::new();
        for card in self.cards.iter() {
            counter.entry(*card).and_modify(|i| *i += 1).or_insert(1);
        }
        let counts = counter.values().sorted().rev().collect::<Vec<_>>();
        match *counts[0] {
            5 => HandType::FiveOfAKind,
            4 => HandType::FourOfAKind,
            3 if *counts[1] == 2 => HandType::FullHouse,
            3 => HandType::ThreeOfAKind,
            2 if *counts[1] == 2 => HandType::TwoPair,
            2 => HandType::Pair,
            _ => HandType::HighCard,
        }
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.hand_type().cmp(&other.hand_type()) {
            Ordering::Equal => self.cards.cmp(&other.cards),
            a => a,
        }
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.hand_type() == other.hand_type()
    }
}

impl Eq for Hand {}

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
enum HandType {
    HighCard,
    Pair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}



#[derive(PartialOrd, Ord, PartialEq, Eq, Hash, Clone, Copy, Debug)]
enum Card2 {
    Jocker,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Queen,
    King,
    Ace,
}

impl FromChar for Card2 {
    fn fromchar(c: char) -> Self {
        match c {
            'A' => Card2::Ace,
            'K' => Card2::King,
            'Q' => Card2::Queen,
            'J' => Card2::Jocker,
            'T' => Card2::Ten,
            '9' => Card2::Nine,
            '8' => Card2::Eight,
            '7' => Card2::Seven,
            '6' => Card2::Six,
            '5' => Card2::Five,
            '4' => Card2::Four,
            '3' => Card2::Three,
            '2' => Card2::Two,
            _ => panic!("Invalid card {}", c),
        }
    }
}



struct Hand2 {
    cards: Vec<Card2>,
    bid: u32,
}

impl Hand2 {
    fn fromline(line: &str) -> Self {
        let Some((cards, bid)) = line.split(' ').collect_tuple() else {panic!()};
        Self {
            cards: cards.chars().map(Card2::fromchar).collect(),
            bid: bid.parse::<u32>().unwrap(),
        }
    }

    fn hand_type(&self) -> HandType {
        let mut counter: HashMap<Card2, i32> = HashMap::new();
        for card in self.cards.iter() {
            counter.entry(*card).and_modify(|i| *i += 1).or_insert(1);
        }
        let jockers = counter.remove(&Card2::Jocker).unwrap_or(0);
        // 5 jokers
        if counter.is_empty() {
            return HandType::FiveOfAKind;
        }
        let counts = counter.values().sorted().rev().collect::<Vec<_>>();
        match *counts[0] + jockers {
            5 => HandType::FiveOfAKind,
            4 => HandType::FourOfAKind,
            3 if *counts[1] == 2 => HandType::FullHouse,
            3 => HandType::ThreeOfAKind,
            2 if *counts[1] == 2 => HandType::TwoPair,
            2 => HandType::Pair,
            _ => HandType::HighCard,
        }
    }
}

impl Ord for Hand2 {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.hand_type().cmp(&other.hand_type()) {
            Ordering::Equal => self.cards.cmp(&other.cards),
            a => a,
        }
    }
}

impl PartialOrd for Hand2 {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Hand2 {
    fn eq(&self, other: &Self) -> bool {
        self.hand_type() == other.hand_type()
    }
}

impl Eq for Hand2 {}
#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";

    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT), 6440);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(INPUT), 5905);
    }
}
