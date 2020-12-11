use std::fs;
use std::io::{self, BufRead};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(2346, pt1());
    }

    #[test]
    fn it_works_pt2() {
        assert_eq!(2111, pt2());
    }
}

fn read_file() -> io::Lines<io::BufReader<fs::File>> {
    let file = fs::File::open("./input.txt").unwrap();
    io::BufReader::new(file).lines()
}

fn update_seats(seats: Vec<Vec<char>>, is_pt_2: bool) -> i64 {
    let mut change = false;
    let mut tmp_seats = seats.clone();
    let mut count = 0;

    for (i, row) in seats.iter().enumerate() {
        for (j, col) in row.iter().enumerate() {
            let mut occ = 0;
            for dr in -1..=1 {
                for dc in -1..=1 {
                    if !(dr == 0 && dc == 0) {
                        let mut cr = i as i32 + dr;
                        let mut cc = j as i32 + dc;
                        while is_pt_2
                            && cr >= 0
                            && cr < seats.len() as i32
                            && cc >= 0
                            && cc < row.len() as i32
                            && seats[cr as usize][cc as usize] == '.'
                        {
                            cr += dr;
                            cc += dc;
                        }
                        if let Some(cur_r) = seats.get(cr as usize) {
                            if let Some(cur_c) = cur_r.get(cc as usize) {
                                match cur_c {
                                    '#' => occ += 1,
                                    _ => (),
                                }
                            }
                        }
                    }
                }
            }

            let limit = if is_pt_2 { 5 } else { 4 };
            if *col == 'L' && occ == 0 {
                tmp_seats[i][j] = '#';
                change = true;
            } else if *col == '#' {
                count += 1;
                if occ >= limit {
                    tmp_seats[i][j] = 'L';
                    change = true;
                }
            }
        }
    }

    if change {
        return update_seats(tmp_seats, is_pt_2);
    }

    count
}

fn create_grid() -> Vec<Vec<char>> {
    read_file()
        .map(|n| {
            let n = n.unwrap();
            return n.chars().collect::<Vec<_>>();
        })
        .collect::<Vec<Vec<_>>>()
}
pub fn pt1() -> i64 {
    let grid = create_grid();
    update_seats(grid, false)
}

pub fn pt2() -> i64 {
    let grid = create_grid();
    update_seats(grid, true)
}
