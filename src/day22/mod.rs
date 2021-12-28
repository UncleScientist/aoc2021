use crate::utils::read_file;
use regex::Regex;

#[derive(PartialEq)]
enum Charge {
    Positive,
    Negative
}

struct Energy {
    region: Region,
    charge: Charge,
}

struct Reactor {
    reactor: Vec<Energy>,
}

impl Reactor {
    fn new() -> Reactor {
        Reactor { reactor: Vec::new() }
    }

    fn process(&mut self, add: &Region) {
        let mut update = Vec::new();
        for r in &self.reactor {
            if let Some(intersect) = r.region.intersect(add) {
                if r.charge == Charge::Positive {
                    update.push(Energy { region: intersect, charge: Charge::Negative });
                } else {
                    update.push(Energy { region: intersect, charge: Charge::Positive });
                }
            }
        }
        self.reactor.extend(update);
    }

    fn insert(&mut self, add: &Region) {
        self.process(add);
        self.reactor.push(Energy { region: add.clone(), charge: Charge::Positive });
    }

    fn remove(&mut self, rem: &Region) {
        self.process(rem);
    }

    fn volume(&self) -> i64 {
        let mut total = 0;
        for r in &self.reactor {
            match r.charge {
                Charge::Positive => total += r.region.volume(),
                Charge::Negative => total -= r.region.volume(),
            }
        }

        total
    }
}

#[derive(Copy, Clone, Debug)]
struct Region {
    xmin: i64,
    xmax: i64,
    ymin: i64,
    ymax: i64,
    zmin: i64,
    zmax: i64,
}

impl Region {
    fn new(xmin: i64, xmax: i64, ymin: i64, ymax: i64, zmin: i64, zmax: i64) -> Region {
        Region { xmin, xmax, ymin, ymax, zmin, zmax }
    }

    fn intersect(&self, other: &Region) -> Option<Region> {
        if !self.overlaps(other) {
            return None;
        }
        Some(Region {
            xmin: self.xmin.max(other.xmin),
            xmax: self.xmax.min(other.xmax),
            ymin: self.ymin.max(other.ymin),
            ymax: self.ymax.min(other.ymax),
            zmin: self.zmin.max(other.zmin),
            zmax: self.zmax.min(other.zmax),
        })
    }

    fn overlaps(&self, other: &Region) -> bool {
        self.xmin <= other.xmax && self.xmax >= other.xmin &&
        self.ymin <= other.ymax && self.ymax >= other.ymin &&
        self.zmin <= other.zmax && self.zmax >= other.zmin
    }

    fn volume(&self) -> i64 {
        (self.xmax - self.xmin + 1) *
        (self.ymax - self.ymin + 1) *
        (self.zmax - self.zmin + 1)
    }

}

pub fn day22() {
    let lines = read_file("inputs/input-day22.txt");

    let mut part1 = Reactor::new();
    let mut part2 = Reactor::new();

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

        let r = Region::new(xmin, xmax, ymin, ymax, zmin, zmax);

        if xmin >= -50 && xmax <= 50 && ymin >= -50 && ymax <= 50 && zmin >= -50 && zmax <= 50 {
           if switch == "on" {
               part1.insert(&r);
           } else {
               part1.remove(&r);
           }
        }

        if switch == "on" {
            part2.insert(&r);
        } else {
            part2.remove(&r);
        }
    }

    println!("Day 22 - Part 1: {}", part1.volume());
    println!("Day 22 - Part 2: {}", part2.volume());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn does_overlap() {
        let r1 = Region::new(-3, 3, 10, 20, 30, 40);
        let r2 = Region::new(-6, -3, 10, 20, 30, 40);
        assert!(r1.overlaps(&r2));
    }

    #[test]
    fn does_not_overlap() {
        let r1 = Region::new(-3, 3, 10, 20, 30, 40);
        let r2 = Region::new(-6, -3, 50, 60, 30, 40);
        assert!(!r1.overlaps(&r2));
    }

}
