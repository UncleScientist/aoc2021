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
    ];

    for d in days {
        let now = Instant::now();
        d();
        let elapsed = now.elapsed();

        println!(
            "-- Completion time: {:6.2}ms\n",
            elapsed.as_micros() as f64 / 1000.
        );
    }
}
