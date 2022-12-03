use std::fs;
use std::ops::{Index, IndexMut};

struct Rucksack(pub u64, pub u64, pub u64);

impl Index<usize> for Rucksack {
    type Output = u64;

    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.0,
            1 => &self.1,
            2 => &self.2,
            _ => panic!("Index out pf range"),
        }
    }
}

impl IndexMut<usize> for Rucksack {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        match index {
            0 => &mut self.0,
            1 => &mut self.1,
            2 => &mut self.2,
            _ => panic!("Index out pf range"),
        }
    }
}

fn priority_index(c: char) -> usize {
    match c {
        'a'..='z' => (c as usize) - ('a' as usize),
        'A'..='Z' => 26 + (c as usize) - ('A' as usize),
        _ => 0,
    }
}

fn main() {
    let data_str = fs::read_to_string("./data/rucksacks.txt").unwrap();

    let mut priority: usize = 0;
    let mut split = data_str.split("\n").peekable();
    while let Some(_x) = split.peek() {
        let sacks: [Option<&str>; 3] = [split.next(), split.next(), split.next()];
        if sacks.iter().any(|n| n.is_none()) {
            panic!("Unexpected none in array");
        }

        let mut sack: Rucksack = Rucksack(0, 0, 0);

        'outer: for i in 0..sacks.len() {
            // add each item to the mask and test for the values already present
            if let Some(items) = sacks[i] {
                for c in items.chars() {
                    let idx = priority_index(c);
                    let mask = 1 << idx;
                    sack[i] |= mask;

                    if [sack.0, sack.1, sack.2]
                        .iter()
                        .all(|s| (s & mask) >> idx == 1)
                    {
                        priority += idx + 1;
                        break 'outer;
                    }
                }
            }
        }
    }

    print!("Priority: {}\n", priority);
}
