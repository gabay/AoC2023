use std::collections::{HashMap, HashSet, VecDeque};

use num::integer::lcm;

fn main() {
    let input = aoc2023::readfile("inputs/20");
    println!("part1: {}", part1(&input));
    println!("part2: {}", part2(&input));
}

fn part1(input: &str) -> usize {
    let mut modules = get_modules(input);
    (0..1000)
        .map(|_| process_signal(&mut modules))
        .reduce(|(l1, h1), (l2, h2)| (l1 + l2, h1 + h2))
        .map(|(l, h)| l * h)
        .unwrap()
}

fn part2(input: &str) -> usize {
    let modules = get_modules(input);
    let mut goals = vec![Goal::new("rx".to_string(), Signal::Low)];
    // find the conjunction inputs leading to the goal
    for _ in 0..2 {
        goals = goals
            .drain(..)
            .flat_map(|g| g.expand_conjunction_inputs(&modules))
            .collect();
    }
    println!("{:?}", goals);

    // find period for each
    let mut periods = goals
        .iter()
        .map(|g| g.find_period(get_modules(input)))
        .collect::<Vec<_>>();
    println!("{:?}", periods);

    // return LCM of all periods
    periods.drain(..).reduce(lcm).unwrap()
}

fn get_modules(input: &str) -> HashMap<&str, Module> {
    let mut modules = HashMap::new();
    input.lines().for_each(|line| {
        let (full_name, outputs_str) = line.split_once(" -> ").unwrap();
        let name = if full_name == "broadcaster" {
            full_name
        } else {
            &full_name[1..]
        };
        let module = modules.entry(name).or_insert(Module::new(name));
        if full_name.starts_with('%') {
            module.t = ModuleType::Flipflop;
        }
        if full_name.starts_with('&') {
            module.t = ModuleType::Conjunction;
        }

        for output in outputs_str.split(", ") {
            module.add_output(output);
        }
        for output in outputs_str.split(", ") {
            modules
                .entry(output)
                .or_insert(Module::new(output))
                .add_input(name);
        }
    });
    modules
}

fn process_signal(modules: &mut HashMap<&str, Module>) -> (usize, usize) {
    let mut low = 0;
    let mut high = 0;
    let mut queue = VecDeque::new();
    queue.push_back(("button".to_string(), "broadcaster".to_string(), Signal::Low));
    while let Some((src, dst, typ)) = queue.pop_front() {
        match typ {
            Signal::Low => low += 1,
            Signal::High => high += 1,
        }

        if let Some(module) = modules.get_mut(dst.as_str()) {
            queue.extend(module.process(&src, typ));
        }
    }
    (low, high)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Signal {
    Low,
    High,
}

impl Signal {
    fn flip(&self) -> Self {
        match self {
            Signal::Low => Signal::High,
            Signal::High => Signal::Low,
        }
    }
}

#[derive(Debug)]
enum ModuleType {
    Broadcast,
    Flipflop,
    Conjunction,
}

struct Module {
    name: String,
    t: ModuleType,
    inputs: Vec<String>,
    outputs: Vec<String>,
    is_on: bool,
    input_high_signals: HashSet<String>,
}

impl Module {
    fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            t: ModuleType::Broadcast,
            inputs: vec![],
            outputs: vec![],
            is_on: false,
            input_high_signals: HashSet::new(),
        }
    }

    fn add_input(&mut self, input: &str) {
        self.inputs.push(input.to_string())
    }

    fn add_output(&mut self, output: &str) {
        self.outputs.push(output.to_string())
    }

    fn process(&mut self, src: &str, typ: Signal) -> Vec<(String, String, Signal)> {
        match self.t {
            ModuleType::Broadcast => self
                .outputs
                .iter()
                .map(|out| (self.name.to_owned(), out.to_owned(), typ))
                .collect(),
            ModuleType::Flipflop => match typ {
                Signal::High => vec![],
                Signal::Low => {
                    self.is_on = !self.is_on;
                    let out_type = if self.is_on {
                        Signal::High
                    } else {
                        Signal::Low
                    };
                    self.outputs
                        .iter()
                        .map(|out| (self.name.to_owned(), out.to_owned(), out_type))
                        .collect()
                }
            },
            ModuleType::Conjunction => {
                match typ {
                    Signal::Low => self.input_high_signals.remove(src),
                    Signal::High => self.input_high_signals.insert(src.to_string()),
                };
                let out_type = if self.inputs.len() == self.input_high_signals.len() {
                    Signal::Low
                } else {
                    Signal::High
                };
                self.outputs
                    .iter()
                    .map(|out| (self.name.to_owned(), out.to_owned(), out_type))
                    .collect()
            }
        }
    }
}

#[derive(Debug)]
struct Goal {
    name: String,
    signal: Signal,
}

impl Goal {
    fn new(name: String, signal: Signal) -> Self {
        Self { name, signal }
    }

    fn expand_conjunction_inputs(self, modules: &HashMap<&str, Module>) -> Vec<Self> {
        let m = &modules[self.name.as_str()];
        if m.inputs
            .iter()
            .all(|i| matches!(modules[i.as_str()].t, ModuleType::Conjunction))
        {
            m.inputs
                .iter()
                .map(|i| Goal::new(i.to_owned(), self.signal.flip()))
                .collect()
        } else {
            vec![self]
        }
    }

    fn find_period(&self, mut modules: HashMap<&str, Module>) -> usize {
        let mut queue = VecDeque::new();
        for step in 1.. {
            queue.push_back(("button".to_string(), "broadcaster".to_string(), Signal::Low));
            while let Some((src, dst, typ)) = queue.pop_front() {
                if (self.name.as_str(), self.signal) == (dst.as_str(), typ) {
                    return step;
                }

                if let Some(module) = modules.get_mut(dst.as_str()) {
                    queue.extend(module.process(&src, typ));
                }
            }
        }
        panic!();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "broadcaster -> a, b, c
%a -> b
%b -> c
%c -> inv
&inv -> a";

    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT), 32000000);
    }
}
