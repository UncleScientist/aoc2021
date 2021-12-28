use crate::utils::read_file;
use regex::Regex;
use std::collections::HashSet;

struct Reactor {
    region: Vec<Region>,
}

impl Reactor {
    fn new() -> Reactor {
        Reactor { region: Vec::new() }
    }

    fn len(&self) -> i64 {
        0
    }

    fn join(&mut self, other: &Region) {
        for r in &self.region {
            if r.overlaps(&other) {
                // do a join
            }
        }
    }

    fn remove(&mut self, other: &Region) {
        for r in &self.region {
            if r.overlaps(&other) {
                // do a remove
            }
        }
    }
}

#[derive(Copy, Clone)]
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

    fn is_small(&self) -> bool {
        self.xmin >= -50 && self.xmax <= 50 &&
        self.ymin >= -50 && self.ymax <= 50 &&
        self.zmin >= -50 && self.zmax <= 50
    }

    //
    //  +-----+
    //  |self |
    //  |  +--+---------+
    //  |  |  |   other |
    //  +--+--+         |
    //     |            |
    //     +------------+

    fn overlaps(&self, other: &Region) -> bool {
        // minA <= maxB && maxA >= minB iff minA <= maxA
        let xoverlaps = (self.xmin >= other.xmin && self.xmin <= other.xmax) ||
                        (self.xmax >= other.xmin && self.xmax <= other.xmax);
        let yoverlaps = (self.ymin >= other.ymin && self.ymin <= other.ymax) ||
                        (self.ymax >= other.ymin && self.ymax <= other.ymax);
        let zoverlaps = (self.zmin >= other.zmin && self.zmin <= other.zmax) ||
                        (self.zmax >= other.zmin && self.zmax <= other.zmax);
        xoverlaps && yoverlaps && zoverlaps
    }

    //
    // +-----+                  ymax of top square
    // |     |
    // +-----+-----------+      ymax of bottom square
    // |                 |
    // +-----+-----------+      ymin of top square
    //       |           |
    //       +-----------+      ymin of bottom square
    //
    // xit   xxt         xxb

    //
    //          +-----+         ymax of top square
    //          |self |
    // +--------+-----+--+      ymax of bottom square
    // | other  +-----+  |      ymin of top square
    // +-----+-----------+      ymin of bottom square
    // 

    fn join(&self, other: &Region) -> Vec<Region> {
        let mut result: Vec<Region> = Vec::new();

        result.push(other.clone());

        if !self.overlaps(other) {
            result.push(self.clone());
            return result;
        }

        if self.ymin < other.ymin {
            if self.xmin < other.xmin {
                if self.zmin < other.zmin {
                    result.push(Region {
                        xmin: self.xmin, xmax: other.xmin,
                        ymin: self.ymin, ymax: other.ymin,
                        zmin: self.zmin, zmax: other.zmin});
                    if self.xmax > other.xmin {
                        result.push(Region {
                            xmin: self.xmin, xmax: self.xmax,
                            ymin: self.ymin: ymax: other.ymin,
                            zmin: self.zmin, zmax: other.zmin });
                    }
                    if self.zmax > other.zmin {
                        result.push(Region {
                            xmin: self.xmin, xmax: self.xmax,
                            ymin: self.ymin, ymax: other.ymin,
                            zmin: other.zmin, zmax: self.zmax })
                    }
                } else {
                    // self.zmin > other.zmin
                    result.push(Region {
                        xmin: self.xmin, xmax: other.xmin,
                        ymin: self.ymin, ymax: other.ymin,
                        zmin: self.zmin, zmax: self.zmax });
                    if self.xmax > other.xmin {
                        result.push(Region {
                            xmin: self.xmin, xmax: self.xmax,
                            ymin: self.ymin, ymax: other.ymin,
                            zmin: self.zmin, zmax: self.zmax });
                    }
                }
            } else {
                something
            }
        } else {
            something
        }

        //  +-------------+
        //  |  other      |
        //  |  +-------+  |
        //  |  | self  |  |
        //  |  +-------+  |
        //  +-------------+

        if self.xmin >= other.xmin && self.xmax <= other.xmax &&
           self.ymin >= other.ymin && self.ymax <= other.ymax {
            result.push(other.clone());
            if self.zmax > other.zmax {
                // sticking out away from us
                result.push(Region {
                    xmin: self.xmin, xmax: self.xmax,
                    ymin: self.ymin, ymax: self.ymax,
                    zmin: other.zmax, zmax: self.zmax });
            }
            if self.zmin < other.zmin {
                // sticking out towards us
                result.push(Region {
                    xmin: self.xmin, xmax: self.xmax,
                    ymin: self.ymin, ymax: self.ymax,
                    zmin: self.zmin, zmax: other.zmin });
            }
        } else if self.xmin >= other.xmin && self.xmax <= other.xmax &&
           self.zmin >= other.zmin && self.zmax <= other.zmax {
            result.push(other.clone());
            if self.ymax > other.ymax {
                result.push(Region {
                    xmin: self.xmin, xmax: self.xmax,
                    ymin: other.ymax, ymax: self.ymax,
                    zmin: self.zmin, zmax: self.zmax });
            }
            if self.ymin < other.ymin {
                // sticking out towards us
                result.push(Region {
                    xmin: self.xmin, xmax: self.xmax,
                    ymin: self.ymin, ymax: other.ymin,
                    zmin: self.zmin, zmax: self.zmax });
            }
        } else if self.ymin >= other.ymin && self.ymax <= other.ymax &&
           self.zmin >= other.zmin && self.zmax <= other.zmax {
            result.push(other.clone());
            if self.ymax > other.ymax {
                result.push(Region {
                    xmin: other.xmax, xmax: self.xmax,
                    ymin: self.ymin, ymax: self.ymax,
                    zmin: self.zmin, zmax: self.zmax });
            }
            if self.ymin < other.ymin {
                // sticking out towards us
                result.push(Region {
                    xmin: self.xmin, xmax: other.xmin,
                    ymin: self.ymin, ymax: self.ymax,
                    zmin: self.zmin, zmax: self.zmax });
            }
        } else {
            if self.ymax > other.ymax {
                result.push(Region {
                    xmin: self.xmin, xmax: self.xmax,
                    ymin: other.ymax, ymax: self.ymax,
                    zmin: self.zmin, zmax: self.zmax
                });
                if self.zmin < other.zmin {
                    result.push(other.clone());
                    result.push(Region {
                        xmin: self.xmin, xmax: self.xmax,
                        ymin: self.ymin, ymax: other.ymax,
                        zmin: self.zmin, zmax: other.zmin });
                } else if self.zmax > other.zmax {
                    result.push(other.clone());
                    result.push
                }
                if self.xmax > other.xmax {
                    result.push(Region {
                        xmin: other.xmax, xmax: self.xmax,
                        ymin: self.ymin, ymax: other.ymax,
                        zmin: self.zmin, zmax: self.zmax });
                }
            }
        }
        result
    }
}

pub fn day22() {
    let lines = read_file("inputs/test-day22.txt");

    // let mut reactor: HashSet<(i64, i64, i64)> = HashSet::new();
    let mut reactor = Reactor::new();

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

        /*
        if r.is_small() {
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
        */

        if r.is_small() {
            if switch == "on" {
                reactor.join(&r);
            } else {
                reactor.remove(&r);
            }
        }
    }

    println!("Day 22 - Part 1: {}", reactor.len());
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
