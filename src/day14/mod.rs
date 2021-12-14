use std::collections::HashMap;

use crate::utils::read_file;

type PairList = HashMap<(char, char), usize>;
type Mapper = HashMap<(char, char), char>;

pub fn day14() {
    let lines = read_file("inputs/input-day14.txt");

    let template: Vec<char> = lines[0].chars().collect();
    let firstchar = template[0];
    let mut pairs: PairList = HashMap::new();
    for w in template.windows(2) {
        *pairs.entry((w[0], w[1])).or_default() += 1;
    }

    let mut mapping: Mapper = HashMap::new();

    let mut idx = 2;
    while idx < lines.len() {
        if let Some((left, right)) = lines[idx].split_once(" -> ") {
            let mut chars = left.chars();
            let l1 = chars.next().unwrap();
            let l2 = chars.next().unwrap();
            mapping.insert((l1, l2), right.chars().next().unwrap());
        }
        idx += 1;
    }

    pairs = run_loops(&pairs, &mapping, 10);
    println!("Day 14 - Part 1: {}", get_counts(&pairs, firstchar));

    pairs = run_loops(&pairs, &mapping, 30);
    println!("Day 14 - Part 2: {}", get_counts(&pairs, firstchar));
}

fn get_counts(pairs: &PairList, firstchar: char) -> usize {
    let mut count: HashMap<char, usize> = HashMap::new();
    *count.entry(firstchar).or_default() += 1;
    for (key, val) in pairs {
        *count.entry(key.1).or_default() += val;
    }

    let min = count.values().min().unwrap();
    let max = count.values().max().unwrap();
    max - min
}

fn run_loops(pairs: &PairList, mapping: &Mapper, loops: usize) -> PairList {
    let mut pairs = pairs.clone();

    for _ in 0..loops {
        let mut newpairs = pairs.clone();
        for (tuple, count) in pairs {
            let insert = mapping.get(&tuple).unwrap();
            *newpairs.get_mut(&tuple).unwrap() -= count;
            *newpairs.entry((tuple.0, *insert)).or_default() += count;
            *newpairs.entry((*insert, tuple.1)).or_default() += count;
        }
        pairs = newpairs;
    }

    pairs
}
