use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file = fs::read_to_string(&args[1]);
    let mut sum: u32 = 0;
    // let alpha = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklMmnopqrstuvwxyz";

    // for a in alpha.lines() {
    for a in file.expect("Proper input").lines() {
        let first = &a[0..a.len() / 2];
        let second = &a[a.len() / 2..];

        let mut first_field = 0u64;
        let mut second_field = 0u64;

        for c in first.chars() {
            // println!("{}{}", c, c as u32);
            first_field = first_field | 2_u64.pow((c as u32) % 65);
            // println!("{:0>64b} => first ", first_field);
        }
        for c in second.chars() {
            // println!("{}{}", c, c as u32);
            second_field = second_field | 2_u64.pow((c as u32) % 65);
            // println!("{:0>64b} => second ", second_field);
        }
        // println!("{:0>64b} => first ", first_field);
        // println!("{:0>64b} => second ", second_field);

        let result_field = first_field & second_field;
        let c_in_common = result_field.ilog2();
        let ch: char = char::from_u32(c_in_common).unwrap();
        // println!("{:0>64b} => result ", result_field);

        if (ch as u32 + 65) < 91 {
            sum += (ch as u32 % 65) + 1 + 26;
        } else {
            sum += (ch as u32 % 65) - 5 - 26;
        }
    }
    println!("{}", sum);
}
