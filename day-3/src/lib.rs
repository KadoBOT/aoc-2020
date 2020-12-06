use std::fs;
use std::io::{self, BufRead};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(278, pt1());
    }

    #[test]
    fn it_works_pt2() {
        assert_eq!(9709761600, pt2());
    }
}

fn read_file() -> io::Lines<io::BufReader<fs::File>> {
    let file = fs::File::open("./input.txt").unwrap();
    io::BufReader::new(file).lines()
}

fn count_trees(x: usize, y: usize) -> usize {
    let lines = read_file();
    let mut col = 0;
    let mut trees = 0;

    for line in lines.skip(y).step_by(y) {
        let line: String = line.unwrap();
        col += x;
        if line.chars().cycle().nth(col).unwrap() == '#' {
            trees += 1;
        }
    }

    trees
}

pub fn pt1() -> usize {
    count_trees(3, 1)
}

pub fn pt2() -> usize {
    count_trees(1, 1)
        * count_trees(3, 1)
        * count_trees(5, 1)
        * count_trees(7, 1)
        * count_trees(1, 2)
}
