use std::{env::current_dir, fs};

fn approach_1() -> u32 {
    //println!("{:?}", current_dir().unwrap());
    let a_offset = 'A' as u32 - 1;
    let mut input = fs::read_to_string("inputs/0022_names.txt")
        .expect("Should have been able to read the file");
    input = input.replace("\"", "");
    let mut v :Vec<&str> = input.split(",").collect();
    v.sort();

    v.iter()
        .enumerate()
        .map(|(i, name)| name.chars().map(|c| (c as u32 - a_offset)*(1+i as u32)).sum::<u32>())
        .sum()
}

fn main() {
    let sol = approach_1();
    println!(" Sum of all namescores is {sol}");
}
