use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file = fs::read_to_string(&args[1]);

    let mut max = 0;
    let mut sec = 0;
    let mut third = 0;
    let mut current = 0;
    for line in file.expect("No file").lines() {
        let total: i32;
        // println!("total {}", total);
        if line.len() == 0 {
            // println!("current {}", current);
            // println!("max {}", max);
            if current > max {
                third = sec;
                sec = max;
                max = current;
                // println!("max is now {}", max);
            } else if current > sec {
                third = sec;
                sec = current;
            } else if current > third {
                third = current;
            }
            current = 0;
            continue;
        }
        total = line.trim().parse().expect("Can't parse");
        current += total;
    }
    println!("{}", third + sec + max);
}
