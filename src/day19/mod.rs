use crate::utils::read_file;

use std::collections::{HashSet, VecDeque};

type ScanData = (i64, i64, i64);
type Scanner = HashSet<ScanData>;

pub fn day19() {
    let lines = read_file("inputs/input-day19.txt");

    let mut scanners: VecDeque<Scanner> = VecDeque::new();
    let mut scanner: Scanner = HashSet::new();
    for l in lines {
        if l.starts_with("---") {
            continue;
        }
        if l.is_empty() {
            scanners.push_back(scanner);
            scanner = HashSet::new();
            continue;
        }
        let mut splitter = l.split(',');
        let x: i64 = splitter.next().unwrap().parse().unwrap();
        let y: i64 = splitter.next().unwrap().parse().unwrap();
        let z: i64 = splitter.next().unwrap().parse().unwrap();
        scanner.insert((x, y, z));
    }
    scanners.push_back(scanner);

    let (p1, p2) = accumulate(&mut scanners);
    let mut hs: Scanner = HashSet::new();
    for v in &p1 {
        hs.extend(v);
    }

    println!("Day 19 - Part 1: {:?}", hs.len());

    let mut highest = 0;
    for i in 0..p2.len() - 1 {
        for j in i + 1..p2.len() {
            let dist =
                (p2[i].0 - p2[j].0).abs() + (p2[i].1 - p2[j].1).abs() + (p2[i].2 - p2[j].2).abs();
            highest = highest.max(dist);
        }
    }
    println!("Day 19 - Part 2: {:?}", highest);
}

fn accumulate(list: &mut VecDeque<Scanner>) -> (Vec<Scanner>, Vec<ScanData>) {
    let mut known: Vec<Scanner> = Vec::new();
    let mut offsets: Vec<ScanData> = Vec::new();

    known.push(list.pop_front().unwrap());

    while !list.is_empty() {
        let mut found = None;
        let mut idx = 0;
        'outer: for (e, mut check) in list.iter_mut().enumerate() {
            for k in &known {
                if let Some(offset) = rotate_and_compare(k, &mut check) {
                    found = Some(check);
                    offsets.push(offset);
                    idx = e;
                    break 'outer;
                }
            }
        }

        if found.is_none() {
            panic!("no merge found");
        }

        let found = found.unwrap().clone();
        known.push(found);
        list.remove(idx);
        println!("{} left to go", list.len());
    }

    (known, offsets)
}

fn rotate_around_y(
    known: &Scanner,
    b: &mut Vec<ScanData>,
    check: &mut Scanner,
) -> Option<ScanData> {
    for _ in 0..4 {
        if let Some(offset) = alignment(known, b) {
            check.clear();
            for point in b {
                check.insert((point.0 + offset.0, point.1 + offset.1, point.2 + offset.2));
            }
            return Some(offset);
        }
        // rotate on y: x becomes -z, z becomes x
        for point in b.iter_mut() {
            *point = (point.2, point.1, -point.0);
        }
    }
    None
}

fn rotate_and_compare(known: &Scanner, check: &mut Scanner) -> Option<ScanData> {
    let mut b: Vec<ScanData> = check.iter().copied().collect();

    for _ in 0..4 {
        if let Some(offset) = rotate_around_y(known, &mut b, check) {
            return Some(offset);
        }
        // rotate around z: x becomes -y, y becomes x
        for point in b.iter_mut() {
            *point = (point.1, -point.0, point.2);
        }
    }

    for x in 0..2 {
        if x == 0 {
            // rotate top to the front
            for point in b.iter_mut() {
                *point = (point.0, -point.2, point.1);
            }
        } else {
            // rotate twice on x
            for point in b.iter_mut() {
                *point = (point.0, -point.1, -point.2);
            }
        }
        if let Some(offset) = rotate_around_y(known, &mut b, check) {
            return Some(offset);
        }
    }

    None
}

fn alignment(known: &Scanner, check: &[ScanData]) -> Option<ScanData> {
    for pk in known {
        for pc in check {
            let dx = pk.0 - pc.0;
            let dy = pk.1 - pc.1;
            let dz = pk.2 - pc.2;

            let mut count = 0;
            for point in check {
                if known.contains(&(point.0 + dx, point.1 + dy, point.2 + dz)) {
                    count += 1;
                    if count >= 12 {
                        return Some((dx, dy, dz));
                    }
                }
            }
        }
    }

    None
}
