use crate::utils::read_file;
use std::collections::HashMap;

const DIRS: &[(i32, i32); 4] = &[(1, 0), (0, 1), (-1, 0), (0, -1)];

pub fn day15() {
    let lines = read_file("inputs/input-day15.txt");

    let width = lines[0].len() as i32;
    let height = lines.len() as i32;

    let mut dist: HashMap<(i32, i32), usize> = HashMap::new();
    let mut prev: HashMap<(i32, i32), Option<(i32, i32)>> = HashMap::new();
    let mut qset: HashMap<(i32, i32), usize> = HashMap::new();

    for (y, l) in lines.into_iter().enumerate() {
        for (x, c) in l.chars().enumerate() {
            dist.insert((x as i32, y as i32), std::usize::MAX);
            prev.insert((x as i32, y as i32), None);
            qset.insert((x as i32, y as i32), c as usize - '0' as usize);
        }
    }

    *dist.get_mut(&(0, 0)).unwrap() = 0;

    while !qset.is_empty() {
        let mut smallest = std::usize::MAX;
        let mut found: (i32, i32) = (0, 0);
        for k in qset.keys() {
            let dist_u = *dist.get(k).unwrap();
            if dist_u < smallest {
                smallest = dist_u;
                found = *k;
            }
        }

        qset.remove(&found);
        if found.0 == width - 1 && found.1 == height - 1 {
            break;
        }

        for d in DIRS {
            let v = (found.0 + d.0, found.1 + d.1);
            if qset.contains_key(&v) {
                let alt = smallest + qset.get(&v).unwrap();
                if alt < *dist.get(&v).unwrap() {
                    *dist.get_mut(&v).unwrap() = alt;
                    *prev.get_mut(&v).unwrap() = Some(found);
                }
            }
        }
    }

    println!(
        "prev[target] = {:?}, dist[target] = {:?}",
        prev.get(&(width - 1, height - 1)),
        dist.get(&(width - 1, height - 1))
    );
}
