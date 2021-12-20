use crate::utils::read_file;

// ......
// .0..#.
// .#....
// .##..#
// ...#..
// ...###

use std::collections::HashSet;

type Map = HashSet<(i64, i64)>;

struct Image {
    infinity: bool,
    map: Map,
    width: usize,
    height: usize,
}

pub fn day20() {
    let lines = read_file("inputs/input-day20.txt");

    let mut bits: Vec<bool> = Vec::new();
    for c in lines[0].chars() {
        bits.push(c == '#');
    }

    let width = lines[2].len();
    let height = lines.len() - 2;

    let mut map: Map = HashSet::new();
    for (y, l) in lines.iter().enumerate().skip(2) {
        for (x, c) in l.chars().enumerate() {
            if c == '#' {
                map.insert((x as i64, y as i64 - 2));
            }
        }
    }

    let mut img = Image {
        infinity: false,
        map,
        width,
        height,
    };
    enhance(&mut img, &bits);
    enhance(&mut img, &bits);
    println!("Day 20 - Part 1: {}", img.map.len());

    for _ in 0..48 {
        enhance(&mut img, &bits);
    }
    println!("Day 20 - Part 2: {}", img.map.len());
}

fn enhance(img: &mut Image, bits: &[bool]) {
    let mut map: Map = HashSet::new();
    const FIELD: [((i64, i64), usize); 9] = [
        ((-1, -1), 256),
        ((0, -1), 128),
        ((1, -1), 64),
        ((-1, 0), 32),
        ((0, 0), 16),
        ((1, 0), 8),
        ((-1, 1), 4),
        ((0, 1), 2),
        ((1, 1), 1),
    ];

    for y in -1i64..=img.height as i64 {
        for x in -1i64..=img.width as i64 {
            let mut value = 0;
            for delta in FIELD {
                let px = x + delta.0 .0;
                let py = y + delta.0 .1;
                if px < 0 || py < 0 || px >= img.width as i64 || py >= img.height as i64 {
                    if img.infinity {
                        value += delta.1;
                    }
                } else if img.map.contains(&(px, py)) {
                    value += delta.1;
                }
            }
            if bits[value] {
                map.insert((x + 1, y + 1));
            }
        }
    }

    img.width += 2;
    img.height += 2;
    img.map = map;
    if img.infinity {
        img.infinity = bits[511];
    } else {
        img.infinity = bits[0];
    }
}
