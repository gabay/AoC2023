use std::{
    collections::{HashMap, HashSet},
    fmt::Display,
};

use itertools::Itertools;

fn main() {
    let input = aoc2023::readfile("inputs/25");
    println!("part1: {}", part1(&input));
}

fn part1(input: &str) -> usize {
    let g = Graph::from(input);
    let connected_size = g.min_cut_connected_size();
    (connected_size) * (g.len() - connected_size)
}

#[derive(Debug, Clone)]
struct Graph<'a>(HashMap<&'a str, Vec<&'a str>>);

impl<'a> Graph<'a> {
    fn min_cut_connected_size(&self) -> usize {
        let v0 = *self.0.keys().next().unwrap();
        let mut min_cut_size = self.neighbors(v0).len();
        let mut min_cut_connected_size = 1;
        let mut connected = HashSet::from([v0]);
        let mut candidates = HashMap::new();
        for v1 in self.neighbors(v0) {
            candidates.insert(v1, 1);
        }

        while connected.len() + 1 < self.0.len() {
            let candidate = candidates
                .keys()
                .sorted_by_key(|k| candidates[*k])
                .next_back()
                .unwrap()
                .to_owned();

            connected.insert(candidate);
            candidates.remove(candidate);
            for n in self.neighbors(candidate) {
                if !connected.contains(n) {
                    *candidates.entry(n).or_insert(0) += 1;
                }
            }

            let cut_size = candidates.values().sum();
            if min_cut_size > cut_size {
                min_cut_size = cut_size;
                min_cut_connected_size = connected.len();
            }
        }
        min_cut_connected_size
    }

    fn neighbors(&self, u: &str) -> Vec<&str> {
        self.0.get(u).unwrap().to_owned()
    }

    fn len(&self) -> usize {
        self.0.len()
    }

    // // Initial approach - iterating over all tuples of 3 edges and checking if they are a min cut.
    // fn is_connected(&self, a: &str, b: &str, dropped_edges: &HashSet<(&str, &str)>) -> bool {
    //     let mut seen = HashSet::new();
    //     let mut queue = vec![a];
    //     while let Some(u) = queue.pop() {
    //         if u == b {
    //             return true;
    //         }
    //         if seen.insert(u) {
    //             for v in self.neighbors(u) {
    //                 if !dropped_edges.contains(&(u, v)) && !dropped_edges.contains(&(v, u)) {
    //                     queue.push(v);
    //                 }
    //             }
    //         }
    //     }
    //     false
    // }

    // fn connected_size(&self, a: &str, dropped_edges: &HashSet<(&str, &str)>) -> usize {
    //     let mut seen = HashSet::new();
    //     let mut queue = vec![a];
    //     while let Some(u) = queue.pop() {
    //         if seen.insert(u) {
    //             for v in self.neighbors(u) {
    //                 if !dropped_edges.contains(&(u, v)) && !dropped_edges.contains(&(v, u)) {
    //                     queue.push(v);
    //                 }
    //             }
    //         }
    //     }
    //     seen.len()
    // }

    // fn vertices(&self) -> Vec<&&str> {
    //     self.0.keys().collect_vec()
    // }

    // fn edges(&self) -> Vec<(&str, &str)> {
    //     let mut e = vec![];
    //     for u in self.vertices().iter().sorted() {
    //         e.extend(self.neighbors(u).iter().sorted().map(|v| (**u, *v)))
    //     }
    //     e
    // }

}


impl<'a> From<&'a str> for Graph<'a> {
    fn from(value: &'a str) -> Self {
        let mut edges = HashMap::new();
        value
            .lines()
            .flat_map(|line| {
                let (src, dsts) = line.split_once(": ").unwrap();
                dsts.split(' ').map(|dst| (src, dst)).collect_vec()
            })
            .for_each(|(src, dst)| {
                edges.entry(src).or_insert(vec![]).push(dst);
                edges.entry(dst).or_insert(vec![]).push(src);
            });
        Self(edges)
    }
}

impl<'a> Display for Graph<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for key in self.0.keys() {
            writeln!(f, "{} -> {:?}", key, self.0[key]).unwrap();
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "jqt: rhn xhk nvd
rsh: frs pzl lsr
xhk: hfx
cmg: qnr nvd lhk bvb
rhn: xhk bvb hfx
bvb: xhk hfx
pzl: lsr hfx nvd
qnr: nvd
ntq: jqt hfx bvb xhk
nvd: lhk
lsr: lhk
rzs: qnr cmg lsr rsh
frs: qnr lhk lsr";

    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT), 54);
    }
}
