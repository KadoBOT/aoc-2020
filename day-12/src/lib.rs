use std::fs;
use std::io::{self, BufRead};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(1645, pt1());
    }

    #[test]
    fn it_works_pt2() {
        assert_eq!(35292, pt2());
    }
}

#[derive(Clone, Debug)]
enum Direction {
    EAST,
    SOUTH,
    WEST,
    NORTH,
    FORWARD,
}

#[derive(Debug, PartialEq)]
enum MoveBy {
    QTY,
    UNIT,
}

#[derive(Debug)]
struct Ship {
    coord: (i32, i32),
    dir: Direction,
    move_by: MoveBy,
    waypoint: (i32, i32),
}

impl Ship {
    pub fn new() -> Self {
        Ship {
            coord: (0, 0),
            dir: Direction::EAST,
            move_by: MoveBy::QTY,
            waypoint: (10, 1),
        }
    }

    pub fn set(&mut self, ins: &str, qty: i32) {
        if ins == "R" || ins == "L" {
            self.turn(ins);
            if qty > 90 {
                self.set(ins, qty - 90);
            }
        } else {
            let dir = match ins {
                "E" => Direction::EAST,
                "N" => Direction::NORTH,
                "S" => Direction::SOUTH,
                "W" => Direction::WEST,
                _ => Direction::FORWARD,
            };

            self.move_ship(dir, qty);
        }
    }

    pub fn move_by(&mut self, m: MoveBy) {
        self.move_by = m;
    }

    fn move_ship(&mut self, dir: Direction, qty: i32) {
        match self.move_by {
            MoveBy::QTY => self.move_ship_qty(dir, qty),
            MoveBy::UNIT => self.move_ship_unit(dir, qty),
        }
    }

    fn move_ship_qty(&mut self, dir: Direction, qty: i32) {
        match dir {
            Direction::EAST => self.coord = (self.coord.0 + qty, self.coord.1),
            Direction::SOUTH => self.coord = (self.coord.0, self.coord.1 - qty),
            Direction::WEST => self.coord = (self.coord.0 - qty, self.coord.1),
            Direction::NORTH => self.coord = (self.coord.0, self.coord.1 + qty),
            Direction::FORWARD => self.move_ship_qty(self.dir.clone(), qty),
        }
    }

    fn move_ship_unit(&mut self, dir: Direction, qty: i32) {
        match dir {
            Direction::EAST => self.waypoint = (self.waypoint.0 + qty, self.waypoint.1),
            Direction::SOUTH => self.waypoint = (self.waypoint.0, self.waypoint.1 - qty),
            Direction::WEST => self.waypoint = (self.waypoint.0 - qty, self.waypoint.1),
            Direction::NORTH => self.waypoint = (self.waypoint.0, self.waypoint.1 + qty),
            Direction::FORWARD => {
                self.coord = (
                    self.coord.0 + (self.waypoint.0 * qty),
                    self.coord.1 + (self.waypoint.1 * qty),
                )
            }
        }
    }

    fn turn(&mut self, s: &str) {
        match self.move_by {
            MoveBy::QTY => self.turn_qty(s),
            MoveBy::UNIT => self.turn_unit(s),
        }
    }

    fn turn_unit(&mut self, s: &str) {
        match s {
            "R" => self.waypoint = (self.waypoint.1, -self.waypoint.0),
            _ => self.waypoint = (-self.waypoint.1, self.waypoint.0),
        }
    }

    fn turn_qty(&mut self, s: &str) {
        if s == "R" {
            match self.dir {
                Direction::EAST => self.dir = Direction::SOUTH,
                Direction::SOUTH => self.dir = Direction::WEST,
                Direction::WEST => self.dir = Direction::NORTH,
                Direction::NORTH => self.dir = Direction::EAST,
                _ => (),
            }
        } else {
            match self.dir {
                Direction::EAST => self.dir = Direction::NORTH,
                Direction::SOUTH => self.dir = Direction::EAST,
                Direction::WEST => self.dir = Direction::SOUTH,
                Direction::NORTH => self.dir = Direction::WEST,
                _ => (),
            }
        }
    }

    pub fn manhattan_distance(&self) -> i32 {
        self.coord.0.abs() + self.coord.1.abs()
    }
}

fn read_file() -> io::Lines<io::BufReader<fs::File>> {
    // let file = fs::File::open("./sample.txt").unwrap();
    let file = fs::File::open("./input.txt").unwrap();
    io::BufReader::new(file).lines()
}

pub fn pt1() -> i32 {
    let lines = read_file();
    let mut ship = Ship::new();

    for line in lines {
        let line = line.unwrap();
        let (ins, qty) = line.split_at(1);

        ship.set(ins, qty.parse::<i32>().unwrap());
    }

    ship.manhattan_distance()
}

pub fn pt2() -> i32 {
    let lines = read_file();
    let mut ship = Ship::new();
    ship.move_by(MoveBy::UNIT);

    for line in lines {
        let line = line.unwrap();
        let (ins, qty) = line.split_at(1);

        ship.set(ins, qty.parse::<i32>().unwrap());
    }

    ship.manhattan_distance()
}
