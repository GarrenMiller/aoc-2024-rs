use std::{
    fs::File,
    io::{prelude::*, BufReader},
    path::Path,
};

pub fn get_answer() {
    let file = File::open(Path::new("input/day1.txt")).expect("No such file exists");
    let buf = BufReader::new(file);

    let mut left_side = Vec::new();
    let mut right_side = Vec::new();
    let mut sum = 0;

    for line in buf.lines() {
        let line = line.expect("Could not read line");
        let split = line.split_whitespace().collect::<Vec<&str>>();
        left_side.push(split[0].to_string());
        right_side.push(split[1].to_string());
    }

    left_side.sort();
    right_side.sort();

    // Zip the two vectors together
    for (left, right) in left_side.iter().zip(right_side.iter()) {
        // Get the absolute difference between the two values and add it to the sum
        sum += (left.parse::<i32>().unwrap() - right.parse::<i32>().unwrap()).abs();
    }
    println!("Day 1 sum: {}", sum);
}