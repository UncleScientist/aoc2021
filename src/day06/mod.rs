pub fn day06() {
    let lanternfish = "1,1,1,2,1,1,2,1,1,1,5,1,1,1,1,1,1,1,1,1,1,2,1,1,1,1,1,4,1,1,1,1,3,1,1,3,1,1,1,4,1,5,1,3,1,1,1,1,1,5,1,1,1,1,1,5,5,2,5,1,1,2,1,1,1,1,3,4,1,1,1,1,1,1,1,1,1,1,1,1,2,1,1,1,1,5,4,1,1,1,1,1,5,1,2,4,1,1,1,1,1,3,3,2,1,1,4,1,1,5,5,1,1,1,1,1,2,5,1,4,1,1,1,1,1,1,2,1,1,5,2,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,4,3,1,1,3,1,3,1,4,1,5,4,1,1,2,1,1,5,1,1,1,1,1,5,1,1,1,1,1,1,1,1,1,4,1,1,4,1,1,1,1,1,1,1,5,4,1,2,1,1,1,1,1,1,1,1,1,1,1,3,1,1,1,1,1,1,1,1,1,1,4,1,1,1,2,1,4,1,1,1,1,1,1,1,1,1,4,2,1,2,1,1,4,1,1,1,1,1,1,3,1,1,1,1,1,1,1,1,3,2,1,4,1,5,1,1,1,4,5,1,1,1,1,1,1,5,1,1,5,1,2,1,1,2,4,1,1,2,1,5,5,3".to_string();

    let mut school = parse(lanternfish);

    println!("Day 06 - Part 1: {}", cycle(&mut school, 80));
    println!("Day 06 - Part 2: {}", cycle(&mut school, 256 - 80));
}

fn parse(line: String) -> Vec<u64> {
    let v: Vec<usize> = line.split(',').map(|x| x.parse().unwrap()).collect();

    let mut result = vec![0; 9];
    for number in v {
        result[number] += 1;
    }

    result
}

fn cycle(fish: &mut [u64], cycles: usize) -> u64 {
    for _ in 0..cycles {
        let new_fish = fish[0];
        fish[7] += new_fish;
        for i in 1..9 {
            fish[i - 1] = fish[i];
        }
        fish[8] = new_fish;
    }

    fish.iter().sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parsing() {
        assert_eq!(
            parse("3,4,3,1,2".to_string()),
            vec![0, 1, 1, 2, 1, 0, 0, 0, 0]
        );
    }

    #[test]
    fn test_18_days() {
        let mut x = parse("3,4,3,1,2".to_string());
        assert_eq!(cycle(&mut x, 18), 26);
    }

    #[test]
    fn test_80_days() {
        let mut x = parse("3,4,3,1,2".to_string());
        assert_eq!(cycle(&mut x, 80), 5934);
    }

    #[test]
    fn test_256_days() {
        let mut x = parse("3,4,3,1,2".to_string());
        assert_eq!(cycle(&mut x, 256), 26984457539);
    }
}
