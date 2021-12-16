use crate::utils::read_file;
use std::slice::*;

pub fn day16() {
    let lines = read_file("inputs/input-day16.txt");
    let decode = decode_hex(&lines[0]);
    println!("Day 16 - Part 1: {}", sum_versions(&decode));
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

fn sum_versions(bits: &[u8]) -> u64 {
    let mut iter = bits.iter();
    get_version_sum(&mut iter, true)
}

fn get_version_sum(iter: &mut Iter<'_, u8>, first: bool) -> u64 {
    let mut version = 0;

    while let Some(v) = iter.next() {
        let rest = iter.bits_to_num(2);
        version += (v << 2) as u64 | rest;
        let msg_type = iter.bits_to_num(3);
        if msg_type == 4 {
            while iter.bits_to_num(5) > 15 {
                // consume bits
            }
        } else {
            let len_type = iter.bits_to_num(1);
            if len_type == 0 {
                let subpacket_len = iter.bits_to_num(15);
                let subpackets: Vec<u8> = iter.take(subpacket_len as usize).copied().collect();
                let mut newiter = subpackets.iter();
                version += get_version_sum(&mut newiter, false);
            } else {
                let subpacket_count = iter.bits_to_num(11);
                for _ in 0..subpacket_count {
                    version += get_version_sum(iter, false);
                }
            }
        }

        if first {
            break;
        }
    }

    version
}

#[cfg(test)]
mod tests {
    use super::*;

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
        assert_eq!(sum_versions(&bits), 6);
    }

    #[test]
    fn double_packet() {
        let bits = decode_hex("38006F45291200");
        assert_eq!(sum_versions(&bits), 1 + 6 + 2);
    }

    #[test]
    fn three_sub_packets() {
        let bits = decode_hex("EE00D40C823060");
        assert_eq!(sum_versions(&bits), 7 + 2 + 4 + 1);
    }

    #[test]
    fn double_recusion() {
        let bits = decode_hex("8A004A801A8002F478");
        assert_eq!(sum_versions(&bits), 16);
    }

    #[test]
    fn two_plus_two() {
        let bits = decode_hex("620080001611562C8802118E34");
        assert_eq!(sum_versions(&bits), 12);
    }

    #[test]
    fn two_plus_two_different_length() {
        let bits = decode_hex("C0015000016115A2E0802F182340");
        assert_eq!(sum_versions(&bits), 23);
    }

    #[test]
    fn triple_nested_five_values() {
        let bits = decode_hex("A0016C880162017C3686B18A3D4780");
        assert_eq!(sum_versions(&bits), 31);
    }

    #[test]
    fn bits_check() {
        let v: Vec<u8> = vec![1, 1];
        let mut i = v.iter();
        assert_eq!(i.bits_to_num(3), 3);
        //assert_eq!(i.bits_to_num(3), 3);
    }
}
