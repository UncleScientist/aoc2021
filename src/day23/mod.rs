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

type Rooms = [[Amphipod; 2]; 4];
type Hallway = [Amphipod; 11];

#[derive(PartialEq, Eq, Hash, Copy, Clone, Debug)]
struct State(Rooms, Hallway);

impl State {
    fn neighbors(&self) -> Vec<(State, i64)> {
        let mut result: Vec<(State, i64)> = Vec::new();

        let (room, hall) = (self.0, self.1);

        // Come up with a list of moves for amphipods leaving a room and going into the hallway
        for (c, amps) in room.iter().enumerate() {
            let col = 2 + c * 2;
            for a in 0..2 {
                if amps[a] == Empty {
                    continue;
                }
                let desired = amps[a].col();
                if a == 0 && col == desired && amps[0] == amps[1] {
                    continue;
                }
                if a == 1 && col == desired {
                    continue;
                }
                let mut possible = Vec::new();
                for h in [0, 1, 3, 5, 7, 9, 10] {
                    if hall[h] != Empty {
                        if h < col {
                            possible.clear();
                        } else if h > col {
                            break;
                        }
                    } else if a == 0 || (a == 1 && amps[0] == Empty) {
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
            if room[r][0] == Empty && room[r][1] == Empty {
                let mut newroom = room.clone();
                let mut newhall = hall.clone();
                newroom[r][1] = *amp;
                newhall[c] = Empty;
                let cost = amp.cost();
                result.push((State(newroom, newhall),
                    cost * (2 + (desired as i64 - c as i64).abs())));
            } else if room[r][0] == Empty && room[r][1] == *amp {
                let mut newroom = room.clone();
                let mut newhall = hall.clone();
                newroom[r][0] = *amp;
                newhall[c] = Empty;
                let cost = amp.cost();
                result.push((State(newroom, newhall),
                    cost * (1 + (desired as i64 - c as i64).abs())));
            }
        }

        result
    }

    fn print(&self) { 
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
        for a in self.0 {
            match a[1] {
                A => print!("A#"),
                B => print!("B#"),
                C => print!("C#"),
                D => print!("D#"),
                Empty => print!(".#"),
            }
        }
        println!("\n  #########");
    }

}


pub fn day23() {
    let e: Rooms = [[A, A], [B, B], [C, C], [D, D]];
    // let r: Rooms = [[B, A], [C, D], [B, C], [D, A]]; // test data
    let r: Rooms = [[B, C], [C, D], [A, D], [B, A]]; // given data
    let h: Hallway = [Empty; 11];
    let s: State = State(r, h);
    let end_condition = State(e, h);

    /*
    let tr: Rooms = [[Empty, A], [B, B], [C, C], [D, D]];
    let th: Hallway = [Empty,Empty,Empty,Empty,Empty,Empty,Empty,Empty,Empty,Empty,A];
    let ts = State(tr, th);
    println!("{:?}", ts);
    for n in ts.neighbors() {
        println!(" > {:?}", n);
    }
    std::process::exit(0);
    */

    s.print();
    end_condition.print();

    let mut qset: HashMap<State, i64> = HashMap::new();
    let mut dist: HashMap<State, i64> = HashMap::new();
    
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
            println!("Day 23 - Part 1: {} ", smallest);
            break;
        }

        qset.remove(&found);

        for n in found.neighbors() {
            // n.0.print();
            let alt = smallest + n.1;
            if alt < *dist.get(&n.0).unwrap_or(&i64::MAX) {
                dist.insert(n.0.clone(), alt);
                qset.insert(n.0.clone(), alt);
            }
        }
    }
}
