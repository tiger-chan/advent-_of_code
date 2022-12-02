use std::fs;

fn main() {
    let data_str = fs::read_to_string("./data/paper_rock_scissors.txt").unwrap();

    let score: i32 = 0;
    for _entry in data_str.split('\n') {
        
    }

    print!("Score: {}\n", score);
}
