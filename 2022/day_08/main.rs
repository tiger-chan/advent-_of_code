use std::fs;

fn main() {
    let _data_str = fs::read_to_string("./data/height_map.txt").unwrap();

    println!("[2022][08]");
}
