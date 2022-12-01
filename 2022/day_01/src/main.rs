use std::fs;

fn main() {
    let elves_data = fs::read_to_string("./data/elf.txt").unwrap();
	let mut max_elf_id = 0;
	let mut max_elf_value = 0;
	let mut elf_id = 0;
	let mut cur_total = 0;
	for entry in elves_data.split('\n') {
		if let Ok(v) = i32::from_str_radix(entry, 10) {
			cur_total += v;
		} else {
			if max_elf_value <= cur_total {
				max_elf_id = elf_id;
				max_elf_value = cur_total;
			}
			cur_total = 0;
		}

		elf_id += 1;
	}

	print!("elf: {}, value: {}\n", max_elf_id, max_elf_value);
}
