use std::collections::{HashMap, HashSet};
use std::fs;
use std::io::{self, BufRead};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works_pt1() {
        assert_eq!(6735, pt1());
    }

    #[test]
    fn it_works_pt2() {
        assert_eq!(3221, pt2());
    }
}

fn read_file() -> io::Lines<io::BufReader<fs::File>> {
    let file = fs::File::open("./input.txt").unwrap();
    io::BufReader::new(file).lines()
}

pub fn pt1() -> usize {
    let lines = read_file();
    let mut answers = HashSet::new();
    let mut count = 0;

    for line in lines {
        let line: String = line.unwrap();
        if line.is_empty() {
            count += answers.len();
            answers = HashSet::new();
            continue;
        }

        for ch in line.chars() {
            answers.insert(ch);
        }
    }

    count + answers.len()
}

pub fn pt2() -> usize {
    let lines = read_file();
    let mut count = 0;
    let mut group_size = 0;
    let mut temp = HashMap::new();
    let mut reset = |c: &mut usize, t: &mut HashMap<char, usize>| {
        let a = t.iter().filter(|(_, &val)| val == *c).collect::<Vec<_>>();
        count += a.len();
        *t = HashMap::new();
        *c = 0;
    };

    for line in lines {
        let line: String = line.unwrap();
        if line.is_empty() {
            reset(&mut group_size, &mut temp);
            continue;
        }

        group_size += 1;
        for ch in line.chars() {
            let entry = temp.entry(ch).or_insert(0);
            *entry += 1;
        }
    }

    reset(&mut group_size, &mut temp);
    count
}
