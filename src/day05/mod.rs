use regex::Regex;

use crate::utils::read_file;
use std::collections::HashMap;

pub fn day05() {
    let lines = read_file("inputs/input-day05.txt");

    println!("Day 05 - Part 1: {}", solve_part_1(&lines));
}

fn solve_part_1(lines: &[String]) -> usize {
    let re = Regex::new(r"(?P<x1>\d+),(?P<y1>\d+) -> (?P<x2>\d+),(?P<y2>\d+)").unwrap();
    let mut hm: HashMap<(usize, usize), usize> = HashMap::new();

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
        }
    }

    hm.values().filter(|&x| *x > 1).count() as usize
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
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

        assert_eq!(solve_part_1(&lines), 5);
    }
}
