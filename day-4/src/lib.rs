use std::collections::HashMap;
use std::fs;
use std::io::{self, BufRead};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(190, pt1());
    }
    #[test]
    fn it_works_pt2() {
        assert_eq!(121, pt2());
    }
}

struct Passport {
    fields: HashMap<String, String>,
}

const REQUIRED_FIELDS: &[&str] = &["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];

impl Passport {
    pub fn new() -> Self {
        Passport {
            fields: HashMap::new(),
        }
    }

    pub fn weak_validation(&self) -> bool {
        for field in REQUIRED_FIELDS {
            if self.fields.contains_key(&field.to_string()) {
                continue;
            }
            return false;
        }
        true
    }

    pub fn strong_validation(&self) -> bool {
        for field in REQUIRED_FIELDS {
            let f = self.fields.get(&field.to_string());
            match f {
                Some(x) if ["byr", "iyr", "eyr"].contains(field) => {
                    let num = x.parse::<i32>().unwrap();
                    let valid_byr = *field == "byr" && num >= 1920 && num <= 2002;
                    let valid_iyr = *field == "iyr" && num >= 2010 && num <= 2020;
                    let valid_eyr = *field == "eyr" && num >= 2020 && num <= 2030;
                    if !valid_byr && !valid_eyr && !valid_iyr {
                        return false;
                    }
                }
                Some(x) if field == &"hgt" => {
                    let mut val = String::new();
                    let mut unit = String::new();
                    for ch in x.chars() {
                        if ch.is_numeric() {
                            val.push(ch);
                        } else {
                            unit.push(ch);
                        }
                    }
                    let val = val.parse::<i32>().unwrap();
                    let is_valid_cm = unit == "cm" && (val >= 150 && val <= 193);
                    let is_valid_in = unit == "in" && (val >= 59 && val <= 76);
                    if !is_valid_cm && !is_valid_in {
                        return false;
                    }
                }
                Some(x) if field == &"hcl" => {
                    if x.len() != 7 {
                        return false;
                    }

                    for (i, ch) in x.chars().enumerate() {
                        match i {
                            0 if ch == '#' => continue,
                            1..=6 if ch.is_numeric() || ('a'..='f').contains(&ch) => continue,
                            _ => return false,
                        }
                    }
                }
                Some(x) if field == &"ecl" => {
                    if !["amb", "blu", "brn", "gry", "grn", "hzl", "oth"].contains(&x.as_str()) {
                        return false;
                    }
                }
                Some(x) if field == &"pid" => {
                    if x.len() != 9 {
                        return false;
                    }
                    for ch in x.chars() {
                        if !ch.is_numeric() {
                            return false;
                        }
                    }
                }
                Some(_) if field == &"cid" => continue,
                _ => return false,
            }
        }
        true
    }

    pub fn insert(&mut self, key: &str, value: &str) {
        self.fields.insert(key.to_string(), value.to_string());
    }
}

fn read_file() -> io::Lines<io::BufReader<fs::File>> {
    let file = fs::File::open("./input.txt").unwrap();
    io::BufReader::new(file).lines()
}

pub fn pt1() -> i32 {
    let lines = read_file();
    let mut valid_passports_count = 0;
    let mut pass = Passport::new();
    for line in lines {
        let line: String = line.unwrap();
        if line.is_empty() {
            if pass.weak_validation() {
                valid_passports_count += 1;
            }
            pass = Passport::new();
        }
        let fields = line.split_whitespace().collect::<Vec<_>>();
        for field in fields {
            let f = field.split(":").collect::<Vec<_>>();
            let (key, value) = (f[0], f[1]);
            pass.insert(key, value);
        }
    }
    valid_passports_count
}

pub fn pt2() -> i32 {
    let lines = read_file();
    let mut valid_passports_count = 0;
    let mut pass = Passport::new();
    for line in lines {
        let line: String = line.unwrap();
        if line.is_empty() {
            if pass.strong_validation() {
                valid_passports_count += 1;
            }
            pass = Passport::new();
        }
        let fields = line.split_whitespace().collect::<Vec<_>>();
        for field in fields {
            let f = field.split(":").collect::<Vec<_>>();
            let (key, value) = (f[0], f[1]);
            pass.insert(key, value);
        }
    }
    valid_passports_count
}
