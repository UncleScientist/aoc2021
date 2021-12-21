pub fn day21() {
    let mut p1pos = 4;
    let mut p2pos = 8;

    let mut rolls = 0;

    let mut p1score = 0;
    let mut p2score = 0;

    let mut value: i64 = -3;

    while p1score < 1000 && p2score < 1000 {
        value += 9;
        rolls += 3;
        p1pos = (p1pos + value) % 10;
        if p1pos == 0 {
            p1pos = 10;
        }
        p1score += p1pos;
        if p1score >= 1000 {
            break;
        }

        value += 9;
        rolls += 3;
        p2pos = (p2pos + value) % 10;
        if p2pos == 0 {
            p2pos = 10;
        }
        p2score += p2pos;
    }
    //println!("player 1 lands on {}, and has a score of {}", p1pos, p1score);
    //println!("player 2 lands on {}, and has a score of {}", p2pos, p2score);
    //println!("total rolls = {}", rolls);
    println!("Day 21 - Part 1: {}", rolls * p1score.min(p2score));

    let mut p1wins = 0;
    let mut p2wins = 0;
    part2((0, 0), 1, true, (3, 5), &mut p1wins, &mut p2wins);
    println!("Day 21 - Part 2: {}", p1wins.max(p2wins));
}

fn part2(
    scores: (i64, i64),
    universes: u64,
    p1turn: bool,
    positions: (i64, i64),
    p1wins: &mut u64,
    p2wins: &mut u64,
) {
    const UNI: [u64; 7] = [1, 3, 6, 7, 6, 3, 1];
    let p1score = scores.0;
    let p2score = scores.1;
    let p1pos = positions.0;
    let p2pos = positions.1;

    if p1score >= 21 {
        *p1wins += universes;
        return;
    }

    if p2score >= 21 {
        *p2wins += universes;
        return;
    }

    for (i, u) in UNI.iter().enumerate() {
        if p1turn {
            let mut p1next = (p1pos + (i as i64 + 3)) % 10;
            if p1next == 0 {
                p1next = 10;
            }
            part2(
                (p1score + p1next, p2score),
                universes * u,
                !p1turn,
                (p1next, p2pos),
                p1wins,
                p2wins,
            );
        } else {
            let mut p2next = (p2pos + (i as i64 + 3)) % 10;
            if p2next == 0 {
                p2next = 10;
            }
            part2(
                (p1score, p2score + p2next),
                universes * u,
                !p1turn,
                (p1pos, p2next),
                p1wins,
                p2wins,
            );
        }
    }
}

//  3  4  5  6  7  8  9
// [1, 3, 6, 7, 6, 3, 1]
//
// if score > 21:
//    wins += universes
// for each choice:
//    increase the score by the (choice % 10) { if zero then ten }
//    multiply number of universes by outcomes[choice]
