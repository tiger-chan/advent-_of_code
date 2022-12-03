use std::fs;
use std::ops::Add;

#[derive(PartialEq, Eq, PartialOrd, Ord, Copy, Clone)]
enum Hand {
    Rock = 0,
    Paper = 1,
    Scissors = 2,
}

impl Add<i32> for Hand {
    type Output = Self;
    fn add(self, other: i32) -> Self::Output {
        if let Ok(x) = Hand::try_from((self as i32 + other) % 3) {
            x
        } else {
            Hand::Rock
        }
    }
}

impl TryFrom<i32> for Hand {
    type Error = &'static str;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Hand::Rock),
            1 => Ok(Hand::Paper),
            2 => Ok(Hand::Scissors),
            _ => Err("Unknown value"),
        }
    }
}

fn p1_hand(c: Option<char>) -> Hand {
    match c {
        Some('A') => Hand::Rock,
        Some('B') => Hand::Paper,
        Some('C') => Hand::Scissors,
        None => Hand::Rock,
        _ => panic!("Unexpected input"),
    }
}

fn p2_hand(c: Option<char>) -> Hand {
    match c {
        Some('X') => Hand::Rock,
        Some('Y') => Hand::Paper,
        Some('Z') => Hand::Scissors,
        None => Hand::Rock,
        _ => panic!("Unexpected input"),
    }
}

fn main() {
    let data_str = fs::read_to_string("./data/paper_rock_scissors.txt").unwrap();

	let sign_values: [i32; 3] = [1, 2, 3];
    let mut score: i32 = 0;

    for entry in data_str.split('\n') {
        let p1 = p1_hand(entry.chars().nth(0));
        let p2 = p2_hand(entry.chars().nth(2));

		score += &sign_values[p2 as usize];
        if p1 == p2 {
			// Tie
			score += 3;
        } else if (p1 + 1) == p2 {
			// P2 Win
			score += 6;
        } else {
			// P1 Win
        }
    }

    print!("Score: {}\n", score);
}
