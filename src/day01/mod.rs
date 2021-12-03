use crate::utils::read_file;

pub fn day01() {
    let lines = read_file("inputs/input-day01.txt");

    let nums: Vec<i32> = lines.iter().map(|num| num.parse().unwrap()).collect();

    println!("Day 01 - Part 1: {}", increasing(&nums, 1));
    println!("Day 01 - Part 2: {}", increasing(&nums, 3));
}

fn increasing(nums: &[i32], offset: usize) -> usize {
    nums.windows(offset + 1)
        .map(|x| (x[0] < x[offset]) as usize)
        .sum() // fold(0, |a, b| a + b)
}
