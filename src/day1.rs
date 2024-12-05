use std::{
    fs::File,
    io::{prelude::*, BufReader},
    path::Path,
};

pub fn clean_input() -> (Vec<i32>, Vec<i32>) {
    let file = File::open(Path::new("input/day1.txt")).expect("No such file exists");
    let buf = BufReader::new(file);

    let mut left_side = Vec::new();
    let mut right_side = Vec::new();

    for line in buf.lines() {
        let line = line.expect("Could not read line");
        let split = line.split_whitespace().collect::<Vec<&str>>();
        let left = split[0].to_string().parse::<i32>().expect("Could not parse left side value to integer");
        let right = split[1].to_string().parse::<i32>().expect("Could not parse right side value to integer");
        left_side.push(left);
        right_side.push(right);
    }

    left_side.sort();
    right_side.sort();

    return (left_side, right_side);
}

pub fn get_distance(left_side: &Vec<i32>, right_side: &Vec<i32>) {
    let mut sum = 0;
    // Zip the two vectors together
    for (left, right) in left_side.iter().zip(right_side.iter()) {
        // Get the absolute difference between the two values and add it to the sum
        sum += (left - right).abs();
    }
    println!("Day 1 distance: {}", sum);
}

pub fn get_similarity(left_side: &Vec<i32>, right_side: &Vec<i32>) {
    let mut similarity = 0;
    for left in left_side.iter() {
        let val = right_side.iter().filter(|right| right == &left).count() as i32 * left;
        similarity += val;
    }
    println!("Day 1 similarity: {}", similarity);
}

pub fn get_answer() {
    let (left_side, right_side) = clean_input();
    get_distance(&left_side, &right_side);
    get_similarity(&left_side, &right_side);
}