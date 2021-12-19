use chrono::Datelike;
use std::env;
use std::time::Instant;

mod utils;

mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod day09;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod day16;
mod day17;
mod day18;
mod day19;

fn main() {
    let days: Vec<fn()> = vec![
        day01::day01,
        day02::day02,
        day03::day03,
        day04::day04,
        day05::day05,
        day06::day06,
        day07::day07,
        day08::day08,
        day09::day09,
        day10::day10,
        day11::day11,
        day12::day12,
        day13::day13,
        day14::day14,
        day15::day15,
        day16::day16,
        day17::day17,
        day18::day18,
        day19::day19,
    ];

    let args: Vec<String> = env::args().collect();
    let today = chrono::Local::now();
    let mut run_all = false;
    let mut which_day = today.day() as usize - 1;

    if args.len() > 1 {
        if args[1] == "-a" {
            run_all = true;
        } else if let Ok(day) = args[1].parse::<usize>() {
            which_day = day - 1;
        }
    } else if today.month() == 12 {
        which_day = today.day() as usize - 1;
    } else {
        run_all = true;
    }

    if run_all {
        for d in days {
            run(d);
        }
    } else {
        run(days[which_day]);
    }
}

fn run(func: fn()) {
    let now = Instant::now();
    func();
    let elapsed = now.elapsed();
    println!(
        "-- Completion time: {:6.2}ms\n",
        elapsed.as_micros() as f64 / 1000.
    );
}
