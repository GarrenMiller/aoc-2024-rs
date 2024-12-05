use std::{
    fs::File,
    io::{prelude::*, BufReader},
    path::Path
};

fn clean_input() -> Vec<Vec<i32>> {
    let file = File::open(Path::new("input/day2.txt")).expect("Couldn't open input file");
    let buf = BufReader::new(file);
    let mut result = Vec::new();

    for line in buf.lines() {
        let line = line.expect("Couldn't parse line");
        let inner = line.split_whitespace().map(|c| c.parse::<i32>().expect("Cannot parse value in level to integer")).collect::<Vec<i32>>();
        result.push(inner);
    }
    return result;
}

fn level_is_safe(report: Vec<i32>) -> bool {
    let mut last_level: i32 = -1;
    let mut increased = false;
    let mut decreased = false;

    for level in report {
        // If first level, continue
        if last_level == -1 {
            last_level = level;
            continue;
        }

        let difference = level - last_level;
        let abs = difference.abs();

        // Decreased
        if difference < 0 {
            decreased = true;
        }

        // Increased
        else if difference > 0 {
            increased = true;
        }

        // Increased and decreased, or stayed the same
        if (increased && decreased) || (abs <  1 || abs > 3) {
            return false;
        }
        last_level = level;
    }
    return true;
}

fn num_of_safe_levels(levels: Vec<Vec<i32>>) {
    let mut count = 0;
    for level in levels {
        if level_is_safe(level) {
            count += 1;
        }
    }
    println!("Day 2 safe count: {}", count);
}

pub fn get_answers() {
    let reports = clean_input();
    num_of_safe_levels(reports);
}