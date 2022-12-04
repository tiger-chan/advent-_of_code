use std::fs;
use std::ops::Add;

#[derive(PartialEq, Eq, PartialOrd, Ord, Copy, Clone)]
enum Hand {
    Rock = 0,
    Paper = 1,
    Scissors = 2,
}

enum GameResult {
    Lose = 0,
    Draw = 1,
    Win = 2,
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
        _ => panic!("Unexpected input"),
    }
}

fn p2_result(c: Option<char>) -> GameResult {
    match c {
        Some('X') => GameResult::Lose,
        Some('Y') => GameResult::Draw,
        Some('Z') => GameResult::Win,
        _ => panic!("Unexpected input"),
    }
}

fn p2_hand(result: GameResult, hand: Hand) -> Hand {
    match result {
        GameResult::Draw => hand,
        GameResult::Win => hand + 1,
        GameResult::Lose => hand + 2, // 3 for the full rotation but we want one less
    }
}

fn main() {
    let data_str = fs::read_to_string("./data/paper_rock_scissors.txt").unwrap();

    let sign_values: [i32; 3] = [1, 2, 3];
    let mut score: i32 = 0;

    for entry in data_str.split('\n') {
        let p1 = p1_hand(entry.chars().nth(0));
        let p2_result = p2_result(entry.chars().nth(2));
        let p2 = p2_hand(p2_result, p1);

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

    println!("Score: {}", score);
}
