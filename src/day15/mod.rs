use crate::utils::read_file;
use std::collections::{BinaryHeap, HashMap};

const DIRS: &[(i32, i32); 4] = &[(1, 0), (0, 1), (-1, 0), (0, -1)];

struct Dijkstra {
    width: i32,
    height: i32,
    dist: HashMap<(i32, i32), usize>,
    prev: HashMap<(i32, i32), Option<(i32, i32)>>,
    qset: HashMap<(i32, i32), usize>,
}

struct AStar {
    width: i32,
    height: i32,
    open_set: BinaryHeap<(usize, (i32, i32))>,
    cost: HashMap<(i32, i32), usize>,
    came_from: HashMap<(i32, i32), (i32, i32)>,
    g_score: HashMap<(i32, i32), usize>,
    f_score: HashMap<(i32, i32), usize>,
}

pub fn day15() {
    let lines = read_file("inputs/input-day15.txt");

    let mut dijkstra = build_part_1(&lines);
    println!("Day 15 - Part 1: {}", dijkstra_search(&mut dijkstra));

    let mut astar = build_part_2(&lines);
    println!("Day 15 - Part 2: {}", astar_search(&mut astar));
}

fn astar_search(astar: &mut AStar) -> usize {
    loop {
        let found = astar.open_set.pop().unwrap().1;

        if found.0 == astar.width - 1 && found.1 == astar.height - 1 {
            break *astar.g_score.get(&found).unwrap() - astar.cost.get(&(0, 0)).unwrap()
                + astar
                    .cost
                    .get(&(astar.width - 1, astar.height - 1))
                    .unwrap();
        }

        for d in DIRS {
            let neighbor = (found.0 + d.0, found.1 + d.1);
            if neighbor.0 < 0
                || neighbor.1 < 0
                || neighbor.0 >= astar.width
                || neighbor.1 >= astar.height
            {
                continue;
            }
            let tentative = astar.g_score.get(&found).unwrap() + astar.cost.get(&found).unwrap();
            if tentative < *astar.g_score.get(&neighbor).unwrap() {
                astar.came_from.insert(neighbor, found);
                *astar.g_score.entry(neighbor).or_default() = tentative;
                let f_score = tentative
                    + (astar.width - neighbor.0) as usize
                    + (astar.height - neighbor.1) as usize;
                *astar.f_score.entry(neighbor).or_default() = f_score;
                astar.open_set.push((std::usize::MAX - f_score, neighbor));
            }
        }
    }
}

fn dijkstra_search(di: &mut Dijkstra) -> usize {
    while !di.qset.is_empty() {
        let mut smallest = std::usize::MAX;
        let mut found: (i32, i32) = (0, 0);
        for k in di.qset.keys() {
            let dist_u = *di.dist.get(k).unwrap();
            if dist_u < smallest {
                smallest = dist_u;
                found = *k;
            }
        }

        di.qset.remove(&found);
        if found.0 == di.width - 1 && found.1 == di.height - 1 {
            break;
        }

        for d in DIRS {
            let v = (found.0 + d.0, found.1 + d.1);
            if di.qset.contains_key(&v) {
                let alt = smallest + di.qset.get(&v).unwrap();
                if alt < *di.dist.get(&v).unwrap() {
                    *di.dist.get_mut(&v).unwrap() = alt;
                    *di.prev.get_mut(&v).unwrap() = Some(found);
                }
            }
        }
    }

    *di.dist.get(&(di.width - 1, di.height - 1)).unwrap()
}

fn build_part_1(lines: &[String]) -> Dijkstra {
    let width = lines[0].len() as i32;
    let height = lines.len() as i32;

    let mut dist: HashMap<(i32, i32), usize> = HashMap::new();
    let mut prev: HashMap<(i32, i32), Option<(i32, i32)>> = HashMap::new();
    let mut qset: HashMap<(i32, i32), usize> = HashMap::new();

    for (y, l) in lines.iter().enumerate() {
        for (x, c) in l.chars().enumerate() {
            dist.insert((x as i32, y as i32), std::usize::MAX);
            prev.insert((x as i32, y as i32), None);
            qset.insert((x as i32, y as i32), c as usize - '0' as usize);
        }
    }

    *dist.get_mut(&(0, 0)).unwrap() = 0;

    Dijkstra {
        width,
        height,
        dist,
        prev,
        qset,
    }
}

fn build_part_2(lines: &[String]) -> AStar {
    let width = lines[0].len() as i32;
    let height = lines.len() as i32;

    let mut g_score: HashMap<(i32, i32), usize> = HashMap::new();
    let mut f_score: HashMap<(i32, i32), usize> = HashMap::new();
    let mut cost: HashMap<(i32, i32), usize> = HashMap::new();

    let mut open_set: BinaryHeap<(usize, (i32, i32))> = BinaryHeap::new();
    open_set.push((0, (0, 0)));

    for (y, l) in lines.iter().enumerate() {
        for (x, c) in l.chars().enumerate() {
            for xmul in 0..5 {
                for ymul in 0..5 {
                    let xpos = xmul * width + x as i32;
                    let ypos = ymul * height + y as i32;
                    g_score.insert((xpos, ypos), std::usize::MAX);
                    f_score.insert((xpos, ypos), std::usize::MAX);

                    let mut val = c as usize - '0' as usize + xmul as usize + ymul as usize;
                    if val > 9 {
                        val -= 9
                    }
                    cost.insert((xpos, ypos), val);
                }
            }
        }
    }

    *g_score.get_mut(&(0, 0)).unwrap() = 0;
    *f_score.get_mut(&(0, 0)).unwrap() = (width + height) as usize;
    let came_from: HashMap<(i32, i32), (i32, i32)> = HashMap::new();

    AStar {
        width: width * 5,
        height: height * 5,
        open_set,
        cost,
        came_from,
        g_score,
        f_score,
    }
}
