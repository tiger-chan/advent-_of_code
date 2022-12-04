use std::fs;

fn main() {
    let elves_data = fs::read_to_string("./data/elf.txt").unwrap();

    let mut elves: [i32; 4] = [0, 0, 0, 0];
    for entry in elves_data.split('\n') {
        if let Ok(v) = i32::from_str_radix(entry, 10) {
            elves[0] += v;
        } else {
            for i in 1..elves.len() {
                if elves[i - 1] > elves[i] {
                    elves.swap(i - 1, i);
                }
            }
            elves[0] = 0;
        }
    }

    let sum: i32 = elves[1..elves.len()].iter().sum();

    println!("calories: {}", sum);
}
