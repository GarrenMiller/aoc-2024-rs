use std::cmp::Ordering;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn parse_ordering_rules<R: std::io::Read>(reader: &mut BufReader<R>) -> Vec<(i32, i32)> {
    let mut rules = Vec::new();
    for line in reader.lines() {
        let rule = line.unwrap();
        if rule.trim().is_empty() {
            break;
        }
        let parsed_rule = rule
            .split('|')
            .map(|s| {
                s.trim()
                    .parse::<i32>()
                    .expect("Could not parse number in rule")
            })
            .collect::<Vec<i32>>();
        rules.push((parsed_rule[0], parsed_rule[1]));
    }
    return rules;
}

fn parse_levels<R: std::io::Read>(reader: &mut BufReader<R>) -> Vec<Vec<i32>> {
    let mut levels = Vec::new();
    for line in reader.lines() {
        let level = line.unwrap();
        let parsed_level = level
            .split(',')
            .map(|s| {
                s.trim()
                    .parse::<i32>()
                    .expect("Could not parse number in level")
            })
            .collect::<Vec<i32>>();
        levels.push(parsed_level);
    }
    return levels;
}

fn get_input() -> (Vec<(i32, i32)>, Vec<Vec<i32>>) {
    let file = File::open("input/day5.txt").expect("Failed to open file");
    let mut reader = BufReader::new(file);
    let rules = parse_ordering_rules(&mut reader);
    let levels = parse_levels(&mut reader);
    return (rules, levels);
}

fn get_applicable_rules<'a>(rules: &'a [(i32, i32)], level: &[i32]) -> Vec<&'a (i32, i32)> {
    let applicable = rules
        .iter()
        .filter(|r| level.contains(&r.0))
        .collect::<Vec<&(i32, i32)>>();
    return applicable;
}

fn get_sums() -> (i32, i32) {
    let (rules, mut levels) = get_input();
    let mut sum = 0;
    let mut broken_sum = 0;

    for level in &mut levels {
        let applicable_rules = get_applicable_rules(&rules, &level);
        let compare = |x: &i32, y: &i32| {
            let (x, y) = (*x, *y);
            if applicable_rules.contains(&&(x, y)) {
                Ordering::Less
            } else if applicable_rules.contains(&&(y, x)) {
                Ordering::Greater
            } else {
                Ordering::Equal
            }
        };

        if level.is_sorted_by(|a, b| compare(a, b) != Ordering::Greater) {
            sum += level[level.len() / 2];
        } else {
            level.sort_by(compare);
            broken_sum += level[level.len() / 2];
        }
    }
    return (sum, broken_sum);
}

pub fn get_answers() {
    let sums = get_sums();
    println!("Day 5 part 1 passing sum: {:?}", sums.0);
    println!("Day 5 part 2 broken sum: {:?}", sums.1);
}
