use crate::utils::read_file;

use std::collections::{HashMap, HashSet};

type CavePath = HashMap<String, Vec<String>>;

pub fn day12() {
    let lines = read_file("inputs/input-day12.txt");

    let mut caves: CavePath = HashMap::new();

    for l in lines {
        let mut split = l.split('-');
        let from = split.next().unwrap();
        let to = split.next().unwrap();

        caves
            .entry(from.to_string())
            .or_default()
            .push(to.to_string());
        caves
            .entry(to.to_string())
            .or_default()
            .push(from.to_string());
    }

    println!("Day 12 - Part 1: {}", count_visited(&caves));
}

struct Pather {
    visited: HashSet<String>,
    path: Vec<String>,
}

fn count_visited(caves: &CavePath) -> usize {
    let mut path_walker = Pather {
        visited: HashSet::new(),
        path: Vec::new(),
    };

    count_visited_from("start", &mut path_walker, caves)
}

fn count_visited_from(step: &str, walker: &mut Pather, caves: &CavePath) -> usize {
    if step == "end" {
        return 1;
    }

    walker.visited.insert(step.to_string());
    walker.path.push(step.to_string());

    let mut count = 0;
    for next in caves.get(step).unwrap() {
        if next.chars().next().unwrap().is_uppercase() || !walker.visited.contains(next) {
            count += count_visited_from(next, walker, caves);
        }
    }

    walker.visited.remove(step);
    walker.path.pop();

    count
}
