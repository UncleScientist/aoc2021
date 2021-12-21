pub fn day21() {
    let mut p1pos = 3;
    let mut p2pos = 5;

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
}
