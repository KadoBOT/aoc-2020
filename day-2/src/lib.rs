use std::fs;
use std::io::{self, BufRead};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(493, pt_1());
    }

    #[test]
    fn it_works_pt2() {
        assert_eq!(593, pt_2());
    }
}

fn read_file() -> io::Lines<io::BufReader<fs::File>> {
    let file = fs::File::open("./input.txt").unwrap();
    io::BufReader::new(file).lines()
}

fn parse_line(line: Result<String, io::Error>) -> ((i32, i32), char, String) {
    let line = line.unwrap();
    let r = line.split(" ").collect::<Vec<&str>>();
    let qty = r[0].split("-").collect::<Vec<_>>();
    let letter = r[1].chars().nth(0).unwrap();
    let pass = r[2];
    let (min, max) = (
        qty[0].parse::<i32>().unwrap(),
        qty[1].parse::<i32>().unwrap(),
    );
    ((min, max), letter, pass.to_string())
}

pub fn pt_1() -> i32 {
    let lines = read_file();
    let mut counter = 0;

    for line in lines {
        let mut letter_counter = 0;
        let ((min, max), letter, pass) = parse_line(line);
        for ch in pass.chars() {
            if letter_counter > max {
                break;
            }
            if ch == letter {
                letter_counter += 1;
            }
        }

        if letter_counter >= min && max >= letter_counter {
            counter += 1;
        }
    }
    counter
}

pub fn pt_2() -> i32 {
    let lines = read_file();
    let mut counter = 0;

    for line in lines {
        let mut letter_counter = 0;
        let ((idx1, idx2), letter, pass) = parse_line(line);
        if pass.chars().nth(idx1 as usize - 1).unwrap() == letter {
            letter_counter += 1;
        }
        if pass.chars().nth(idx2 as usize - 1).unwrap() == letter {
            letter_counter += 1;
        }
        if letter_counter == 1 {
            counter += 1;
        }
    }
    counter
}
