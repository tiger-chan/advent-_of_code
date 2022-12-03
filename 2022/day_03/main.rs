use std::fs;

struct Rucksack(pub u64, pub u64);

fn priority_index(c: char) -> usize {
	match c {
		'a'..='z' => (c as usize) - ('a' as usize),
		'A'..='Z' => 26 + (c as usize) - ('A' as usize),
		_ => 0
	}
}

fn main() {
    let data_str = fs::read_to_string("./data/rucksacks.txt").unwrap();

	let mut priority: usize = 0;
    for line in data_str.split("\n") {
        let mid = line.len() / 2;
        let mut sack: Rucksack = Rucksack(0, 0);

        for i in 0..mid {
			if let Some(c0) = line.chars().nth(i) {
				let i0 = priority_index(c0);
				let mask = 1 << i0;
				sack.0 |= mask;
				if (sack.1 & mask) >> i0 == 1 {
					priority += i0 + 1;
					break;
				}
			}
			if let Some(c1) = line.chars().nth(mid + i) {
				let i1 = priority_index(c1);
				let mask = 1 << i1;
				sack.1 |= mask;
				if (sack.0 & mask) >> i1 == 1 {
					priority += i1 + 1;
					break;
				}
			}
		}
    }

    print!("Priority: {}\n", priority);
}
