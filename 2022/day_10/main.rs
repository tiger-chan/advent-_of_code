use std::fs;

const SIGNALS: usize = 256;

fn main() {
    let data_str = fs::read_to_string("./data/day_10.txt").unwrap();

    let mut signals: [i32; SIGNALS] = [0; SIGNALS];
    signals[0] = 1;

    let mut cycle: usize = 0;
    for line in data_str.split("\n") {
        match line {
            "noop" => {
                cycle = cycle + 1;
                signals[cycle] += signals[cycle - 1];
            }
            _ => {
                let num_str = &line[5..];
                if let Ok(num) = num_str.parse::<i32>() {
                    cycle = cycle + 2;
                    signals[cycle - 1] += signals[cycle - 2];
                    signals[cycle] += signals[cycle - 1];
                    signals[cycle] += num;
                }
            }
        }
    }

    for _ in 0..2 {
        cycle = cycle + 1;
        signals[cycle] += signals[cycle - 1];
    }

    let mut signal_points: [i32; 6] = [0; 6];
    for i in 0..signal_points.len() {
        let idx = 19 + i * 40;
        signal_points[i] = signals[idx] * (idx + 1) as i32;
    }

    let signal_str = signal_points.iter().fold(0, |x, y| x + y);

    for y in 0..6 {
        for x in 0..40 {
            let idx = x + y * 40;
            let signal = signals[idx];
            if ((x + 1) as i32) < signal || signal + 2 < ((x + 1) as i32) {
                print!(".");
            } else {
                print!("#");
            }
        }
        println!("");
    }

    println!("[2022][10] signal strength: {}", signal_str);
}
