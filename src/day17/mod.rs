// for my login, input data is `target area: x=288..330, y=-96..-50`

pub fn day17() {
    let minx: i64 = 288;
    let maxx: i64 = 330;
    let miny: i64 = -96;
    let maxy: i64 = -50;

    println!("Day 17 - Part 1: {}", part1(miny));
    println!("Day 17 - Part 2: {}", part2(minx, maxx, miny, maxy));
}

pub fn part1(miny: i64) -> i64 {
    miny * (miny + 1) / 2
}

pub fn part2(minx: i64, maxx: i64, miny: i64, maxy: i64) -> usize {
    let mut count = 0;

    for y in miny..=1 - miny {
        for x in 0..=maxx {
            let mut xpos = 0;
            let mut ypos = 0;

            for t in 0..1000 {
                ypos += y - t;
                if x - t > 0 {
                    xpos += x - t;
                }
                if miny <= ypos && ypos <= maxy && minx <= xpos && xpos <= maxx {
                    count += 1;
                    break;
                }
            }
        }
    }

    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn count_velocities() {
        assert_eq!(part2(20, 30, -10, -5), 112);
    }
}
