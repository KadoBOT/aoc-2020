use std::collections::HashMap;
use std::fs;
use std::io::{self, BufRead};

#[cfg(test)]
mod tests {
    use crate::pt1;
    use crate::pt2;

    #[test]
    fn it_works_pt1() {
        assert_eq!(88311122, pt1());
    }

    #[test]
    fn it_works_pt2() {
        assert_eq!(13549369, pt2());
    }
}

fn read_file() -> io::Lines<io::BufReader<fs::File>> {
    let file = fs::File::open("./input.txt").unwrap();
    io::BufReader::new(file).lines()
}

pub fn pt1() -> i32 {
    for (i, line) in read_file().enumerate().skip(25) {
        let mut targets = HashMap::new();
        let line = line.unwrap().parse::<i32>().unwrap();
        let prev_lines = read_file()
            .enumerate()
            .skip(i - 25)
            .take_while(|(idx, _)| *idx < i);
        let mut is_valid = false;
        for (_, previous_line) in prev_lines {
            let prev = previous_line.unwrap().parse::<i32>().unwrap();
            if targets.contains_key(&prev) {
                is_valid = true;
                break;
            }
            let target = line - prev;
            targets.insert(target, false);
        }

        if !is_valid {
            return line;
        }
    }

    0
}

pub fn pt2() -> i64 {
    for (i, line) in read_file().enumerate() {
        let line = line.unwrap().parse::<i64>().unwrap();
        let next_lines = read_file().skip(i + 1);
        let mut sum = line;
        let mut smallest = line;
        let mut largest = line;

        for next_line in next_lines {
            let next = next_line.unwrap().parse::<i64>().unwrap();
            if next < smallest {
                smallest = next;
            }
            if next > largest {
                largest = next;
            }

            if sum + next == 88311122 {
                return smallest + largest;
            }

            if sum + next >= 88311122 {
                break;
            }

            sum += next;
        }
    }

    0
}
