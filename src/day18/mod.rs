use crate::utils::read_file;

use std::collections::VecDeque;

type SnailNum = VecDeque<Tok>;

#[derive(Debug, PartialEq)]
enum Tok {
    Start,
    Left,
    Right,
    Num(u64),
}

pub fn day18() {
    let lines = read_file("inputs/input-day18.txt");
    let mut nums: VecDeque<SnailNum> = VecDeque::new();

    for l in lines {
        nums.push_back(convert(&l));
    }

    let mut result = nums.pop_front().unwrap();
    while !nums.is_empty() {
        let mut next_num = nums.pop_front().unwrap();
        add(&mut result, &mut next_num);
        process(&mut result);
    }
    println!("Day 18 - Part 1: {}", mag(&mut result));
}

fn convert(l: &str) -> SnailNum {
    let mut data: SnailNum = VecDeque::from(vec![Tok::Start]);

    for c in l.chars() {
        match c {
            '[' => data.push_back(Tok::Left),
            ']' => data.push_back(Tok::Right),
            '0'..='9' => data.push_back(Tok::Num((c as u8 - b'0') as u64)),
            'a'..='f' => data.push_back(Tok::Num((c as u8 - b'a' + 10) as u64)),
            ',' => {}
            _ => panic!("bad input"),
        }
    }

    data
}

fn stringify(num: &SnailNum) -> String {
    let mut s = String::new();
    for tok in num {
        match tok {
            Tok::Start => s.push('>'),
            Tok::Left => s.push('['),
            Tok::Right => s.push(']'),
            Tok::Num(n) => s = format!("{} {}", s, n),
        }
    }

    s
}

fn explode(num: &mut SnailNum) -> bool {
    let m = num.pop_front().unwrap();
    if m != Tok::Start {
        panic!("snail num does not start with a start");
    }
    num.push_back(m);

    let mut exploded = false;
    let mut nest = 0;
    'out: loop {
        let t = num.pop_front().unwrap();
        match t {
            Tok::Left => {
                nest += 1;
                if nest >= 5 {
                    // do explode
                    let left_tok = num.pop_front().unwrap();
                    let right_tok = num.pop_front().unwrap();
                    let right_bracket = num.pop_front().unwrap();
                    if right_bracket != Tok::Right {
                        panic!("nest is 5 deep and we don't have a right bracket {:?}", right_bracket);
                    }
                    
                    // add rhs number to next number in the list
                    if let Tok::Num(right) = right_tok {
                        for next in num.iter_mut() {
                            match next {
                                Tok::Num(n) => {
                                    *next = Tok::Num(*n + right);
                                    break;
                                },
                                Tok::Start => {
                                    break;
                                },
                                _ => {},
                            }
                        }
                    } else {
                        panic!("rhs was not a number");
                    }

                    // replace the current pair with 0
                    num.push_front(Tok::Num(0));

                    if let Tok::Num(left) = left_tok {
                        loop {
                            let next = num.pop_back().unwrap();
                            match next {
                                Tok::Start => {
                                    num.push_front(next);
                                    break;
                                },
                                Tok::Num(n) => {
                                    num.push_back(Tok::Num(n + left));
                                    break;
                                }
                                _ => num.push_front(next),
                            }
                        }
                    } else {
                        panic!("lhs was not a number");
                    }
                    exploded = true;
                    break 'out;
                } else {
                    num.push_back(t);
                }
            },
            Tok::Right => {
                nest -= 1;
                num.push_back(t);
            }
            Tok::Num(_) => {
                num.push_back(t);
            }
            Tok::Start => {
                num.push_front(t);
                break 'out;
            }
        }
    }

    while let Some(t) = num.front() {
        if *t == Tok::Start {
            break;
        } else {
            num.rotate_left(1);
        }
    }

    exploded
}

fn split(num: &mut SnailNum) -> bool {
    let mut split = false;

    let m = num.pop_front().unwrap();
    if m != Tok::Start {
        panic!("snail num does not start with a start");
    }
    num.push_back(m);

    loop {
        let t = num.pop_front().unwrap();
        match t {
            Tok::Num(n) => {
                if n > 9 {
                    let left = n / 2;
                    let right = n - left;
                    num.push_back(Tok::Left);
                    num.push_back(Tok::Num(left));
                    num.push_back(Tok::Num(right));
                    num.push_back(Tok::Right);
                    split = true;
                    break;
                } else {
                    num.push_back(t);
                }
            }
            Tok::Start => {
                num.push_front(t);
                break;
            }
            _ => {
                num.push_back(t);
            }
        }
    }

    while let Some(t) = num.front() {
        if *t == Tok::Start {
            break;
        } else {
            num.rotate_left(1);
        }
    }

    split
}

fn process(num: &mut SnailNum) {
    loop {
        while explode(num) {}
        if !split(num) {
            break;
        }
    }
}

fn add(lhs: &mut SnailNum, rhs: &mut SnailNum) {
    let m1 = lhs.pop_front().unwrap();
    if m1 != Tok::Start {
        panic!("Adding lhs invalid num");
    }

    let m1 = rhs.pop_front().unwrap();
    if m1 != Tok::Start {
        panic!("Adding lhs invalid num");
    }

    lhs.push_front(Tok::Left);
    lhs.push_front(m1);
    lhs.append(rhs);
    lhs.push_back(Tok::Right);
}

fn mag(num: &mut SnailNum) -> u64 {
    while num.len() > 4 {       // Start Left Ans Right
        let m = num.pop_front().unwrap();
        if m != Tok::Start {
            panic!("snail num does not start with a start");
        }
        num.push_back(m);

        let mut was_num = false;
        loop {
            let t = num.pop_front().unwrap();
            match t {
                Tok::Left | Tok::Right => {
                    num.push_back(t);
                    was_num = false;
                },
                Tok::Num(n) => {
                    if was_num {
                        if let Tok::Num(lhs) = num.pop_back().unwrap() {
                            let mag = Tok::Num(3 * lhs + 2 * n);
                            num.pop_back();     // remove the left bracket
                            num.pop_front();    // remove the right bracket
                            num.push_back(mag);
                            was_num = false;
                        }
                    } else {
                        was_num = true;
                        num.push_back(t);
                    }
                }
                Tok::Start => {
                    num.push_front(t);
                    break;
                }
            }
        }
    }

    if let Tok::Num(answer) = num.pop_back().unwrap() {
        return answer;
    }

    0
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn explode_1() {
        let mut num = convert("[[[[[9,8],1],2],3],4]");
        assert_eq!(stringify(&num), ">[[[[[ 9 8] 1] 2] 3] 4]");
        assert!(explode(&mut num));
        assert_eq!(stringify(&num), ">[[[[ 0 9] 2] 3] 4]");
    }

    #[test]
    fn split_1() {
        let mut num = convert("[[[[0,7],4],[f,[0,d]]],[1,1]]");
        assert!(split(&mut num));
        assert_eq!(stringify(&num), ">[[[[ 0 7] 4][[ 7 8][ 0 13]]][ 1 1]]");
    }

    #[test]
    fn process_1() {
        let mut num = convert("[[[[[4,3],4],4],[7,[[8,4],9]]],[1,1]]");
        process(&mut num);
        assert_eq!(stringify(&num), ">[[[[ 0 7] 4][[ 7 8][ 6 0]]][ 8 1]]");
    }

    #[test]
    fn add_1() {
        let mut lhs = convert("[[[[4,3],4],4],[7,[[8,4],9]]]");
        let mut rhs = convert("[1,1]");
        add(&mut lhs, &mut rhs);
        process(&mut lhs);
        assert_eq!(stringify(&lhs), ">[[[[ 0 7] 4][[ 7 8][ 6 0]]][ 8 1]]");
    }

    #[test]
    fn magnitude_0() {
        let mut num = convert("[[1,2],[[3,4],5]]");
        assert_eq!(mag(&mut num), 143);
    }

    #[test]
    fn magnitude_1() {
        let mut num = convert("[[[[0,7],4],[[7,8],[6,0]]],[8,1]]");
        assert_eq!(mag(&mut num), 1384);
    }

    #[test]
    fn magnitude_2() {
        let mut num = convert("[[[[1,1],[2,2]],[3,3]],[4,4]]");
        assert_eq!(mag(&mut num), 445);
    }

    #[test]
    fn magnitude_3() {
        let mut num = convert("[[[[3,0],[5,3]],[4,4]],[5,5]]");
        assert_eq!(mag(&mut num), 791);
    }

    #[test]
    fn magnitude_4() {
        let mut num = convert("[[[[5,0],[7,4]],[5,5]],[6,6]]");
        assert_eq!(mag(&mut num), 1137);
    }

    #[test]
    fn magnitude_5() {
        let mut num = convert("[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]");
        assert_eq!(mag(&mut num), 3488);
    }
}
