#[derive(PartialEq, Clone)]
enum RPS {
    Rock,
    Paper,
    Scissors,
    Invalid,
}

#[derive(PartialEq)]
enum Score {
    Loss,
    Draw,
    Win,
    Invalid,
}

impl Score {
    fn value(&self) -> i32 {
        match self {
            Score::Loss => 0,
            Score::Draw => 3,
            Score::Win => 6,
            _ => 0 
        }
    } 
}

impl RPS {
    fn value(&self) -> i32 {
        match self {
            RPS::Rock => 1,
            RPS::Paper => 2,
            RPS::Scissors => 3,
            _ => 0 
        }
    }
}

use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file = fs::read_to_string(&args[1]);
    let mut opponent = RPS::Invalid;
    let mut outcome = Score::Invalid;
    let mut you = RPS::Invalid;
    let mut score = 0;
    for c in file.expect("Malformed file").chars() {
        match c {
            'A' => {
                opponent = RPS::Rock;
                continue;
            },
            'B' => {
                opponent = RPS::Paper;
                continue;
            },
            'C' => {
                opponent = RPS::Scissors;
                continue;
            },
            ' ' => continue,
            '\n' => continue,
            _ => {},
        }
        match c {
            'X' => {
                outcome = Score::Loss;  
            },
            'Y' => {
                outcome = Score::Draw;  
            },
            'Z' => {
                outcome = Score::Win;  
            },
            _ => { },
        }

        match outcome {
            Score::Draw => {
                score += Score::Draw.value();
                you = opponent.clone();
            }
            Score::Loss => {
                score += Score::Loss.value();
                match opponent {
                    RPS::Rock => you = RPS::Scissors,
                    RPS::Paper => you = RPS::Rock,
                    RPS::Scissors => you = RPS::Paper,
                    _ => {}
                }
            },
            Score::Win => {
                score += Score::Win.value();
                match opponent {
                    RPS::Rock => you = RPS::Paper,
                    RPS::Paper => you = RPS::Scissors,
                    RPS::Scissors => you = RPS::Rock,
                    _ => {}
                }
            }
            _ => {}
        }
        score += you.value();
    }
    println!("{}", score);
}
