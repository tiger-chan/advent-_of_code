use std::fs;

fn main() {
    let _data_str = fs::read_to_string("./data/day_10.txt").unwrap();

	let signal_strength = 0;

    println!("[2022][10] signal strength: {}", signal_strength);
}
