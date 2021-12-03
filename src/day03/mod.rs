use crate::utils::read_file;

pub fn day03() {
    let lines = read_file("inputs/input-day03.txt");
    let nums: Vec<usize> = lines
        .iter()
        .map(|n| usize::from_str_radix(n, 2).unwrap())
        .collect();
    let bitwidth = lines[0].len();

    println!("Day 03 - Part 1: {}", part1(&nums, bitwidth));
    println!("Day 03 - Part 2: {}", part2(&nums, bitwidth));
}

fn part1(nums: &[usize], bitwidth: usize) -> usize {
    let max = nums.len() / 2;

    let count = bit_freq(nums, bitwidth);

    let mut gamma = 0;
    for value in &count {
        gamma <<= 1;
        gamma |= (*value > max) as usize;
    }
    let epsilon = ((1 << bitwidth) - 1) ^ gamma;
    epsilon * gamma
}

fn part2(nums: &[usize], bitwidth: usize) -> usize {
    let oxygen = filter(nums, bitwidth, '1');
    let co2 = filter(nums, bitwidth, '0');

    oxygen * co2
}

fn bit_freq(nums: &[usize], bitwidth: usize) -> Vec<usize> {
    let mut count = vec![0; bitwidth];
    for (i, entry) in count.iter_mut().enumerate().take(bitwidth) {
        for n in nums {
            if (n & (1 << (bitwidth - i - 1))) != 0 {
                *entry += 1;
            }
        }
    }
    count
}

fn filter(nums: &[usize], bitwidth: usize, key: char) -> usize {
    let mut nums = nums.to_owned();

    for i in 0..bitwidth {
        let check = 1 << (bitwidth - i - 1);
        let max = nums.len();
        let count = bit_freq(&nums, bitwidth);
        nums = nums
            .iter()
            .map(|n| match (2 * count[i] >= max, (n & check) != 0) {
                (true, true) if key == '1' => Some(*n),
                (false, false) if key == '1' => Some(*n),
                (true, false) if key == '0' => Some(*n),
                (false, true) if key == '0' => Some(*n),
                (_, _) => None,
            })
            .filter(|x| x.is_some())
            .flatten()
            .collect();
        if nums.len() < 2 {
            break;
        }
    }

    nums[0]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let lines = vec![
            "00100".to_string(),
            "11110".to_string(),
            "10110".to_string(),
            "10111".to_string(),
            "10101".to_string(),
            "01111".to_string(),
            "00111".to_string(),
            "11100".to_string(),
            "10000".to_string(),
            "11001".to_string(),
            "00010".to_string(),
            "01010".to_string(),
        ];

        let nums: Vec<usize> = lines
            .iter()
            .map(|n| usize::from_str_radix(n, 2).unwrap())
            .collect();
        assert_eq!(part1(&nums, 5), 198);
        assert_eq!(part2(&nums, 5), 230);
    }
}
