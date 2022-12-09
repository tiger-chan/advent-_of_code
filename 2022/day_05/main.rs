use std::{fs, collections::VecDeque};

fn is_between(l: char, u: char, c: char) -> bool {
	l <= c && c <= u
}

fn main() {
    let data_str = fs::read_to_string("./data/crates.txt").unwrap();

	let mut iter = data_str.split('\n');

	let create_count = if let Some(line) = iter.clone().next() {
		(line.len() / 4) + 1
	} else {
		panic!("Malformed input file");
	};

	let mut crates: Vec<VecDeque<char>> = Vec::new();
	crates.resize(create_count, VecDeque::new());

    'crate_list: while let Some(line) = iter.next() {
		if line.is_empty() {
			break 'crate_list;
		}

		for entry in 0..create_count {
			let value = if let Some(c) = line.chars().nth(entry * 4 + 1) {
				c
			} else {
				panic!("Malformed input file");
			};

			if is_between('l', '0', value) {
				break 'crate_list;
			}

			if is_between('A', 'Z', value) {
				crates[entry].push_front(value);
			}
		}
    }

	while let Some(line) = iter.next() {
		let split = line.split(' ');
		let split_val: Vec<&str> = split.clone().collect();
		let quantity = if let Some(x) = split_val.iter().nth(1) {
			usize::from_str_radix(x, 10).unwrap()
		} else {
			panic!("Malformed input file");
		};
		let src = if let Some(x) = split_val.iter().nth(3) {
			usize::from_str_radix(x, 10).unwrap() - 1
		} else {
			panic!("Malformed input file");
		};
		let dest = if let Some(x) = split_val.iter().nth(5) {
			usize::from_str_radix(x, 10).unwrap() - 1
		} else {
			panic!("Malformed input file");
		};

		for _ in 0..quantity {
			let src = &mut crates[src];
			if let Some(item) = src.pop_back() {
				let dest = &mut crates[dest];
				dest.push_back(item);
			}
		}
	}

	let mut top: String = String::new();
	for item in crates {
		if let Some(c) = item.back() {
			top.push(c.clone());
		}
	}

    println!("Top of crates stacks: {:?}", top);
}
