use std::io::{BufRead, BufReader};
use std::fs::File;
// TODO:
// 1. Parse ordering rules and sort them by number
// 2. Determine all rules that apply
// 3. Check if the orders are matched
// 4. For all valid updates, sum up the middle numbers


fn parse_ordering_rules<R: std::io::Read>(reader: &mut BufReader<R>) -> Vec<(i32, i32)> {
    let mut rules = Vec::new();
    for line in reader.lines() {
        let rule = line.unwrap();
        if rule.trim().is_empty() {
            break;
        }
        let parsed_rule = rule
            .split('|')
            .map(|s| s.trim().parse::<i32>().expect("Could not parse number in rule"))
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
            .map(|s| s.trim().parse::<i32>().expect("Could not parse number in level"))
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

fn get_applicable_rules<'a>(rules: &'a mut Vec<(i32, i32)>, level: &'a Vec<i32>) -> Vec<&'a (i32, i32)> {
    let applicable = rules
            .iter()
            .filter(|r| level.contains(&r.0))
            .collect::<Vec<&(i32, i32)>>();
    return applicable;
}

fn level_passes_rules(rules: Vec<&(i32, i32)>, level: Vec<i32>) -> bool {
    let mut pass = true;
    for rule in rules {
        let forerunner_index = level.iter().position(|&x| x == rule.0).expect("forerunner not found");
        let caboose_index = level.iter().position(|&x| x == rule.1);
        if caboose_index.is_none() {
            continue;
        }
        if caboose_index.unwrap() < forerunner_index {
            pass = false;
        }
    }
    return pass;
}

fn get_passing_levels_sum() -> i32 {
    let (mut rules, levels) = get_input();
    let mut sum = 0;
    levels
        .iter()
        .filter(|l| level_passes_rules(get_applicable_rules(&mut rules, l), l.to_vec()))
        .for_each(|l| {
            let middle_index = l.len() / 2;
            sum += l[middle_index];
        });
    return sum;
}

pub fn get_answers() {
    let passing_sum = get_passing_levels_sum();
    println!("Day 5 part 1 passing sum: {:?}", passing_sum);
}