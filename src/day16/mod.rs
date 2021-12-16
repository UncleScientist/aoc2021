use crate::utils::read_file;
use std::slice::*;

pub fn day16() {
    let lines = read_file("inputs/input-day16.txt");
    let decode = decode_hex(&lines[0]);
    println!("Day 16 - Part 1: {:?}", calculate(&decode));
}

trait BitReader {
    fn bits_to_num(&mut self, count: u64) -> u64;
}

impl BitReader for Iter<'_, u8> {
    fn bits_to_num(&mut self, count: u64) -> u64 {
        self.take(count as usize)
            .fold(0, |acc, bit| (acc << 1) | *bit as u64)
    }
}

fn decode_hex(hex: &str) -> Vec<u8> {
    let mut result = Vec::new();
    for c in hex.chars() {
        let val = if c >= 'A' {
            c as u8 - b'A' + 10
        } else {
            c as u8 - b'0'
        };
        for b in [8, 4, 2, 1] {
            result.push(((val & b) != 0) as u8);
        }
    }

    result
}

fn calculate(bits: &[u8]) -> (u64, Vec<u64>) {
    let mut iter = bits.iter();
    calculate_packets(&mut iter, true)
}

fn calculate_packets(iter: &mut Iter<'_, u8>, first: bool) -> (u64, Vec<u64>) {
    let mut version = 0;
    let mut value = Vec::new();

    while let Some(v) = iter.next() {
        let rest = iter.bits_to_num(2);
        version += (v << 2) as u64 | rest;
        let msg_type = iter.bits_to_num(3);
        println!("{} msg type == {}", first, msg_type);
        if msg_type == 4 {
            let mut digit = iter.bits_to_num(5);
            let mut result = 0;
            while digit > 15 {
                result = result << 4 | (digit & 0xf);
                digit = iter.bits_to_num(5);
            }
            result = result << 4 | (digit & 0xf);
            println!(" > literal {}", result);
            value.push(result);
        } else {
            let len_type = iter.bits_to_num(1);
            if len_type == 0 {
                let subpacket_len = iter.bits_to_num(15);
                println!("packet-by-len {}", subpacket_len);
                let subpackets: Vec<u8> = iter.take(subpacket_len as usize).copied().collect();
                let mut newiter = subpackets.iter();
                let (versum, valsum) = calculate_packets(&mut newiter, false);
                version += versum;
                value.extend(valsum);
            } else {
                let subpacket_count = iter.bits_to_num(11);
                println!("packet-by-count {}", subpacket_count);
                for _ in 0..subpacket_count {
                    let (versum, valsum) = calculate_packets(iter, true);
                    println!(" > packet values are: {:?}", valsum);
                    version += versum;
                    value.extend(valsum);
                    println!(" > current values: {:?}", value);
                }
            }
            match msg_type {
                0 => { 
                    let newval = value.iter().sum();
                    value.clear();
                    println!("sum is {}", newval);
                    value.push(newval);
                },
                1 => {
                    let newval = value.iter().product();
                    value.clear();
                    println!("product is {}", newval);
                    value.push(newval);
                },
                2 => {
                    let newval = *value.iter().min().unwrap();
                    value.clear();
                    value.push(newval);
                },
                3 => {
                    println!("values = {:?}", value);
                    let newval = *value.iter().max().unwrap();
                    println!("max = {}", newval);
                    value.clear();
                    value.push(newval);
                },

                5 => {
                    let left = value[0];
                    let right = value[1];
                    value.clear();
                    value.push((left > right) as u64);
                }

                6 => {
                    let left = value[0];
                    let right = value[1];
                    value.clear();
                    value.push((left < right) as u64);
                }

                7 => {
                    println!("eq - {:?}", value);
                    let left = value[0];
                    let right = value[1];
                    value.clear();
                    value.push((left == right) as u64);
                }

                _ => panic!()
            }
        }

        if first {
            break;
        }
    }

    println!("returning value: {:?}", value);
    (version, value)
}

#[cfg(test)]
mod tests {
    use super::*;
/*
    #[test]
    fn can_decode_hex() {
        assert_eq!(
            decode_hex("D2FE28"),
            vec![1, 1, 0, 1, 0, 0, 1, 0, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 1, 0, 1, 0, 0, 0]
        );
    }

    #[test]
    fn simple_version() {
        let bits = decode_hex("D2FE28");
        assert_eq!(calculate(&bits).0, 6);
    }

    #[test]
    fn double_packet() {
        let bits = decode_hex("38006F45291200");
        assert_eq!(calculate(&bits).0, 1 + 6 + 2);
    }

    #[test]
    fn three_sub_packets() {
        let bits = decode_hex("EE00D40C823060");
        assert_eq!(calculate(&bits).0, 7 + 2 + 4 + 1);
    }

    #[test]
    fn double_recusion() {
        let bits = decode_hex("8A004A801A8002F478");
        assert_eq!(calculate(&bits).0, 16);
    }

    #[test]
    fn two_plus_two() {
        let bits = decode_hex("620080001611562C8802118E34");
        assert_eq!(calculate(&bits).0, 12);
    }

    #[test]
    fn two_plus_two_different_length() {
        let bits = decode_hex("C0015000016115A2E0802F182340");
        assert_eq!(calculate(&bits).0, 23);
    }

    #[test]
    fn triple_nested_five_values() {
        let bits = decode_hex("A0016C880162017C3686B18A3D4780");
        assert_eq!(calculate(&bits).0, 31);
    }

    #[test]
    fn sum_of_1_and_2() {
        let bits = decode_hex("C200B40A82");
        assert_eq!(*calculate(&bits).1.iter().next().unwrap(), 3);
    }

    #[test]
    fn prod_of_6_and_9() {
        let bits = decode_hex("04005AC33890");
        assert_eq!(*calculate(&bits).1.iter().next().unwrap(), 54);
    }

    #[test]
    fn min_of_789() {
        let bits = decode_hex("880086C3E88112");
        assert_eq!(*calculate(&bits).1.iter().next().unwrap(), 7);
    }

    #[test]
    fn max_of_789() {
        let bits = decode_hex("CE00C43D881120");
        assert_eq!(*calculate(&bits).1.iter().next().unwrap(), 9);
    }

    #[test]
    fn less_than() {
        let bits = decode_hex("D8005AC2A8F0");
        assert_eq!(*calculate(&bits).1.iter().next().unwrap(), 1);
    }

    #[test]
    fn greater_than() {
        let bits = decode_hex("F600BC2D8F");
        assert_eq!(*calculate(&bits).1.iter().next().unwrap(), 0);
    }

    #[test]
    fn equality() {
        let bits = decode_hex("9C005AC2F8F0");
        assert_eq!(*calculate(&bits).1.iter().next().unwrap(), 0);
    }
*/
    #[test]
    fn math() {
        let bits = decode_hex("9C0141080250320F1802104A08");
        assert_eq!(*calculate(&bits).1.iter().next().unwrap(), 1);
    }
}
