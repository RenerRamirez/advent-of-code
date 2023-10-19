use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file = fs::read_to_string(&args[1]);

    let mut sum: u32 = 0;
    let mut elf_number = 1;
    let mut result_field = 0u64;
    for a in file.expect("Proper input").lines() {
        let mut elf_field = 0u64;
        let elf = &a[0..a.len()];

        for c in elf.chars() {
            elf_field = elf_field | 2_u64.pow((c as u32) % 65);
        }

        if result_field != 0 {
            result_field = result_field & elf_field;
        } else {
            result_field = elf_field;
        }

        if elf_number % 3 == 0 {
            let c_in_common = result_field.ilog2();
            let ch: char = char::from_u32(c_in_common).unwrap();

            if (ch as u32 + 65) < 91 {
                sum += (ch as u32 % 65) + 1 + 26;
            } else {
                sum += (ch as u32 % 65) - 5 - 26;
            }

            elf_number = 0;
            result_field = 0u64;
        }
        elf_number += 1;
    }
    println!("{}", sum);
}
