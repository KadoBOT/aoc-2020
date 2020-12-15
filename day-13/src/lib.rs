use std::fs;
use std::io::{self, BufRead};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(3215, pt1());
    }

    #[test]
    fn it_works_pt2() {
        assert_eq!(1001569619313439, pt2());
    }
}

fn read_file() -> io::Lines<io::BufReader<fs::File>> {
    let file = fs::File::open("./input.txt").unwrap();
    io::BufReader::new(file).lines()
}

fn parse_input() -> (i64, Vec<(i64, i64)>) {
    let file = read_file()
        .map(|n| n.unwrap().to_string())
        .collect::<Vec<_>>();
    let times = file[1]
        .split(",")
        .enumerate()
        .filter_map(|(i, n)| n.parse().ok().map(|b| (i as i64, b)))
        .collect::<Vec<_>>();
    (file[0].parse().unwrap(), times)
}

pub fn pt1() -> i64 {
    let (timestamp, ids) = parse_input();
    let (mut earliest_id, mut earliest_diff) = (0, i64::MAX);
    for (_, id) in ids {
        let diff = ((timestamp / id + 1) % timestamp) * id % timestamp;
        if diff < earliest_diff {
            earliest_id = id;
            earliest_diff = diff;
        }
    }
    earliest_id * earliest_diff
}

pub fn pt2() -> i64 {
    let (_, lines) = parse_input();
    let prod: i64 = lines.iter().map(|(_, b)| b).product();
    lines
        .iter()
        .map(|&(a, b)| -a * (prod / b) * inv_mod(prod / b, b))
        .sum::<i64>()
        .rem_euclid(prod)
}

fn inv_mod(x: i64, p: i64) -> i64 {
    (0..p - 2).fold(1, |o, _| (o * x) % p)
}
