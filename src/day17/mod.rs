// for my login, input data is `target area: x=288..330, y=-96..-50`

pub fn day17() {
    let minx: i64 = 288;
    let maxx: i64 = 330;
    let miny: i64 = -96;
    let maxy: i64 = -50;

    println!("Day 17 - Part 1: {}", part1(miny));
}

pub fn part1(miny: i64) -> i64 {
    miny * (miny + 1) / 2
}
