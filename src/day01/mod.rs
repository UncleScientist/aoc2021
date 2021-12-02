use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

pub fn day01() {
    let file = File::open("inputs/input-day01.txt").expect("Cannot find file");
    let buf = BufReader::new(file);
    let lines = buf
        .lines()
        .map(|line| line.unwrap())
        .collect::<Vec<String>>();

    let nums: Vec<i32> = lines.iter().map(|num| num.parse().unwrap()).collect();

    println!("Day 01 - Part 1: {}", increasing(&nums, 1));
    println!("Day 01 - Part 2: {}", increasing(&nums, 3));
}

fn increasing(nums: &[i32], offset: usize) -> usize {
    nums.windows(offset + 1)
        .map(|x| (x[0] < x[offset]) as usize)
        .sum() // fold(0, |a, b| a + b)
}
