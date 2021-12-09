use std::collections::HashSet;

use crate::utils::read_file;

pub fn day09() {
    let lines = read_file("inputs/input-day09.txt");

    let width = lines[0].len();
    let height = lines.len();
    let mut grid: Vec<u8> = Vec::new();
    for l in lines {
        for c in l.chars() {
            grid.push(c as u8 - b'0');
        }
    }
    let floor = Floor {
        grid,
        width,
        height,
    };
    let mut sum: usize = 0;
    let mut sizes: Vec<usize> = Vec::new();
    for x in 0..width {
        for y in 0..height {
            if floor.is_lowest(x, y) {
                sizes.push(floor.basin_size(x, y));
                sum += floor.get(x, y) as usize + 1;
            }
        }
    }
    println!("Day 09 - Part 1: {}", sum);

    sizes.sort_by(|a, b| b.cmp(a));
    println!(
        "Day 09 - Part 2: {}",
        sizes.iter().take(3).product::<usize>()
    );
}

#[derive(Debug)]
struct Floor {
    grid: Vec<u8>,
    width: usize,
    height: usize,
}

impl Floor {
    fn get(&self, x: usize, y: usize) -> u8 {
        self.grid[y * self.width + x]
    }

    fn is_lowest(&self, x: usize, y: usize) -> bool {
        let surrounds = self.surrounding(x, y);
        let cur = self.grid[y * self.width + x];
        for s in surrounds {
            let idx = s.1 * self.width + s.0;
            if cur >= self.grid[idx] {
                return false;
            }
        }
        true
    }

    fn surrounding(&self, x: usize, y: usize) -> Vec<(usize, usize)> {
        let mut result = Vec::new();
        if x > 0 {
            result.push((x - 1, y));
        }
        if x < self.width - 1 {
            result.push((x + 1, y));
        }
        if y > 0 {
            result.push((x, y - 1));
        }
        if y < self.height - 1 {
            result.push((x, y + 1));
        }
        result
    }

    fn basin_size(&self, x: usize, y: usize) -> usize {
        let mut hs: HashSet<(usize, usize)> = HashSet::new();
        let mut check: Vec<(usize, usize)> = Vec::new();

        hs.insert((x, y));
        let mut x = x;
        let mut y = y;
        loop {
            // get a list of points surrounding us and filter out
            // any that we have already visited
            let all_sur = self.surrounding(x, y);
            let sur = all_sur
                .iter()
                .filter(|x| !hs.contains(x))
                .copied()
                .collect::<Vec<(usize, usize)>>();

            // check each point to see if it's in the basin, e.g. not a "9"
            // and add it to a list of points to check further
            for s in sur {
                if self.get(s.0, s.1) != 9 {
                    check.push(s);
                    hs.insert(s);
                }
            }

            // if there are more points to check, then grab one, otherwise
            // we're done and the number of points is the length of the visited
            // locations hashmap
            if let Some(next) = check.pop() {
                x = next.0;
                y = next.1;
            } else {
                return hs.len();
            }
        }
    }
}
