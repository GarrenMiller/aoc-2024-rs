use std::{
    fs::File,
    io::{prelude::*, BufReader},
    path::Path
};

struct WordMatrix {
    grid: Vec<Vec<char>>,
    found: Vec<(usize, usize, Direction)>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
    LeftUp,
    LeftDown,
    RightUp,
    RightDown,
}

impl Direction {
    fn iterator() -> impl Iterator<Item = Direction> {
        [
            Direction::Up,
            Direction::Down,
            Direction::Left,
            Direction::Right,
            Direction::LeftUp,
            Direction::LeftDown,
            Direction::RightUp,
            Direction::RightDown,
        ]
        .iter()
        .copied()
    }

    fn values(&self) -> (i32, i32) {
        match self {
            Direction::Up => (-1, 0),
            Direction::Down => (1, 0),
            Direction::Left => (0, -1),
            Direction::Right => (0, 1),
            Direction::LeftUp => (-1, -1),
            Direction::LeftDown => (1, -1),
            Direction::RightUp => (-1, 1),
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

    pub fn read_direction(&self, row: usize, col: usize, direction: Direction) -> Option<char> {
        let mut result = None;
        match direction {
            Direction::Up => {
                if row > 0 {
                    result = Some(self.grid[row - 1][col]);
                }
            },
            Direction::Down => {
                if row + 1 < self.grid.len() {
                    result = Some(self.grid[row + 1][col]);
                }
            },
            Direction::Left => {
                if col > 0 {
                    result = Some(self.grid[row][col - 1]);
                }
            },
            Direction::Right => {
                if col + 1 < self.grid[row].len() {
                    result = Some(self.grid[row][col + 1]);
                }
            },
            Direction::LeftUp => {
                if row > 0 && col > 0 {
                    result = Some(self.grid[row - 1][col - 1]);
                }
            },
            Direction::LeftDown => {
                if row + 1 < self.grid.len() && col > 0 {
                    result = Some(self.grid[row + 1][col - 1]);
                }
            },
            Direction::RightUp => {
                if row > 0 && col + 1 < self.grid[row].len() {
                    result = Some(self.grid[row - 1][col + 1]);
                }
            },
            Direction::RightDown => {
                if row + 1 < self.grid.len() && col + 1 < self.grid[row].len() {
                    result = Some(self.grid[row + 1][col + 1]);
                }
            },
        }
        return result;
    }

    pub fn char_is_present(&self, row: usize, col: usize, target: &char) -> Vec<Direction> {
        let mut directions = Vec::new();
        for direction in Direction::iterator() {
            if let Some(cell) = self.read_direction(row, col, direction) {
                if &cell == target {
                    let reversed_direction = reverse_vec(row, col, direction, 4);
                    if self.found.contains(&reversed_direction) {
                        println!("Skipping direction {:?} because it's already been found", direction);
                        continue;
                    }   
                    directions.push(direction);
                }
            }
        }
        println!("\t Found target character {:?} in directions: {:?}", target, directions);
        return directions;
    }

    pub fn check_for_word<I>(&mut self, row: usize, col: usize, mut chars: I) where I: Iterator<Item = char> + Clone, {
        // Check if the second character is present in the surrounding cells
        let (mut curr_row, mut curr_col) = (row as i32, col as i32);
        let second_char = chars.next().unwrap();
        let directions = self.char_is_present(row, col, &second_char);

        for dir in directions {
            let mut char_list = chars.clone();
            while let Some(next_char) = char_list.next() {
                curr_row += dir.values().0;
                curr_col += dir.values().1;
                if next_char == self.read_direction(curr_row as usize, curr_col as usize, dir).unwrap_or(' ') {
                    let dir_mags = dir.values();
                    println!("\t Found next character {:?} in direction {:?} at cell ({}, {})", next_char, dir, curr_row + dir_mags.0, curr_col + dir_mags.1);
                    continue;
                }
                else {
                    break;
                }
            }
            if char_list.next().is_none() {
                self.found.push((row, col, dir));
            }
        }
    }

    pub fn find_word_instances(&mut self, word: &str) {
        println!("Self grid: {:?}", self.grid);
        for row in 0..self.grid.len() {
            for col in 0..self.grid[row].len() {
                let start_char = word.chars().next().unwrap();
                let end_char = word.chars().last().unwrap();
                let current_char = self.grid[row][col];
                if &current_char == &start_char {
                    println!("Found start character X in cell ({}, {})", row, col);
                    let chars = word.chars().skip(1);
                    self.check_for_word(row, col, chars);
                }
                else if &current_char == &end_char {
                    println!("Found end character S in cell ({}, {})", row, col);
                    let chars = word.chars().rev().skip(1);
                    self.check_for_word(row, col, chars);
                }
            }
        }
        println!("Found {:?} vectors of the word", self.found);
        for row in &self.grid {
            println!("{:?}", row.iter().collect::<String>());
        }
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

fn reverse_vec(row: usize, column: usize, direction: Direction, word_length: i32) -> (usize, usize, Direction) {
    let opposite_direction = match direction {
        Direction::Up => Direction::Down,
        Direction::Down => Direction::Up,
        Direction::Left => Direction::Right,
        Direction::Right => Direction::Left,
        Direction::LeftUp => Direction::RightDown,
        Direction::LeftDown => Direction::RightUp,
        Direction::RightUp => Direction::LeftDown,
        Direction::RightDown => Direction::LeftUp,
    };
    let mut dir_vals = direction.values();
    let magnitude = word_length - 1;
    dir_vals = (row as i32 + dir_vals.0 * magnitude, column as i32 + dir_vals.1 * magnitude);
    println!("\t Reversed ({:?}, {:?}, {:?}) to ({:?}, {:?}, {:?})", row, column, direction, dir_vals.0, dir_vals.1, opposite_direction);
    return (dir_vals.0 as usize, dir_vals.1 as usize, opposite_direction);
} 

pub fn get_answers() {
    let mut input = get_input();
    input.find_word_instances("XMAS");
    println!("Day 4, Part 1: Found {} instances of the word", input.found.len());
}