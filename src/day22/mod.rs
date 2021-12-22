use crate::utils::read_file;
use regex::Regex;
use std::collections::HashSet;

pub fn day22() {
    let lines = read_file("inputs/input-day22.txt");

    let mut reactor: HashSet<(i64, i64, i64)> = HashSet::new();

    let re = Regex::new(r"x=(?P<xmin>-*\d+)\.\.(?P<xmax>-*\d+),y=(?P<ymin>-*\d+)\.\.(?P<ymax>-*\d+),z=(?P<zmin>-*\d+)\.\.(?P<zmax>-*\d+)").unwrap();

    for l in lines {
        let (switch, cube) = l.split_once(' ').unwrap();
        let caps = re.captures(cube).unwrap();
        let xmin: i64 = caps["xmin"].parse().unwrap();
        let xmax: i64 = caps["xmax"].parse().unwrap();
        let ymin: i64 = caps["ymin"].parse().unwrap();
        let ymax: i64 = caps["ymax"].parse().unwrap();
        let zmin: i64 = caps["zmin"].parse().unwrap();
        let zmax: i64 = caps["zmax"].parse().unwrap();
        println!(
            "sw={}, {} {} {} {} {} {}",
            switch, xmin, xmax, ymin, ymax, zmin, zmax
        );

        if xmax < -50 || xmin > 50 || ymax < -50 || ymin > 50 || zmax < -50 || zmin > 50 {
            continue;
        }

        for x in xmin..=xmax {
            for y in ymin..=ymax {
                for z in zmin..=zmax {
                    if switch == "on" {
                        reactor.insert((x, y, z));
                    } else {
                        reactor.remove(&(x, y, z));
                    }
                }
            }
        }
    }

    println!("Day 22 - Part 1: {}", reactor.len());
}
