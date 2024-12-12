use std::{
    fs::File,
    io::{prelude::*, BufReader},
    path::Path
};
// Need to get all strings that are like mul(num,num)

// Use regex to match valid pattern of mul(num,num) and capture groups for the numbers
static REG_EXP: &str = r"mul\(([0-9]{1,3}),([0-9]{1,3})\)";

fn get_input() -> String {
    let file = File::open(Path::new("input/day3.txt")).expect("Couldn't open input file");
    let mut buf = BufReader::new(file);
    let mut contents = String::new();
    buf.read_to_string(&mut contents).expect("Couldn't read file to string");
    return contents;
}

fn day3_part1(input: String) {
    let re = regex::Regex::new(REG_EXP).expect("Couldn't compile regex");
    let mut products = Vec::new();
    for cap in re.captures_iter(&input) {
        let product = cap[1].parse::<i32>().unwrap() * cap[2].parse::<i32>().unwrap();
        products.push(product);
    }
    println!("Day 3 Part 1 product sum: {}", products.iter().sum::<i32>());
}


pub fn get_answers() {
    let input = get_input();
    day3_part1(input);
}