use std::cmp::Ordering::{Greater, Less};

use regex::Regex;

use crate::utils::read_file;
use std::collections::HashMap;

pub fn day05() {
    let lines = read_file("inputs/input-day05.txt");

    let (part1, part2) = solve_both_parts(&lines);
    println!("Day 05 - Part 1: {}", part1);
    println!("Day 05 - Part 2: {}", part2);
}

fn solve_both_parts(lines: &[String]) -> (usize, usize) {
    let re = Regex::new(r"(?P<x1>\d+),(?P<y1>\d+) -> (?P<x2>\d+),(?P<y2>\d+)").unwrap();
    let mut hm: HashMap<(usize, usize), usize> = HashMap::new();
    let mut diags: Vec<(usize, usize, usize, usize)> = Vec::new();

    for l in lines {
        let caps = re.captures(l).unwrap();
        let x1: usize = caps["x1"].parse().unwrap();
        let y1: usize = caps["y1"].parse().unwrap();
        let x2: usize = caps["x2"].parse().unwrap();
        let y2: usize = caps["y2"].parse().unwrap();

        if x1 == x2 {
            let (starty, endy) = if y1 < y2 { (y1, y2) } else { (y2, y1) };
            for y in starty..=endy {
                *hm.entry((x1, y)).or_default() += 1;
            }
        } else if y1 == y2 {
            let (startx, endx) = if x1 < x2 { (x1, x2) } else { (x2, x1) };
            for x in startx..=endx {
                *hm.entry((x, y1)).or_default() += 1;
            }
        } else {
            diags.push((x1, y1, x2, y2));
        }
    }

    let part1 = hm.values().filter(|&x| *x > 1).count() as usize;

    for d in &diags {
        let (mut x1, mut y1, x2, y2) = d;
        while x1 != *x2 {
            *hm.entry((x1, y1)).or_default() += 1;
            match x1.cmp(x2) {
                Less => x1 += 1,
                Greater => x1 -= 1,
                _ => {}
            }
            match y1.cmp(y2) {
                Less => y1 += 1,
                Greater => y1 -= 1,
                _ => {}
            }
        }
        *hm.entry((x1, y1)).or_default() += 1;
    }

    let part2 = hm.values().filter(|&x| *x > 1).count() as usize;

    (part1, part2)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_both_parts() {
        let lines = vec![
            "0,9 -> 5,9".to_string(),
            "8,0 -> 0,8".to_string(),
            "9,4 -> 3,4".to_string(),
            "2,2 -> 2,1".to_string(),
            "7,0 -> 7,4".to_string(),
            "6,4 -> 2,0".to_string(),
            "0,9 -> 2,9".to_string(),
            "3,4 -> 1,4".to_string(),
            "0,0 -> 8,8".to_string(),
            "5,5 -> 8,2".to_string(),
        ];

        assert_eq!(solve_both_parts(&lines), (5, 12));
    }
}
