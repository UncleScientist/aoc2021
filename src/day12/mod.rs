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
    println!("Day 12 - Part 2: {}", count_double_visited(&caves));
}

#[derive(Default)]
struct Pather {
    visited: HashMap<String, usize>,
    path: Vec<String>,
    unique: HashSet<String>,
}

fn count_visited(caves: &CavePath) -> usize {
    let mut path_walker = Pather {
        ..Default::default()
    };

    count_visited_from("start", &mut path_walker, caves, &None)
}

fn count_double_visited(caves: &CavePath) -> usize {
    let mut path_walker = Pather {
        ..Default::default()
    };

    for c in caves.keys() {
        if *c != "start" && *c != "end" {
            let first = c.chars().next().unwrap();
            if first.is_lowercase() {
                count_visited_from("start", &mut path_walker, caves, &Some(c.to_string()));
            } else {
                count_visited_from("start", &mut path_walker, caves, &None);
            }
        }
    }

    path_walker.unique.len()
}

fn count_visited_from(
    step: &str,
    walker: &mut Pather,
    caves: &CavePath,
    double: &Option<String>,
) -> usize {
    if step == "end" {
        walker.unique.insert(format!("{:?}", walker.path));
        return 1;
    }

    *walker.visited.entry(step.to_string()).or_default() += 1;
    walker.path.push(step.to_string());

    let mut count = 0;
    for next in caves.get(step).unwrap() {
        if next.chars().next().unwrap().is_uppercase()
            || *walker.visited.entry(next.to_string()).or_default() == 0
        {
            count += count_visited_from(next, walker, caves, double);
        } else if let Some(d) = double {
            if d == next && *walker.visited.entry(next.to_string()).or_default() < 2 {
                count += count_visited_from(next, walker, caves, double);
            }
        }
    }

    *walker.visited.entry(step.to_string()).or_default() -= 1;
    walker.path.pop();

    count
}
