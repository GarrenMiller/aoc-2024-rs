use std::{
    fs::File,
    io::{prelude::*, BufReader},
    path::Path
};

use log::{debug, error, info, warn, trace};
use env_logger;

#[derive(Debug, Clone, PartialEq, Eq)]
struct WordMatrix {
    grid: Vec<Vec<char>>,
    found: Vec<(usize, usize, Direction)>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Direction {
    Down,
    Right,
    LeftDown,
    RightDown,
}

impl Direction {
    fn iterator() -> impl Iterator<Item = Direction> {
        [
            Direction::Down,
            Direction::Right,
            Direction::LeftDown,
            Direction::RightDown,
        ]
        .iter()
        .copied()
    }

    fn values(&self) -> (i32, i32) {
        match self {
            Direction::Down => (1, 0),
            Direction::Right => (0, 1),
            Direction::LeftDown => (1, -1),
            Direction::RightDown => (1, 1),
        }
    }
}

impl WordMatrix {
    fn new() -> Self {
        WordMatrix {
            grid: Vec::new(),
            found: Vec::new(),
        }
    }

    pub fn add_row(&mut self, row: Vec<char>) {
        self.grid.push(row);
    }

    pub fn read_direction(&self, row: usize, col: usize, direction: Direction) -> Option<&char> {
        let mut result = None;
        match direction {
            direction => {
                let (row_offset, col_offset) = direction.values();
                result = self.grid.get((row as i32 + row_offset) as usize)?.get((col as i32 + col_offset) as usize);
            },
        }
        return result;
    }

    pub fn char_is_present(&self, row: usize, col: usize, target: &char) -> Vec<Direction> {
        let mut directions = Vec::new();
        for direction in Direction::iterator() {
            if self
                .read_direction(row, col, direction)
                .map_or(false, |c| c == target) {
                    directions.push(direction);
            }
        }
        return directions;
    }

    pub fn try_finish_word<I>(&mut self, row: usize, col: usize, mut chars: I) where I: Iterator<Item = char> + Clone, {
        // Success == count + 1 because we're starting with the second char
        let num_letters_for_success = chars.clone().count() + 1;
        let mut num_letters_found = 2;

        // Get all directions where the second character is present
        let second_char = chars.next().unwrap();
        let valid_directions = self.char_is_present(row, col, &second_char);

        // Cycle through all valid directions
        for dir in valid_directions {
            let (dx, dy) = dir.values();
            let mut pos = (row as i32 + dx, col as i32 + dy);
            let mut char_list = chars.clone();

            // Check if word is present in the direction           
            while let Some(next_char) = char_list.next() {
                if self
                    .read_direction(pos.0 as usize, pos.1 as usize, dir)
                    .map_or(false, |c| c == &next_char) {
                        pos.0 += dx;
                        pos.1 += dy;
                        num_letters_found += 1;
                } else {
                    break;
                }
            }

            // If the word is present, add it to the found list
            if num_letters_found == num_letters_for_success {
                self.found.push((row, col, dir));
            }

            // Else reset the number of letters found
            num_letters_found = 2;
        }
    }

    pub fn find_word_instances(&mut self, word: &str) {
        self.found.clear();
        // Iterate through characters in the grid
        for row in 0..self.grid.len() {
            for col in 0..self.grid[row].len() {
                // Trigger a check for the word if first or last character is detected
                if let (Some(start), Some(end)) = (word.chars().next(), word.chars().last()) {
                    let current = self.grid[row][col];
                    if current == start {
                        self.try_finish_word(row, col, word.chars().skip(1));
                    }
                    else if current == end {
                        self.try_finish_word(row, col, word.chars().rev().skip(1));
                    }
                }
            }
        }
    }

    pub fn find_crosses(&mut self, word_length: usize) -> i32 {
        let mut crosses = 0;
        for word in &self.found {
            if word.2 == Direction::RightDown && (word.1 + (word_length - 1) > self.grid[0].len() - 1) {
                continue;
            }
            let mirrored_dir = match word.2 {
                Direction::Down => None,
                Direction::Right => None,
                Direction::LeftDown => None,
                Direction::RightDown => Some(Direction::LeftDown),
            };
            if mirrored_dir.is_none() {
                continue;
            }
            // Go to the right by word_length - 1
            let mirror = (word.0, word.1 + (word_length - 1), mirrored_dir.unwrap());
            if mirror.1 > self.grid[0].len() - 1 {
                continue;
            }
            if self.found.contains(&mirror) {
                crosses += 1;
            }
        }
        return crosses;
    }
}

fn get_input() -> WordMatrix {
    let file = File::open(Path::new("input/day4.txt")).expect("Couldn't open input file");
    let buf = BufReader::new(file);
    let mut results = WordMatrix::new();
    for line in buf.lines() {
        let line = line.expect("Couldn't read line");
        let chars = line.chars().collect::<Vec<char>>();
        results.add_row(chars);
    }
    return results;
}

pub fn get_answers() {
    env_logger::init();
    let mut input = get_input();
    input.find_word_instances("XMAS");
    println!("Day 4 part 1 instances of word: {:?}", input.found.len());
    input.find_word_instances("MAS");
    println!("Day 4 part 2 crosses: {:?}", input.find_crosses("MAS".len()));
}