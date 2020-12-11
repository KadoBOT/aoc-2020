use std::collections::HashMap;
use std::collections::HashSet;
use std::fs;
use std::io::{self, BufRead};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(2210, pt1());
    }

    #[test]
    fn it_works_pt2() {
        assert_eq!(7086739046912, pt2());
    }
}

fn read_file() -> io::Lines<io::BufReader<fs::File>> {
    let file = fs::File::open("./input.txt").unwrap();
    io::BufReader::new(file).lines()
}

pub fn pt1() -> i32 {
    let lines = read_file()
        .map(|n| n.unwrap().parse::<i32>().unwrap())
        .collect::<HashSet<_>>();

    let mut one_count = 0;
    let mut three_count = 1;
    let mut count = 0;

    loop {
        if lines.contains(&(count + 1)) {
            one_count += 1;
            count += 1;
        } else if lines.contains(&(count + 3)) {
            three_count += 1;
            count += 3;
        } else {
            break;
        }
    }

    one_count * three_count
}

fn count(i: usize, lines: &Vec<i64>, items: &mut HashMap<usize, i64>) -> i64 {
    if i == lines.len() - 1 {
        return 1;
    }
    if items.contains_key(&i) {
        return *items.get(&i).unwrap();
    }
    let mut result = 0;
    for j in i + 1..lines.len() {
        if lines[j] - lines[i] <= 3 {
            result += count(j, lines, items)
        }
    }
    items.insert(i, result);
    result
}

pub fn pt2() -> i64 {
    let mut lines = read_file()
        .map(|n| n.unwrap().parse::<i64>().unwrap())
        .collect::<Vec<_>>();
    lines.sort();
    lines.insert(0, 0);
    lines.push(lines.last().unwrap() + 3);

    let mut items = HashMap::new();

    count(0, &lines, &mut items)
}
