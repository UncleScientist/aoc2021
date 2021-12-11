use crate::utils::read_file;

struct Grid {
    grid: Vec<u8>,
    width: usize,
}

pub fn day11() {
    let lines = read_file("inputs/input-day11.txt");

    let width = lines[0].len();
    let mut grid: Vec<u8> = Vec::new();

    for l in lines {
        for c in l.chars() {
            grid.push(c as u8 - b'0');
        }
    }

    let mut octos = Grid { grid, width };
    let mut part1 = 0;
    let all_flash = octos.grid.len();
    let mut part2 = 0;

    let mut part1_done = false;
    let mut part2_done = false;
    let mut steps = 0;

    while !part1_done || !part2_done {
        let flash_count = octos.step();
        steps += 1;

        if steps <= 100 {
            part1 += flash_count;
        } else {
            part1_done = true;
        }

        if flash_count == all_flash {
            part2 = steps;
            part2_done = true;
        }
    }

    println!("Day 11 - Part 1: {}", part1);
    println!("Day 11 - Part 2: {}", part2);
}

// 8 directions: N, NE, E, SE, S, SW, W, NW
const DIRS: [(i32, i32); 8] = [
    (0, -1),
    (1, -1),
    (1, 0),
    (1, 1),
    (0, 1),
    (-1, 1),
    (-1, 0),
    (-1, -1),
];

impl Grid {
    // convert index to x,y coord
    fn xy(&self, idx: usize) -> (i32, i32) {
        ((idx % self.width) as i32, (idx / self.width) as i32)
    }

    // convert x,y coord to index
    fn idx(&self, x: i32, y: i32) -> Option<usize> {
        if x < 0 || y < 0 || x >= self.width as i32 || y >= self.width as i32 {
            None
        } else {
            Some((y * self.width as i32 + x) as usize)
        }
    }

    // do one step of the problem
    fn step(&mut self) -> usize {
        let mut flashers: Vec<(i32, i32)> = Vec::new();
        let mut flashed = 0;

        // find all octopusses which will flash this turn
        for i in 0..self.grid.len() {
            self.grid[i] += 1;
            if self.grid[i] > 9 {
                flashers.push(self.xy(i));
            }
        }

        // find any octopusses triggered by the above flashers
        while !flashers.is_empty() {
            let (x, y) = flashers.pop().unwrap();
            flashed += 1;

            for d in DIRS {
                let (lx, ly) = (x + d.0, y + d.1);
                if let Some(idx) = self.idx(lx, ly) {
                    if self.grid[idx] <= 9 {
                        self.grid[idx] += 1;
                        if self.grid[idx] > 9 {
                            flashers.push((lx, ly));
                        }
                    }
                }
            }
        }

        // set any flashed octopusses to 0
        for i in 0..self.grid.len() {
            if self.grid[i] > 9 {
                self.grid[i] = 0;
            }
        }

        flashed
    }
}
