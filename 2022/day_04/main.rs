use std::fs;

fn main() {
    let data_str = fs::read_to_string("./data/section_assignments.txt").unwrap();

	let conained_sets: u32 = 0;

    println!("Contained sets: {}", conained_sets);
}
