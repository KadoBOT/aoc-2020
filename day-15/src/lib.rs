use std::collections::hash_map::Entry;
use std::collections::HashMap;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(981, solve(2020));
    }

    #[test]
    fn it_works_pt2() {
        assert_eq!(164878, solve(30_000_000));
    }
}

trait DB {
    fn modify_or_insert(self, idx: usize);
}

impl DB for Entry<'_, i64, (Option<usize>, Option<usize>)> {
    fn modify_or_insert(self, idx: usize) {
        self.and_modify(|(a, b)| {
            *a = *b;
            *b = Some(idx);
        })
        .or_insert((None, Some(idx)));
    }
}

const INPUT: &[i64] = &[8, 0, 17, 4, 1, 12];

pub fn solve(n: usize) -> i64 {
    let mut db = HashMap::new();

    let mut last = 12;

    for i in 0..n {
        if i <= INPUT.len() - 1 {
            db.insert(INPUT[i], (None, Some(i)));
            continue;
        }

        let n = db.get(&last).unwrap();
        if n.0.is_none() {
            last = 0;
            db.entry(0).modify_or_insert(i);
        } else {
            last = (n.1.unwrap() - n.0.unwrap()) as i64;
            db.entry(last).modify_or_insert(i);
        }
    }

    last
}
