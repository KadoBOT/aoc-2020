use std::collections::HashSet;
use std::fs;
use std::io::{self, BufRead};

#[cfg(test)]
mod tests {
    use crate::pt1;
    use crate::pt2;

    #[test]
    fn it_works_pt1() {
        assert_eq!(1684, pt1());
    }

    #[test]
    fn it_works_pt2() {
        assert_eq!(Some(2188), pt2());
    }
}

fn read_file() -> io::Lines<io::BufReader<fs::File>> {
    let file = fs::File::open("./input.txt").unwrap();
    io::BufReader::new(file).lines()
}

fn get_ops() -> Vec<Vec<String>> {
    let lines = read_file();
    lines
        .map(|line| {
            let line = line.unwrap();
            line.split_ascii_whitespace()
                .map(|line| line.to_string())
                .collect::<Vec<String>>()
        })
        .collect::<Vec<Vec<_>>>()
}

fn update(line: &Vec<String>, acc: &mut i32, index: &mut i32) {
    let num = line[1][0..].parse::<i32>().unwrap();
    match line[0].as_str() {
        "acc" => {
            *acc = *acc + num;
            *index = *index + 1;
        }
        "jmp" => *index = *index + num,
        "nop" => *index = *index + 1,
        _ => (),
    }
}

pub fn pt1() -> i32 {
    let mut visited = HashSet::new();
    let ops = get_ops();
    let mut index: i32 = 0;
    let mut acc = 0;

    while !visited.contains(&index) {
        visited.insert(index);
        update(ops.get(index as usize).unwrap(), &mut acc, &mut index)
    }

    acc
}

pub fn pt2() -> Option<i32> {
    let ops = get_ops();

    for i in 0..ops.len() {
        let mut visited = HashSet::new();
        let mut ops_clone = ops.clone();

        match ops_clone.get(i).unwrap()[0].as_str() {
            "nop" => ops_clone[i][0] = "jmp".to_string(),
            "jmp" => ops_clone[i][0] = "nop".to_string(),
            _ => continue,
        }

        let mut index: i32 = 0;
        let mut acc = 0;

        while index < ops_clone.len() as i32 && !visited.contains(&index) {
            visited.insert(index);
            update(ops_clone.get(index as usize).unwrap(), &mut acc, &mut index)
        }

        if index as usize == ops_clone.len() {
            return Some(acc);
        }
    }
    None
}
