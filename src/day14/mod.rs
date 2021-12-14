use std::collections::HashMap;

use crate::utils::read_file;

pub fn day14() {
    let lines = read_file("inputs/input-day14.txt");

    let mut template: Vec<char> = lines[0].chars().collect();

    let mut mapping: HashMap<(char, char), char> = HashMap::new();

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

    for _ in 0..10 {
        let mut newvec: Vec<char> = vec![template[0]];
        for w in template.windows(2) {
            let nextchar = mapping.get(&(w[0], w[1])).unwrap();
            newvec.push(*nextchar);
            newvec.push(w[1]);
        }
        template = newvec;
    }

    let mut count: HashMap<char, usize> = HashMap::new();
    for c in template {
        *count.entry(c).or_default() += 1;
    }

    let min = count.values().min().unwrap();
    let max = count.values().max().unwrap();
    println!("Day 14 - Part 1: {}", max - min);
}
