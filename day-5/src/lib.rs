use std::fs;
use std::io::{self, BufRead};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works_example() {
        assert_eq!(357, search("FBFBBFFRLR".to_string()));
    }

    #[test]
    fn it_works_pt1() {
        assert_eq!(848, pt1());
    }

    #[test]
    fn it_works_pt2() {
        assert_eq!(682, pt2());
    }
}

fn read_file() -> io::Lines<io::BufReader<fs::File>> {
    let file = fs::File::open("./input.txt").unwrap();
    io::BufReader::new(file).lines()
}

pub fn search(line: String) -> i32 {
    let (mut start, mut end) = (0, 127);
    let (mut col_s, mut col_e) = (0, 7);
    for ch in line.chars() {
        match ch {
            'F' => end -= (end - start) / 2 + 1,
            'B' => start += (end - start) / 2 + 1,
            'L' => col_e -= (col_e - col_s) / 2 + 1,
            'R' => col_s += (col_e - col_s) / 2 + 1,
            _ => (),
        }
    }
    start * 8 + col_s
}

pub fn pt1() -> i32 {
    let lines = read_file();
    let mut highest_id = 0;
    for line in lines {
        let line: String = line.unwrap();
        let res = search(line);
        if res > highest_id {
            highest_id = res;
        }
    }

    highest_id
}

pub fn pt2() -> i32 {
    let lines = read_file();
    let mut highest_id = 0;
    let mut lowest_id = i32::MAX;
    let mut sum_ids = 0;

    for line in lines {
        let line: String = line.unwrap();
        let seat = search(line);
        if seat > highest_id {
            highest_id = seat;
        }
        if seat < lowest_id {
            lowest_id = seat;
        }
        sum_ids += seat;
    }

    let mut result = 0;
    for id in lowest_id..=highest_id {
        result += id;
    }

    result - sum_ids
}
