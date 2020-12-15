use regex::*;
use std::collections::BTreeMap;
use std::fs;
use std::io::{self, BufRead};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = pt1();
        println!("{}", result);
        assert_eq!(13105044880745, result);
    }

    #[test]
    fn it_works_pt2() {
        assert_eq!(3505392154485, pt2());
    }
}

fn read_file() -> io::Lines<io::BufReader<fs::File>> {
    let file = fs::File::open("./input.txt").unwrap();
    io::BufReader::new(file).lines()
}

fn write(
    mem: &mut BTreeMap<u64, u64>,
    addr: u64,
    val: u64,
    mut mask: impl Iterator<Item = (usize, char)> + Clone,
) {
    match mask.next() {
        None => *mem.entry(addr).or_default() = val,
        Some((bit, bit_mask)) => match bit_mask {
            '0' => write(mem, addr, val, mask),
            '1' => write(mem, addr | (1 << bit), val, mask),
            'X' => {
                write(mem, addr & !(1 << bit), val, mask.clone());
                write(mem, addr | (1 << bit), val, mask);
            }
            _ => unreachable!(),
        },
    }
}

pub fn pt2() -> u64 {
    let lines = read_file().filter_map(Result::ok);
    let re = Regex::new(r"mem\[(\d+)] = (\d+)").unwrap();
    let mut memory = BTreeMap::new();
    let mut mask = "0".repeat(36).to_string();

    for line in lines {
        if line.starts_with("mask") {
            mask = line.split_at(7).1.to_string();
        } else {
            let caps = re.captures(&line).unwrap();
            let addr: u64 = caps[1].parse().unwrap();
            let val: u64 = caps[2].parse().unwrap();

            write(
                &mut memory,
                addr,
                val,
                mask.chars().enumerate().map(|(bit, mask)| (35 - bit, mask)),
            )
        }
    }

    memory.values().sum()
}

pub fn pt1() -> u64 {
    let lines = read_file().filter_map(Result::ok);
    let (mut and, mut or): (u64, u64) = (0xFFFFFFFFF, 0);
    let re = regex::Regex::new(r"mem\[(\d+)] = (\d+)").unwrap();
    let mut memory = BTreeMap::new();

    for line in lines {
        if line.starts_with("mask") {
            or = 0;
            let mut not_and = 0;
            line.chars().skip(7).enumerate().for_each(|(i, c)| match c {
                'X' => {}
                '0' => not_and |= 1 << (35 - i),
                '1' => or |= 1 << (35 - i),
                _ => unreachable!(),
            });
            and = !not_and;
        } else {
            let caps = re.captures(&line).unwrap();
            let addr: u64 = caps[1].parse().unwrap();
            let val: u64 = caps[2].parse().unwrap();

            *memory.entry(addr).or_default() = (val & and) | or;
        }
    }

    memory.values().sum()
}
