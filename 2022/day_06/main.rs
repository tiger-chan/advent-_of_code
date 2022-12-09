use std::{fs};

fn all_unique(d: &[char; 4]) -> bool {
	for x in 0..d.len() {
		for y in (x + 1)..d.len() {
			if d[x] == d[y] {
				return false;
			}
		}
	}
	return true;
}

fn main() {
    let data_str = fs::read_to_string("./data/start_of_packet.txt").unwrap();

	if data_str.len() < 4 {
		return
	}

	let mut data: [char; 4] = data_str.chars().take(4).collect::<Vec<char>>().try_into().unwrap();
	let mut i = 3;
	let mut result = 0;
	for n in 3..data_str.len() {
		data[i] = data_str.chars().nth(n).unwrap();
		i = (i + 1) % 4;

		if all_unique(&data) {
			result = n + 1;
			break;
		}
	}

    println!("[2022][06] {}", result);
}
