use std::{
    fs,
    ops::{Add, AddAssign, Neg},
};

mod vector;

use crate::vector::Vector;

const KNOT_COUNT: usize = 10;

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
    Nil,
}

impl Neg for Direction {
    type Output = Direction;
    fn neg(self) -> Self::Output {
        match self {
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
            Direction::Nil => Direction::Nil,
        }
    }
}

impl From<Direction> for Vector {
    fn from(d: Direction) -> Self {
        match d {
            Direction::Up => Vector::up(),
            Direction::Down => Vector::down(),
            Direction::Left => Vector::left(),
            Direction::Right => Vector::right(),
            Direction::Nil => Vector::new(0, 0),
        }
    }
}

impl From<Vector> for Direction {
    fn from(d: Vector) -> Self {
        match d {
            Vector { x: _, y: 1 } => Direction::Up,
            Vector { x: _, y: -1 } => Direction::Down,
            Vector { x: -1, y: _ } => Direction::Left,
            Vector { x: 1, y: _ } => Direction::Right,
            _ => Direction::Nil,
        }
    }
}

impl Add<Direction> for Vector {
    type Output = Vector;

    fn add(self, rhs: Direction) -> Self::Output {
        self + Vector::from(rhs)
    }
}

impl AddAssign<Direction> for Vector {
    fn add_assign(&mut self, rhs: Direction) {
        self.add_assign(Vector::from(rhs))
    }
}

fn main() {
    let data_str = fs::read_to_string("./data/day_09.txt").unwrap();

    let mut knots: [Vector; KNOT_COUNT] = [Vector::new(0, 0); KNOT_COUNT];
    let mut positions: Vec<Vector> = Vec::new();
    positions.push(knots[0]);

    for line in data_str.split("\n") {
        if line.len() < 3 {
            panic!("Malformed input file");
        }
        let dir = match &line[..1] {
            "U" => Direction::Up,
            "D" => Direction::Down,
            "L" => Direction::Left,
            "R" => Direction::Right,
            _ => panic!("Malformed input file"),
        };

        let repeat = if let Ok(n) = u32::from_str_radix(&line[2..], 10) {
            n
        } else {
            panic!("Malformed input file");
        };

        for _ in 0..repeat {
            knots[0] += dir;

            for k in 1..knots.len() {
                let delta = knots[k] - knots[k - 1];
                knots[k] = match delta.abs() {
                    Vector { x: 2, y: 2 } => knots[k - 1] + Vector::clamp(delta, -1, 1),
                    Vector { x: 2, y: _ } => {
                        knots[k - 1] + Vector::clamp(delta * Vector::right(), -1, 1)
                    }
                    Vector { x: _, y: 2 } => {
                        knots[k - 1] + Vector::clamp(delta * Vector::up(), -1, 1)
                    }
                    _ => knots[k],
                };
            }

            if let Some(&l) = knots.last() {
                positions.push(l);
            }
        }
    }

    positions.sort_unstable();
    positions.dedup();

    println!(
        "[2022][09] Unique occupied tail spaces: {}",
        positions.len()
    );
}
