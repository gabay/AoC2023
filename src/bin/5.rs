use itertools::Itertools;
use lazy_static::lazy_static;
use regex::Regex;
use std::cmp;
use std::collections::HashMap;

lazy_static! {
    static ref TYPES_RE: Regex = Regex::new(r"(\w+)-to-(\w+) map").unwrap();
}

fn main() {
    let input = aoc2023::readfile("inputs/5");
    println!("part1: {}", part1(&input));
    println!("part2: {}", part2(&input));
}

fn part1(input: &str) -> u64 {
    let mapper = get_mapper(input);
    get_seeds(input)
        .iter()
        .map(|seed| mapper.map_to(seed, "location"))
        .min()
        .unwrap()
}

fn part2(input: &str) -> u64 {
    let mapper = get_mapper(input);
    let ranges = get_seeds2(input);
    *mapper
        .map_ranges_to(&ranges, "location")
        .iter()
        .min()
        .unwrap()
}

fn get_seeds(input: &str) -> Vec<Item> {
    input
        .lines()
        .next()
        .unwrap()
        .split(' ')
        .skip(1)
        .map(|n| Item {
            type_: "seed",
            value: n.parse::<u64>().unwrap(),
        })
        .collect()
}
fn get_seeds2(input: &str) -> Vec<Range> {
    input
        .lines()
        .next()
        .unwrap()
        .split(' ')
        .skip(1)
        .map(|n| n.parse::<u64>().unwrap())
        .chunks(2)
        .into_iter()
        .map(|mut chunk| Range {
            type_: "seed",
            start: chunk.next().unwrap(),
            length: chunk.next().unwrap(),
        })
        .collect()
}

fn get_mapper(input: &str) -> Mapper {
    let mut source = "";
    let mut destination = "";
    let mut mapper = Mapper::new();
    for line in input.lines().skip(1).filter(|line| !line.is_empty()) {
        if let Some(capture) = TYPES_RE.captures(line) {
            source = capture.get(1).unwrap().as_str();
            destination = capture.get(2).unwrap().as_str();
        } else {
            mapper.add(source, destination, line.into());
        }
    }
    mapper
}

struct Item<'a> {
    type_: &'a str,
    value: u64,
}

#[derive(Debug)]
struct Range<'a> {
    type_: &'a str,
    start: u64,
    length: u64,
}

#[derive(Debug)]
struct Mapping {
    destination_start: u64,
    source_start: u64,
    length: u64,
}

impl From<&str> for Mapping {
    fn from(value: &str) -> Self {
        println!("{}", value);
        let p = value
            .split(' ')
            .map(|part| part.parse::<u64>().unwrap())
            .collect::<Vec<_>>();
        Self {
            destination_start: p[0],
            source_start: p[1],
            length: p[2],
        }
    }
}

impl Mapping {
    fn trymap(&self, value: u64) -> Option<u64> {
        if value >= self.source_start && value < self.source_start + self.length {
            Some(value - self.source_start + self.destination_start)
        } else {
            None
        }
    }
    fn get_unmapped<'a>(&self, ranges: &Vec<Range<'a>>) -> Vec<Range<'a>> {
        let mut unmapped = vec![];
        for range in ranges {
            if range.start < self.source_start {
                let end = cmp::min(range.start + range.length, self.source_start);
                let length = end - range.start;
                unmapped.push(Range {
                    type_: range.type_,
                    start: range.start,
                    length,
                });
            }
            println!("{:?} {:?}", range, self);
            if range.start + range.length > self.source_start + self.length {
                let start = cmp::max(range.start, self.source_start + self.length);
                let length = range.start + range.length - start;
                unmapped.push(Range {
                    type_: range.type_,
                    start,
                    length,
                });
            }
        }
        unmapped
    }

    fn trymap_range<'a>(&self, range: &Range, dest_type: &'a str) -> Option<Range<'a>> {
        println!("{:?} {:?}", range, self);
        if range.start + range.length >= self.source_start
            && range.start < self.source_start + self.length
        {
            let start = cmp::max(self.source_start, range.start);
            let end = cmp::min(self.source_start + self.length, range.start + range.length);
            let length = end - start;
            let mapped_start = start - self.source_start + self.destination_start;
            Some(Range {
                type_: dest_type,
                start: mapped_start,
                length,
            })
        } else {
            None
        }
    }
}

struct Mapper {
    mappings: HashMap<String, (String, Vec<Mapping>)>,
}

impl Mapper {
    fn new() -> Mapper {
        Mapper {
            mappings: HashMap::new(),
        }
    }

    fn add(&mut self, source: &str, destination: &str, mapping: Mapping) {
        self.mappings
            .entry(source.to_string())
            .or_insert((destination.to_string(), vec![]))
            .1
            .push(mapping);
    }

    fn map(&self, item: &Item) -> Item {
        let (type_, mappings) = self.mappings.get(item.type_).unwrap();
        let value = mappings
            .iter()
            .filter_map(|mapping| mapping.trymap(item.value))
            .next()
            .unwrap_or(item.value);
        Item { type_, value }
    }

    fn map_to(&self, item: &Item, type_: &str) -> u64 {
        if item.type_ == type_ {
            item.value
        } else {
            self.map_to(&self.map(item), type_)
        }
    }

    fn map_ranges(&self, ranges: &Vec<Range>) -> Vec<Range> {
        let (type_, mappings) = self.mappings.get(ranges[0].type_).unwrap();
        let mut result = vec![];
        for range in ranges {
            result.extend(
                mappings
                    .iter()
                    .filter_map(|mapping| mapping.trymap_range(range, type_)),
            );
        }
        let mut unmapped = ranges;
        let mut new_unmapped = vec![];
        for mapping in mappings {
            new_unmapped = mapping.get_unmapped(unmapped);
            unmapped = &new_unmapped;
        }
        result.extend(new_unmapped.iter().map(|range| Range {
            type_,
            start: range.start,
            length: range.length,
        }));
        result
    }

    fn map_ranges_to(&self, ranges: &Vec<Range>, type_: &str) -> Vec<u64> {
        println!("{:?}", ranges);
        if ranges[0].type_ == type_ {
            ranges.iter().map(|r| r.start).collect()
        } else {
            self.map_ranges_to(&self.map_ranges(ranges), type_)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const TEST_INPUT: &str = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT), 35);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST_INPUT), 46);
    }
}
