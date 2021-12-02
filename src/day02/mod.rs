use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

pub fn day02() {
    let file = File::open("inputs/input-day02.txt").expect("Cannot find file");
    let buf = BufReader::new(file);
    let mut lines = buf
        .lines()
        .map(|line| line.unwrap())
        .collect::<Vec<String>>();

    println!("Day 02 - Part 1: {}", part1(&mut lines));
    println!("Day 02 - Part 2: x");
}

fn part1(commands: &mut Vec<String>) -> i32 {
    let result: (i32, i32) = commands
        .iter()
        .map(|s| s.split_whitespace())
        .map(|mut cmd| match cmd.next().unwrap() {
            "forward" => (0, cmd.next().unwrap().parse().unwrap()),
            "down" => (cmd.next().unwrap().parse().unwrap(), 0),
            "up" => (-(cmd.next().unwrap().parse::<i32>().unwrap()), 0),
            x => panic!("got strange command: {}", x),
        })
        .fold((0, 0), |a, b| (a.0 + b.0, a.1 + b.1));

    result.0 * result.1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn position() {
        let mut commands = vec![
            "forward 5\n".to_string(),
            "down 5\n".to_string(),
            "forward 8\n".to_string(),
            "up 3\n".to_string(),
            "down 8\n".to_string(),
            "forward 2\n".to_string(),
        ];

        assert_eq!(part1(&mut commands), 150);
    }
}
