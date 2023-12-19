use itertools::Itertools;
use lazy_static::lazy_static;
use regex::Regex;
use std::cmp::{max, min};
use std::collections::HashMap;

lazy_static! {
    static ref PART_RE: Regex = Regex::new(r"\{x=(\d+),m=(\d+),a=(\d+),s=(\d+)\}").unwrap();
}

fn main() {
    let input = aoc2023::readfile("inputs/19");
    println!("part1: {}", part1(&input));
    println!("part2: {}", part2(&input));
}

fn part1(input: &str) -> usize {
    let (workflows_str, parts_str) = input.split_once("\n\n").unwrap();
    let workflows = workflows_str
        .lines()
        .map(Workflow::from)
        .map(|wf| (wf.name.to_string(), wf))
        .collect::<HashMap<_, _>>();
    let parts = parts_str.lines().map(Part::from).collect_vec();

    parts
        .iter()
        .filter(|p| p.is_accepted_by(&workflows))
        .map(Part::value)
        .sum()
}

fn part2(input: &str) -> usize {
    let (workflows_str, _) = input.split_once("\n\n").unwrap();
    let workflows = workflows_str
        .lines()
        .map(Workflow::from)
        .map(|wf| (wf.name.to_string(), wf))
        .collect::<HashMap<_, _>>();

    count_accepted(
        &workflows,
        "in",
        InclusiveRange::new(),
        InclusiveRange::new(),
        InclusiveRange::new(),
        InclusiveRange::new(),
    )
}

fn count_accepted(
    workflows: &HashMap<String, Workflow>,
    curr: &str,
    mut x: InclusiveRange,
    mut m: InclusiveRange,
    mut a: InclusiveRange,
    mut s: InclusiveRange,
) -> usize {
    if let Some(workflow) = workflows.get(curr) {
        let mut result = 0;
        for rule in &workflow.rules {
            let (nx, x1) = rule.cut_x(x);
            x = x1;
            let (nm, m1) = rule.cut_m(m);
            m = m1;
            let (na, a1) = rule.cut_a(a);
            a = a1;
            let (ns, s1) = rule.cut_s(s);
            s = s1;

            if !nx.is_empty() && !nm.is_empty() && !na.is_empty() && !ns.is_empty() {
                result += count_accepted(workflows, rule.target(), nx, nm, na, ns);
            }
        }
        result
    } else if curr == "R" {
        0
    } else if curr == "A" {
        x.len() * m.len() * a.len() * s.len()
    } else {
        panic!("Unknown state {}", curr)
    }
}

enum Rule {
    XGreater(usize, String),
    XLess(usize, String),
    MGreater(usize, String),
    MLess(usize, String),
    AGreater(usize, String),
    ALess(usize, String),
    SGreater(usize, String),
    SLess(usize, String),
    Default(String),
}

impl From<&str> for Rule {
    fn from(value: &str) -> Self {
        if let Some((rule, target)) = value.split_once(':') {
            let var = rule.chars().nth(0).unwrap();
            let op = rule.chars().nth(1).unwrap();
            let constant = aoc2023::parse(&rule[2..]);
            match (var, op) {
                ('x', '>') => Rule::XGreater(constant, target.to_string()),
                ('x', '<') => Rule::XLess(constant, target.to_string()),
                ('m', '>') => Rule::MGreater(constant, target.to_string()),
                ('m', '<') => Rule::MLess(constant, target.to_string()),
                ('a', '>') => Rule::AGreater(constant, target.to_string()),
                ('a', '<') => Rule::ALess(constant, target.to_string()),
                ('s', '>') => Rule::SGreater(constant, target.to_string()),
                ('s', '<') => Rule::SLess(constant, target.to_string()),
                (_, _) => panic!("Invalid rule {}", rule),
            }
        } else {
            Rule::Default(value.to_string())
        }
    }
}

impl Rule {
    fn process(&self, p: &Part) -> Option<&str> {
        match self {
            Rule::XGreater(n, s) if p.x > *n => Some(s),
            Rule::XLess(n, s) if p.x < *n => Some(s),
            Rule::MGreater(n, s) if p.m > *n => Some(s),
            Rule::MLess(n, s) if p.m < *n => Some(s),
            Rule::AGreater(n, s) if p.a > *n => Some(s),
            Rule::ALess(n, s) if p.a < *n => Some(s),
            Rule::SGreater(n, s) if p.s > *n => Some(s),
            Rule::SLess(n, s) if p.s < *n => Some(s),
            Rule::Default(s) => Some(s),
            _ => None,
        }
    }

    fn target(&self) -> &str {
        match self {
            Rule::XGreater(_, s) => s,
            Rule::XLess(_, s) => s,
            Rule::MGreater(_, s) => s,
            Rule::MLess(_, s) => s,
            Rule::AGreater(_, s) => s,
            Rule::ALess(_, s) => s,
            Rule::SGreater(_, s) => s,
            Rule::SLess(_, s) => s,
            Rule::Default(s) => s,
        }
    }

    fn cut_x(&self, x: InclusiveRange) -> (InclusiveRange, InclusiveRange) {
        match self {
            Rule::XGreater(n, _) => (x.greater(*n), x.less_equal(*n)),
            Rule::XLess(n, _) => (x.less(*n), x.greater_equal(*n)),
            _ => (x, x),
        }
    }

    fn cut_m(&self, m: InclusiveRange) -> (InclusiveRange, InclusiveRange) {
        match self {
            Rule::MGreater(n, _) => (m.greater(*n), m.less_equal(*n)),
            Rule::MLess(n, _) => (m.less(*n), m.greater_equal(*n)),
            _ => (m, m),
        }
    }

    fn cut_a(&self, a: InclusiveRange) -> (InclusiveRange, InclusiveRange) {
        match self {
            Rule::AGreater(n, _) => (a.greater(*n), a.less_equal(*n)),
            Rule::ALess(n, _) => (a.less(*n), a.greater_equal(*n)),
            _ => (a, a),
        }
    }

    fn cut_s(&self, s: InclusiveRange) -> (InclusiveRange, InclusiveRange) {
        match self {
            Rule::SGreater(n, _) => (s.greater(*n), s.less_equal(*n)),
            Rule::SLess(n, _) => (s.less(*n), s.greater_equal(*n)),
            _ => (s, s),
        }
    }
}

struct Workflow {
    name: String,
    rules: Vec<Rule>,
}

impl From<&str> for Workflow {
    fn from(value: &str) -> Self {
        let (name, args) = value.split_once('{').unwrap();
        let rules = args
            .trim_end_matches('}')
            .split(',')
            .map(Rule::from)
            .collect();
        Self {
            name: name.to_string(),
            rules,
        }
    }
}

impl Workflow {
    fn process(&self, p: &Part) -> &str {
        self.rules
            .iter()
            .filter_map(|r| r.process(p))
            .next()
            .unwrap()
    }
}

struct Part {
    x: usize,
    m: usize,
    a: usize,
    s: usize,
}

impl From<&str> for Part {
    fn from(value: &str) -> Self {
        let capture = PART_RE.captures(value).unwrap();
        let x = aoc2023::parse(capture.get(1).unwrap().as_str());
        let m = aoc2023::parse(capture.get(2).unwrap().as_str());
        let a = aoc2023::parse(capture.get(3).unwrap().as_str());
        let s = aoc2023::parse(capture.get(4).unwrap().as_str());
        Part { x, m, a, s }
    }
}

impl Part {
    fn is_accepted_by(&self, workflows: &HashMap<String, Workflow>) -> bool {
        let mut curr = "in";
        while let Some(workflow) = workflows.get(curr) {
            curr = workflow.process(self);
        }
        match curr {
            "R" => false,
            "A" => true,
            _ => panic!("Part.is_accepted_by() failed"),
        }
    }

    fn value(&self) -> usize {
        self.x + self.m + self.a + self.s
    }
}

#[derive(Debug, Clone, Copy)]
struct InclusiveRange(usize, usize);

impl InclusiveRange {
    fn new() -> Self {
        Self(1, 4000)
    }
    fn is_empty(&self) -> bool {
        self.1 <= self.0
    }
    fn len(&self) -> usize {
        self.1 - self.0 + 1
    }
    fn greater(&self, n: usize) -> Self {
        Self(max(self.0, n + 1), self.1)
    }
    fn less(&self, n: usize) -> Self {
        Self(self.0, min(self.1, n - 1))
    }
    fn greater_equal(&self, n: usize) -> Self {
        Self(max(self.0, n), self.1)
    }
    fn less_equal(&self, n: usize) -> Self {
        Self(self.0, min(self.1, n))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "px{a<2006:qkq,m>2090:A,rfg}
pv{a>1716:R,A}
lnx{m>1548:A,A}
rfg{s<537:gd,x>2440:R,A}
qs{s>3448:A,lnx}
qkq{x<1416:A,crn}
crn{x>2662:A,R}
in{s<1351:px,qqz}
qqz{s>2770:qs,m<1801:hdj,R}
gd{a>3333:R,R}
hdj{m>838:A,pv}

{x=787,m=2655,a=1222,s=2876}
{x=1679,m=44,a=2067,s=496}
{x=2036,m=264,a=79,s=2244}
{x=2461,m=1339,a=466,s=291}
{x=2127,m=1623,a=2188,s=1013}";

    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT), 19114);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(INPUT), 167409079868000);
    }
}
