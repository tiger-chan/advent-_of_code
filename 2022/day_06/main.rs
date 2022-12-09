use std::fs;

const SIZE: usize = 14;

fn all_unique(d: &[char; SIZE]) -> bool {
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

    if data_str.len() < SIZE {
        return;
    }

    let mut data: [char; SIZE] = data_str
        .chars()
        .take(SIZE)
        .collect::<Vec<char>>()
        .try_into()
        .unwrap();
    let mut i = SIZE - 1;
    let mut result = 0;
    for n in (SIZE - 1)..data_str.len() {
        data[i] = data_str.chars().nth(n).unwrap();
        i = (i + 1) % SIZE;

        if all_unique(&data) {
            result = n + 1;
            break;
        }
    }

    println!("[2022][06] {}", result);
}
