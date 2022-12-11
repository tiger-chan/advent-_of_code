use std::fs;

mod vector;

use crate::vector::Vector;

fn main() {
    let data_str = fs::read_to_string("./data/day_09.txt").unwrap();

    let mut head: Vector = Vector::new(0, 0);
    let mut tail: Vector = Vector::new(0, 0);
    let mut positions: Vec<Vector> = Vec::new();
    positions.push(tail);

    for line in data_str.split("\n") {
        if line.len() < 3 {
            panic!("Malformed input file");
        }
        let offset = match &line[..1] {
            "U" => Vector::up(),
            "D" => Vector::down(),
            "L" => Vector::left(),
            "R" => Vector::right(),
            _ => panic!("Malformed input file"),
        };

        let repeat = if let Ok(n) = u32::from_str_radix(&line[2..], 10) {
            n
        } else {
            panic!("Malformed input file");
        };

        for _ in 0..repeat {
            head += offset;

            if (head - tail).mag() > 1.5 {
                // too far away so move the tail closer
                tail = head - offset;
                positions.push(tail);
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
