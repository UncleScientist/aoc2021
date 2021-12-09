use crate::utils::read_file;

pub fn day09() {
    let lines = read_file("inputs/input-day09.txt");

    let width = lines[0].len();
    let height = lines.len();
    let mut grid: Vec<u8> = Vec::new();
    for l in lines {
        for c in l.chars() {
            grid.push(c as u8 - '0' as u8);
        }
    }
    let floor = Floor { grid, width, height };
    let mut sum : usize = 0;
    for x in 0..width {
        for y in 0..height {
            if floor.is_lowest(x, y) {
                sum += floor.get(x, y) as usize + 1;
            }
        }
    }
    println!("Day 09 - Part 1: {}", sum);
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
            if cur >= self.grid[s] {
                return false;
            }
        }
        true
    }

    fn surrounding(&self, x: usize, y: usize) -> Vec<usize> {
        let idx = y * self.width + x;
        let mut result = Vec::new();
        if x > 0 {
            result.push(idx - 1);
        }
        if x < self.width - 1 {
            result.push(idx + 1);
        }
        if y > 0 {
            result.push(idx - self.width);
        }
        if y < self.height - 1 {
            result.push(idx + self.width);
        }
        result
    }
}
