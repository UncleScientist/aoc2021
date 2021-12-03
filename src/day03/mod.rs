use crate::utils::read_file;

pub fn day03() {
    let lines = read_file("inputs/input-day03.txt");

    println!("Day 03 - Part 1: {}", part1(&lines));
    println!("Day 03 - Part 2: {}", part2(&lines));
}

fn part1(lines: &[String]) -> usize {
    let bitwidth = lines[0].len();
    let max = lines.len() / 2;

    let count = bit_freq(lines, bitwidth);

    let mut gamma = 0;
    for value in &count {
        gamma <<= 1;
        gamma |= (*value > max) as usize;
    }
    let epsilon = ((1 << bitwidth) - 1) ^ gamma;
    epsilon * gamma
}

fn part2(input_lines: &[String]) -> usize {
    let bitwidth = input_lines[0].len();

    let mut lines = input_lines.to_owned();

    loop {
        for i in 0..bitwidth {
            let max = lines.len();
            let count = bit_freq(&lines, bitwidth);
            let mut new_lines: Vec<String> = Vec::new();
            if count[i] >= max - count[i] {
                for l in lines {
                    if l.chars().nth(i).unwrap() == '1' {
                        new_lines.push(l.clone().to_string());
                    }
                }
            } else {
                for l in lines {
                    if l.chars().nth(i).unwrap() == '0' {
                        new_lines.push(l.clone().to_string());
                    }
                }
            }
            lines = new_lines;
            if lines.len() < 2 {
                break;
            }
        }
        if lines.len() < 2 {
            break;
        }
    }

    let oxygen = usize::from_str_radix(&lines[0], 2).unwrap();

    lines = input_lines.to_owned();

    loop {
        for i in 0..bitwidth {
            let max = lines.len();
            let count = bit_freq(&lines, bitwidth);
            let mut new_lines: Vec<String> = Vec::new();
            if count[i] < max - count[i] {
                for l in lines {
                    if l.chars().nth(i).unwrap() == '1' {
                        new_lines.push(l.clone().to_string());
                    }
                }
            } else {
                for l in lines {
                    if l.chars().nth(i).unwrap() == '0' {
                        new_lines.push(l.clone().to_string());
                    }
                }
            }
            lines = new_lines;
            if lines.len() < 2 {
                break;
            }
        }
        if lines.len() < 2 {
            break;
        }
    }

    let co2 = usize::from_str_radix(&lines[0], 2).unwrap();

    oxygen * co2
}

fn bit_freq(lines: &[String], bitwidth: usize) -> Vec<usize> {
    let mut count = vec![0; bitwidth];
    for line in lines.iter().map(|s| s.chars()) {
        for c in line.enumerate().filter(|(_, val)| *val == '1') {
            count[c.0] += 1;
        }
    }
    count
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

        assert_eq!(part1(&lines), 198);
        assert_eq!(part2(&lines), 230);
    }
}
