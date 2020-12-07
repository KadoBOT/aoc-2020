use std::collections::{HashMap, HashSet};
use std::fs;
use std::io::{self, BufRead};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works_pt1() {
        assert_eq!(233, pt1());
    }

    #[test]
    fn it_works_pt2() {
        assert_eq!(421550, pt2("shiny gold".to_string()));
    }
}

fn read_file() -> io::Lines<io::BufReader<fs::File>> {
    let file = fs::File::open("./input.txt").unwrap();
    io::BufReader::new(file).lines()
}

fn read_list(cb: &mut dyn FnMut(Vec<&str>)) {
    let lines = read_file();
    for line in lines {
        let l = line.unwrap();
        cb(l.split(" bags contain ").collect::<Vec<_>>());
    }
}

fn count(val: String) -> HashSet<String> {
    let mut bags = HashSet::new();
    let mut parse = |list: Vec<&str>| {
        if list[1].contains(&val) {
            bags.insert(list[0].trim().to_string());
        }
    };
    read_list(&mut parse);
    bags
}

pub fn pt1() -> usize {
    let mut map = HashMap::new();
    map.insert("shiny gold".to_string(), count("shiny gold".to_string()));

    let mut res = HashSet::new();
    let mut stack = vec!["shiny gold".to_string()];

    while !stack.is_empty() {
        let b = stack.pop().unwrap();
        if let Some(bags) = map.get_mut(&b) {
            for b in bags.clone() {
                if !res.contains(&b) {
                    map.insert(b.to_string(), count(b.to_string()));
                    res.insert(b.to_string());
                    stack.push(b);
                }
            }
        }
    }

    res.len()
}

fn parse_bags(bags: String) -> HashMap<String, usize> {
    let bags = bags
        .replace(".", "")
        .replace(" bags", "")
        .replace(" bag", "");
    let bags = bags.split(", ").collect::<Vec<_>>();
    bags.iter()
        .filter_map(|n| {
            if *n == "no other" {
                return None;
            }

            let cur = n.splitn(2, " ").collect::<Vec<_>>();
            return Some((cur[1].to_string(), cur[0].parse::<usize>().unwrap()));
        })
        .collect::<HashMap<_, _>>()
}

pub fn pt2(name: String) -> usize {
    let mut stack = HashMap::new();
    let mut parse = |list: Vec<&str>| {
        if list[0] == name.as_str() {
            stack = parse_bags(list[1].to_string());
        }
    };
    read_list(&mut parse);

    stack
        .iter()
        .fold(0, |acc, (k, v)| acc + v + (v * pt2(k.to_owned())))
}
