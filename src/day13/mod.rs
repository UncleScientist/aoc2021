use crate::utils::read_file;

use std::collections::HashSet;

struct Grid {
    grid: HashSet<(i32, i32)>,
    width: usize,
    height: usize,
}

pub fn day13() {
    let lines = read_file("inputs/input-day13.txt");

    let mut width = 0;
    let mut height = 0;
    let mut grid = HashSet::new();
    let mut idx = 0;
    while !lines[idx].is_empty() {
        if let Some((strx, stry)) = lines[idx].split_once(',') {
            let x: i32 = strx.parse().unwrap();
            let y: i32 = stry.parse().unwrap();
            if x as usize > width {
                width = x as usize;
            }
            if y as usize > height {
                height = y as usize;
            }
            grid.insert((x, y));
        }
        idx += 1;
    }
    idx += 1; // skip blank line

    let mut paper = Grid {
        grid,
        width: width + 1,
        height: height + 1,
    };
    let mut did_part_1 = false;
    while idx < lines.len() {
        let mut split = lines[idx].split(' ');
        split.next();
        split.next();
        if let Some((axis, strline)) = split.next().unwrap().split_once('=') {
            let num: usize = strline.parse().unwrap();
            if axis == "x" {
                paper.fold_x(num);
            } else {
                paper.fold_y(num);
            }
        }
        if !did_part_1 {
            println!("Day 13 - Part 1: {}", paper.grid.len());
            did_part_1 = true;
        }
        idx += 1;
    }
    println!("Day 13 - Part 2:");
    paper.print();
}

impl Grid {
    fn print(&self) {
        for y in 0..self.height {
            for x in 0..self.width {
                if self.grid.contains(&(x as i32, y as i32)) {
                    print!("#");
                } else {
                    print!(".");
                }
            }
            println!();
        }
    }

    fn fold_y(&mut self, ypos: usize) {
        let mut newgrid = HashSet::new();
        let newheight = if ypos <= self.height / 2 {
            // bottom row becomes the new 0
            // starting height = 8
            // 0                3
            // 1                4
            // 2 --- fold ---   5   (ypos)
            // 3                4
            // 4                3
            // 5                2
            // 6                1
            // 7                0
            // new height = 6
            self.height - ypos - 1
        } else {
            // starting height = 8
            // 0                0
            // 1                1
            // 2                2
            // 3                3
            // 4                4
            // 5 --- fold ---   5 (ypos)
            // 6                4
            // 7                3
            // new height = 6
            ypos
        };

        for y in 0..self.height {
            for x in 0..self.width {
                if self.grid.contains(&(x as i32, y as i32)) {
                    let newy = ypos as i32 - i32::abs(y as i32 - ypos as i32);
                    newgrid.insert((x as i32, newy));
                }
            }
        }

        self.grid = newgrid;
        self.height = newheight;
    }

    fn fold_x(&mut self, xpos: usize) {
        let mut newgrid = HashSet::new();
        let newwidth = if xpos <= self.width / 2 {
            self.width - xpos - 1
        } else {
            xpos
        };

        for y in 0..self.height {
            for x in 0..self.width {
                if self.grid.contains(&(x as i32, y as i32)) {
                    let newx = xpos as i32 - i32::abs(x as i32 - xpos as i32);
                    newgrid.insert((newx, y as i32));
                }
            }
        }

        self.grid = newgrid;
        self.width = newwidth;
    }
}
