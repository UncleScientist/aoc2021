use std::collections::HashMap;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
enum Amphipod {
    Empty,
    A,
    B,
    C,
    D,
}
use Amphipod::*;

impl Amphipod {
    fn cost(&self) -> i64 {
        match self {
            A => 1,
            B => 10,
            C => 100,
            D => 1000,
            Empty => panic!("no cost for a non-amphipod"),
        }
    }

    fn col(&self) -> usize {
        match self {
            A => 2,
            B => 4,
            C => 6,
            D => 8,
            Empty => panic!("no column for a non-ampipod"),
        }
    }

}

type Rooms = [[Amphipod; 4]; 4];
type Hallway = [Amphipod; 11];

#[derive(PartialEq, Eq, Hash, Copy, Clone, Debug)]
struct State(Rooms, Hallway);

impl State {
    fn neighbors(&self, depth: usize) -> Vec<(State, i64)> {
        let mut result: Vec<(State, i64)> = Vec::new();

        let (room, hall) = (self.0, self.1);

        // Come up with a list of moves for amphipods leaving a room and going into the hallway
        for (c, amps) in room.iter().enumerate() {
            let col = 2 + c * 2;
            let mut count = 0;
            for a in 0..depth {
                if amps[a] == Empty || amps[a].col() == col {
                    count += 1;
                }
            }
            if count == depth {
                continue;
            }

            for a in 0..depth {
                if room[c][a] == Empty {
                    continue;
                }

                let mut possible = Vec::new();
                'hallway: for h in [0, 1, 3, 5, 7, 9, 10] {
                    if hall[h] != Empty {
                        if h < col {
                            possible.clear();
                        } else if h > col {
                            break;
                        }
                    } else {
                        for check in 0..a {
                            if room[c][check] != Empty {
                                continue 'hallway;
                            }
                        }
                        possible.push((h, a));
                    }
                }
                for p in possible {
                    let mut newroom = room.clone();
                    let mut newhall = hall.clone();
                    let cost = newroom[c][p.1].cost();
                    newhall[p.0] = newroom[c][p.1];
                    newroom[c][p.1] = Empty;
                    let start: i64 = col as i64;
                    let end: i64 = p.0 as i64;
                    result.push((State(newroom, newhall),
                        cost * (p.1 + 1 + (start - end).abs() as usize) as i64));
                }
            }
        }

        // Come up with a list of moves for hallway -> room
        'next: for (c, amp) in hall.iter().enumerate() {
            if *amp == Empty {
                continue;
            }
            let desired = amp.col();
            if desired < c {
                for h in desired..c {
                    if hall[h] != Empty {
                        continue 'next;
                    }
                }
            } else if desired > c {
                for h in c+1..=desired {
                    if hall[h] != Empty {
                        continue 'next;
                    }
                }
            }
            let r = desired / 2 - 1;
            let mut best = depth - 1;
            loop {
                if room[r][best] == Empty {
                    let mut newroom = room.clone();
                    let mut newhall = hall.clone();
                    newroom[r][best] = *amp;
                    newhall[c] = Empty;
                    let cost = amp.cost();
                    result.push((State(newroom, newhall),
                        cost * (best as i64 + 1 + (desired as i64 - c as i64).abs())));
                    break;
                } else if room[r][best] != *amp {
                    break;
                }

                if best == 0 {
                    break;
                }
                best -= 1;
            }
        }

        result
    }

    fn print(&self, depth: usize) { 
        print!("#############\n#");
        for h in self.1.iter() {
            match h {
                A => print!("A"),
                B => print!("B"),
                C => print!("C"),
                D => print!("D"),
                Empty => print!("."),
            }
        }
        print!("#\n###");
        for a in self.0 {
            match a[0] {
                A => print!("A#"),
                B => print!("B#"),
                C => print!("C#"),
                D => print!("D#"),
                Empty => print!(".#"),
            }
        }
        print!("##\n  #");
        for d in 1..depth {
            for a in self.0 {
                match a[d] {
                    A => print!("A#"),
                    B => print!("B#"),
                    C => print!("C#"),
                    D => print!("D#"),
                    Empty => print!(".#"),
                }
            }
            print!("\n  #");
        }
        println!("########");
    }

}


pub fn day23() {
    let end_part1: Rooms = [[A, A, Empty, Empty], [B, B, Empty, Empty], [C, C, Empty, Empty], [D, D, Empty, Empty]];
    let end_part2: Rooms = [[A, A, A, A], [B, B, B, B], [C, C, C, C], [D, D, D, D]];
    // let r: Rooms = [[B, D, D, A], [C, C, B, D], [B, B, A, C], [D, A, C, A]]; // test data
    let start_part1: Rooms = [[B, C, Empty, Empty], [C, D, Empty, Empty], [A, D, Empty, Empty], [B, A, Empty, Empty]]; // given data
    let start_part2: Rooms = [[B, D, D, C], [C, C, B, D], [A, B, A, D], [B, A, C, A]]; // given data

    solve(&start_part1, &end_part1, 2);
    solve(&start_part2, &end_part2, 4);
}

fn solve(start: &Rooms, end: &Rooms, depth: usize) {
    let mut qset: HashMap<State, i64> = HashMap::new();
    let mut dist: HashMap<State, i64> = HashMap::new();
    let mut prev: HashMap<State, State> = HashMap::new();

    let s: State = State(*start, [Empty; 11]);
    let end_condition = State(*end, [Empty; 11]);

    qset.insert(s, 0);

    while !qset.is_empty() {
        let mut smallest = i64::MAX;
        let mut found = None;
        for (s, c) in qset.iter() {
            if smallest > *c {
                smallest = *c;
                found = Some(s.clone());
            }
        }

        if found.is_none() {
            panic!("could not find solution");
        }

        let found = found.unwrap();
        if found == end_condition {
            println!("Day 23 - Part {}: {} ", depth / 2, smallest);
            break;
        }

        qset.remove(&found);

        for n in found.neighbors(depth) {
            let alt = smallest + n.1;
            if alt < *dist.get(&n.0).unwrap_or(&i64::MAX) {
                dist.insert(n.0.clone(), alt);
                qset.insert(n.0.clone(), alt);
                prev.insert(n.0.clone(), found.clone());
            }
        }
    }
}
