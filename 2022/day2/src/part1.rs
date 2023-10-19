#[derive(PartialEq, Debug)]
enum RPS {
    Rock,
    Paper,
    Scissors,
    Invalid,
}

#[derive(Debug)]
enum Score {
    Loss,
    Draw,
    Win,
}

impl Score {
    fn value(&self) -> i32 {
        match self {
            Score::Loss => 0,
            Score::Draw => 3,
            Score::Win => 6
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
    let mut you = RPS::Invalid;
    let mut score = 0;
    for c in file.expect("Malformed file").chars() {
        match c {
            'A' => {
                // print!("Opponent choose rock. ");
                opponent = RPS::Rock;
                continue;
            },
            'B' => {
                // print!("Opponent choose paper. ");
                opponent = RPS::Paper;
                continue;
            },
            'C' => {
                // print!("Opponent choose scissors. ");
                opponent = RPS::Scissors;
                continue;
            },
            ' ' => continue,
            '\n' => continue,
            _ => {},
        }
        match c {
            'X' => {
                // print!("You choose rock. ");
                you = RPS::Rock;
            },
            'Y' => {
                // print!("You choose paper. ");
                you = RPS::Paper;
            },
            'Z' => {
                // print!("You choose scissors. ");
                you = RPS::Scissors;
            },
            _ => { },
        }

        if opponent == you {
            score += Score::Draw.value();
        } 
        else {
            match opponent {
                RPS::Rock => {
                    match you {
                        RPS::Scissors => score += Score::Loss.value(),
                        RPS::Paper => score += Score::Win.value(),
                        _ => {},
                    }
                } 
                RPS::Paper => {
                    match you {
                        RPS::Rock => score += Score::Loss.value(),
                        RPS::Scissors => score += Score::Win.value(),
                        _ => {},
                    }
                }
                RPS::Scissors => {
                    match you {
                        RPS::Paper => score += Score::Loss.value(),
                        RPS::Rock => score += Score::Win.value(),
                       _ => {},
                    }
                }
                _ => {}
            }
        } 
        score += you.value();
    }
    println!("{}", score);
}
