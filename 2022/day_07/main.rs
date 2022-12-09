use std::fs;

fn main() {
    let _data_str = fs::read_to_string("./data/os.txt").unwrap();

    println!("[2022][07]");
}
