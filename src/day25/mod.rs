use crate::utils::read_file;

use std::collections::HashMap;

#[derive(PartialEq, Clone, Copy, Debug)]
enum Cucumber {
    Right,
    Down,
    // Empty
}
use Cucumber::*;

type Floor = HashMap<(usize, usize), Cucumber>;

pub fn day25() {
    let data = read_file("inputs/input-day25.txt");

    let mut floor: Floor = HashMap::new();

    let width = data[0].len();
    let height = data.len();

    for (y, d) in data.iter().enumerate() {
        for (x, c) in d.chars().enumerate() {
            if c == '.' {
                continue;
            }
            floor.insert((x, y), match c {
                'v' => Down,
                '>' => Right,
                _ => panic!("bad input {}", d),
            });
        }
    }

    let mut count = 1;
    while step(&mut floor, width, height) {
        count += 1;
    }
    println!("Day 25 - Part 1: {}", count);
}

#[cfg(test)]
fn print(floor: &Floor, width: usize, height: usize) {
    for y in 0..height {
        for x in 0..width {
            match floor.get(&(x, y)) {
                None => print!("."),
                Some(Right) => print!(">"),
                Some(Down) => print!("v"),
            }
        }
        println!();
    }
}

fn step(floor: &mut Floor, width: usize, height: usize) -> bool {
    let mut new_floor: Floor = HashMap::new();
    let mut moved = false;

    for ((x, y), c) in &*floor {
        if *c == Down {
            continue;
        }
        let newx = (*x + 1) % width;
        if floor.contains_key(&(newx, *y)) {
            new_floor.insert((*x, *y), *c);
            continue;
        }
        new_floor.insert((newx, *y), *c);
        moved = true;
    }

    for ((x, y), c) in &*floor {
        if *c == Right {
            continue;
        }
        let newy = (*y + 1) % height;
        if new_floor.contains_key(&(*x, newy)) {
            new_floor.insert((*x, *y), *c);
            continue;
        }
        if let Some(cur_floor) = floor.get(&(*x, newy)) {
            if *cur_floor == Down {
                new_floor.insert((*x, *y), *c);
                continue;
            }
        }
        new_floor.insert((*x, newy), *c);
        moved = true;
    }

    floor.clear();
    floor.extend(new_floor);

    moved
}
