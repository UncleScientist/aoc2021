use crate::utils::read_file;

pub fn day04() {
    let lines = read_file("inputs/input-day04.txt");

    let (nlist, mut blist) = parse_boards(&lines);
    println!("Day 04 - Part 1: {}", find_winner(nlist, &mut blist));
}

fn find_winner(numbers: Vec<i32>, boards: &mut Vec<Board>) -> i32 {
    for call in numbers {
        for b in &mut *boards {
            if let Some(winner) = b.call_number(call) {
                return winner;
            }
        }
    }
    0
}

fn parse_boards(data: &[String]) -> (Vec<i32>, Vec<Board>) {
    let numbers: Vec<i32> = data[0].split(',').map(|x| x.parse().unwrap()).collect();
    let mut boards: Vec<Board> = Vec::new();

    let mut index = 2;
    while index < data.len() {
        let mut grid = Vec::new();
        for i in 0..5 {
            let row: Vec<i32> = data[index + i]
                .split_whitespace()
                .map(|x| x.parse().unwrap())
                .collect();
            for n in row {
                grid.push(n);
            }
        }
        boards.push(Board::new(grid, 5));
        index += 6;
    }

    (numbers, boards)
}

struct Board {
    grid: Vec<i32>,
    size: usize,
}

impl Board {
    pub fn new(grid: Vec<i32>, size: usize) -> Board {
        Board { grid, size }
    }

    #[cfg(test)]
    pub fn get_grid(&self) -> &Vec<i32> {
        &self.grid
    }

    pub fn call_number(&mut self, called: i32) -> Option<i32> {
        if let Some(loc) = self.grid.iter().position(|loc| *loc == called) {
            self.grid[loc] = -1;

            let row = loc / self.size;
            let col = loc % self.size;

            let mut rsum = 0;
            let mut csum = 0;
            for i in 0..self.size {
                rsum += self.grid[row * self.size + i];
                csum += self.grid[i * self.size + col];
            }

            if rsum == -5 || csum == -5 {
                // TODO: fix size for -5
                let sum: i32 = self.grid.iter().filter(|&x| *x != -1).sum();
                return Some(sum * called);
            }
        }

        None
    }

    #[cfg(test)]
    pub fn print(&self) {
        for row in 0..self.size {
            for col in 0..self.size {
                print!(" {:2}", self.grid[row * self.size + col]);
            }
            println!();
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_winner() {
        let input_data = vec![
            "7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1".to_string(),
            "".to_string(),
            "22 13 17 11  0".to_string(),
            " 8  2 23  4 24".to_string(),
            "21  9 14 16  7".to_string(),
            " 6 10  3 18  5".to_string(),
            " 1 12 20 15 19".to_string(),
            "".to_string(),
            " 3 15  0  2 22".to_string(),
            " 9 18 13 17  5".to_string(),
            "19  8  7 25 23".to_string(),
            "20 11 10 24  4".to_string(),
            "14 21 16 12  6".to_string(),
            "".to_string(),
            "14 21 17 24  4".to_string(),
            "10 16 15  9 19".to_string(),
            "18  8 23 26 20".to_string(),
            "22 11 13  6  5".to_string(),
            " 2  0 12  3  7".to_string(),
        ];

        let (nlist, mut blist) = parse_boards(&input_data);

        let mut won = false;
        'outer: for call in nlist {
            println!("\ncalling number {}", call);
            for b in &mut blist {
                if let Some(winner) = b.call_number(call) {
                    won = true;
                    assert_eq!(winner, 4512);
                    break 'outer;
                }
                b.print();
                println!("-");
            }
        }

        assert!(won);
    }

    #[test]
    fn test_parsing() {
        let input_data = vec![
            "7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1".to_string(),
            "".to_string(),
            "22 13 17 11  0".to_string(),
            " 8  2 23  4 24".to_string(),
            "21  9 14 16  7".to_string(),
            " 6 10  3 18  5".to_string(),
            " 1 12 20 15 19".to_string(),
            "".to_string(),
            " 3 15  0  2 22".to_string(),
            " 9 18 13 17  5".to_string(),
            "19  8  7 25 23".to_string(),
            "20 11 10 24  4".to_string(),
            "14 21 16 12  6".to_string(),
            "".to_string(),
            "14 21 17 24  4".to_string(),
            "10 16 15  9 19".to_string(),
            "18  8 23 26 20".to_string(),
            "22 11 13  6  5".to_string(),
            " 2  0 12  3  7".to_string(),
        ];

        let (nlist, blist) = parse_boards(&input_data);
        assert_eq!(
            nlist,
            vec![
                7, 4, 9, 5, 11, 17, 23, 2, 0, 14, 21, 24, 10, 16, 13, 6, 15, 25, 12, 22, 18, 20, 8,
                19, 3, 26, 1
            ]
        );
        assert_eq!(
            blist[0].get_grid(),
            &vec![
                22i32, 13, 17, 11, 0, 8, 2, 23, 4, 24, 21, 9, 14, 16, 7, 6, 10, 3, 18, 5, 1, 12,
                20, 15, 19
            ]
        );
    }
}
