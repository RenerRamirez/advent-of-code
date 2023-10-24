use std::env;
use std::fs;

fn parse_nums(line: &str) -> (i8, i8, i8, i8) {

    let dash_one_idx = line.find('-').unwrap();
    let x: i8 = line[0..dash_one_idx].parse().expect("not number");

    let comma_one_idx = line.find(',').unwrap();
    let y: i8 = line[dash_one_idx+1..comma_one_idx].parse().expect("not number");

    let dash_two_idx = &line[comma_one_idx..].find('-').unwrap() + comma_one_idx;
    let a: i8 = line[comma_one_idx+1..dash_two_idx].parse().expect("not number");
    let b: i8 = line[dash_two_idx+1..].parse().expect("not number");
    (x, y, a, b)
}

fn count_contains_loop(file: String) -> i32 {
    let mut count = 0;

    for line in file.lines() {

        let (x, y, a, b) = parse_nums(line);
        let xy_diff: i8 = y - x;
        let ab_diff: i8 = b - a;

        if xy_diff == ab_diff && x == a {
            count += 1;
        }        
        else if xy_diff > ab_diff && x <= a && y >= b {
            count += 1;
        } 
        else if xy_diff < ab_diff && x >= a && b >= y {
            count += 1;
        } 
    }
    count
}

fn count_overlaps_loop(file: String) -> i32 {
    let mut count = 0;

    for line in file.lines() {
        let (x, y, a, b) = parse_nums(line);

        if (x <= a || x <= b) && y >= a {
            count += 1;
        }
    }
    count
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let file = fs::read_to_string(&args[1]).expect("expected file");

    // println!("{}", count_contains_loop(file));
    println!("{}", count_overlaps_loop(file));
}
