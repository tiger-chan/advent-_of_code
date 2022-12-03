use std::fs;

fn main() {
    let _data_str = fs::read_to_string("./data/rucksacks.txt").unwrap();

	let priority: i32 = 0;
    
    print!("Priority: {}\n", priority);
}
