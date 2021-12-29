use crate::utils::read_file;

use regex::Regex;

#[derive(Debug)]
enum Inst {
    Inp(usize),
    AddVar(usize, usize),
    AddNum(usize, i64),
    MulVar(usize, usize),
    MulNum(usize, i64),
    DivVar(usize, usize),
    DivNum(usize, i64),
    ModVar(usize, usize),
    ModNum(usize, i64),
    EqlVar(usize, usize),
    EqlNum(usize, i64),
}
use Inst::*;

struct Alu {
    instructions: Vec<Inst>,
    reg: [i64; 4],
}

impl Alu {
    fn new(instructions: Vec<Inst>) -> Alu {
        Alu { instructions, reg: [0, 0, 0, 0] }
    }

    fn run(&mut self, inp: &[i64]) -> Option<(usize, i64)> {
        self.reg[0] = 0;
        self.reg[1] = 0;
        self.reg[2] = 0;
        self.reg[3] = 0;

        let mut cur = 0;
        for i in &self.instructions {
            match i {
                Inp(v) => {
                    self.reg[*v] = inp[cur];
                    cur += 1;
                },
                AddVar(a, b) => { self.reg[*a] += self.reg[*b] },
                AddNum(a, n) => {
                    let looking = self.reg[1] + *n;
                    if *a == 1 && *n < 0 && looking >= 1 && looking <= 9 && self.reg[0] != looking {
                        return Some((cur, looking))
                    }
                    self.reg[*a] += *n;
                },
                MulVar(a, b) => { self.reg[*a] *= self.reg[*b] },
                MulNum(a, n) => { self.reg[*a] *= *n },
                DivVar(a, b) => { self.reg[*a] /= self.reg[*b] },
                DivNum(a, n) => { self.reg[*a] /= *n },
                ModVar(a, b) => { self.reg[*a] %= self.reg[*b] },
                ModNum(a, n) => { self.reg[*a] %= *n },
                EqlVar(a, b) => {
                    self.reg[*a] = (self.reg[*a] == self.reg[*b]) as i64;
                },
                EqlNum(a, n) => { self.reg[*a] = (self.reg[*a] == *n) as i64 },
            }
        }

        None
    }
}

pub fn day24() {
    let data = read_file("inputs/input-day24.txt");
    let mut vec = Vec::new();

    let varnum = Regex::new(r"(?P<var>.) (?P<num>-?\d+)").unwrap();
    let varvar = Regex::new(r"(?P<var1>.) (?P<var2>.)").unwrap();

    let mut inc: Vec<bool> = Vec::new();

    for d in data {
        let (inst, rest) = d.split_once(' ').unwrap();
        
        if inst == "inp" {
            let v = (rest.chars().nth(0).unwrap() as u8 - b'w') as usize;
            vec.push(Inst::Inp(v));
            continue;
        }

        if let Some(vn_cap) = varnum.captures(rest) {
            let v: usize = (vn_cap["var"].chars().nth(0).unwrap() as u8 - b'w') as usize;
            let n: i64 = vn_cap["num"].parse().unwrap();
            match inst {
                "add" => { 
                    vec.push(Inst::AddNum(v, n));
                    if v == 1 {
                        inc.push(n >= 0);
                    }
                },
                "mul" => vec.push(Inst::MulNum(v, n)),
                "div" => vec.push(Inst::DivNum(v, n)),
                "mod" => vec.push(Inst::ModNum(v, n)),
                "eql" => vec.push(Inst::EqlNum(v, n)),
                _ => panic!("invalid var/num instruction: {}", inst),
            }
        } else if let Some(vv_cap) = varvar.captures(rest) {
            let v1: usize = (vv_cap["var1"].chars().nth(0).unwrap() as u8 - b'w') as usize;
            let v2: usize = (vv_cap["var2"].chars().nth(0).unwrap() as u8 - b'w') as usize;
            match inst {
                "add" => vec.push(Inst::AddVar(v1, v2)),
                "mul" => vec.push(Inst::MulVar(v1, v2)),
                "div" => vec.push(Inst::DivVar(v1, v2)),
                "mod" => vec.push(Inst::ModVar(v1, v2)),
                "eql" => vec.push(Inst::EqlVar(v1, v2)),
                _ => panic!("invalid var/var instruction: {}", inst),
            }
        }
    }

    let mut alu = Alu::new(vec);
    let mut input = vec![9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9];
    let answer = 'next_trial: loop {
        let mut trial = input.clone();
        loop {
            if let Some(r) = alu.run(&trial) {
                trial[r.0 - 1] = r.1;
            } else {
                break;
            }
        }
        if alu.reg[3] != 0 {
            let mut i = 0;
            while i < input.len() {
                while !inc[i] {
                    i += 1;
                }
                if input[i] > 1 {
                    input[i] -= 1;
                    continue 'next_trial;
                }
                input[i] = 9;
                i += 1;
            }
            if i >= input.len() {
                println!("no value found");
            }
        } else {
            break trial;
        }
    };

    print!("Day 24 - Part 1: ");
    for a in answer {
        print!("{}", a);
    }
    println!();

    let mut input = vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1];
    let answer = 'next_trial2: loop {
        let mut trial = input.clone();
        loop {
            if let Some(r) = alu.run(&trial) {
                trial[r.0 - 1] = r.1;
            } else {
                break;
            }
        }
        if alu.reg[3] != 0 {
            let mut i = input.len() - 1;
            while i >= 0 {
                while !inc[i] {
                    i -= 1;
                }
                if input[i] < 9 {
                    input[i] += 1;
                    continue 'next_trial2;
                }
                input[i] = 1;
                i -= 1;
            }
        } else {
            break trial;
        }
    };

    print!("Day 24 - Part 2: ");
    for a in answer {
        print!("{}", a);
    }
    println!();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_alu() {
        let vec = vec![Inp(1), Inp(2), AddVar(1, 2)];
        let mut alu = Alu::new(vec);
        alu.run(vec![5, 7]);
        assert_eq!(alu.reg, [0, 12, 7, 0]);
    }

}
