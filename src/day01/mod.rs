use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

pub fn day01() {
    let file = File::open("inputs/input-day01.txt").expect("Cannot find file");
    let buf = BufReader::new(file);
    let lines = buf.lines().map(|line| line.unwrap()).collect::<Vec<String>>();

    let nums : Vec<i32> = lines.iter().map(|num| num.parse().unwrap()).collect();

    let mut count = 0;
    for i in nums.windows(2) {
        if i[0] < i[1] {
            count += 1;
        }
    }

    println!("Day 01 - Part 1: {}", count);
}
