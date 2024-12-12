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

fn report_is_safe(report: &Vec<i32>) -> bool {
    let deltas = report.windows(2).map(|w| w[1] - w[0]).collect::<Vec<i32>>(); 
    let dangerous = deltas.windows(2).any(|w| {
        let (a, b) = (w[0], w[1]);
        (a.signum() != b.signum()) || (a.abs() > 3) || (b.abs() > 3) || (a.abs() == 0) || (b.abs() == 0)
    });

    return !dangerous;
}


fn num_safe_reports(reports: &mut Vec<Vec<i32>>) {
    let mut initial_safe = 0;
    let mut saved = 0;
    // Count the number of reports that are safe
    for report in reports {
        if report_is_safe(report) {
            initial_safe += 1;
        }
        else {
           for i in 0..report.len() {
               let mut new_report = report.clone();
               new_report.remove(i);
               if report_is_safe(&new_report) {
                   saved += 1;
                   break;
               }
           } 
        }
    }

    println!("Day 2 Part 1 safe count: {:?}", initial_safe);
    println!("Day 2 Part 2 safe count: {:?}", initial_safe + saved);
}

pub fn get_answers() {
    let mut reports = clean_input();
    num_safe_reports(&mut reports);
}