use crate::utils::read_file;

pub fn day10() {
    let lines = read_file("inputs/input-day10.txt");

    let part1 = lines.iter().fold(0, |sum, line| sum + score(line));
    println!("Day 10 - Part 1: {}", part1);
}

pub fn score(line: &str) -> i32 {
    let mut expect: Vec<char> = Vec::new();

    for c in line.chars() {
        match c {
            '(' => expect.push(')'),
            '{' => expect.push('}'),
            '[' => expect.push(']'),
            '<' => expect.push('>'),
            _ => {
                if c != expect.pop().unwrap() {
                    match c {
                        ')' => return 3,
                        ']' => return 57,
                        '}' => return 1197,
                        '>' => return 25137,
                        _ => panic!("illegal char {}", c),
                    }
                }
            }
        }
    }

    0
}
