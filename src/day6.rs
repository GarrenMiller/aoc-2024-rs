use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Location {
    values: (usize, usize),
    visited: bool,
    is_obstacle: bool,
}

#[derive(Debug, Clone)]
struct Map {
    locations: HashMap<(usize, usize), Location>,
    guard_location: Location,
    guard_direction: Direction,
}

#[derive(Debug, Clone)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn get_guard_direction(guard: char) -> Direction {
    match guard {
        '^' => Direction::Up,
        'v' => Direction::Down,
        '<' => Direction::Left,
        '>' => Direction::Right,
        _ => panic!("Could not parse guard direction"),
    }
}

fn get_map() -> Map {
    let mut map = Map {
        locations: HashMap::<(usize, usize), Location>::new(),
        guard_location: Location {
            values: (0, 0),
            visited: false,
            is_obstacle: false,
        },
        guard_direction: Direction::Down,
    };
    let file = File::open("input/day6.txt").expect("Could not open input file");
    let reader = BufReader::new(file);
    for (r, line) in reader.lines().enumerate() {
        let line = line.unwrap();
        for (c, val) in line.chars().enumerate() {
            let mut location: Location = Location {
                values: (r, c),
                visited: false,
                is_obstacle: val == '#',
            };
            if ['^', 'v', '<', '>'].contains(&val) {
                location.visited = true;
                map.guard_location = location;
                map.guard_direction = get_guard_direction(val);
            }
            map.locations.insert((r, c), location);
        }
    }
    return map;
}

fn walk_through_map(map: &mut Map) {
    while let Some(next) = get_guard_next_location(map) {
        if next.is_obstacle {
            rotate_90_degrees(map);
            continue;
        }
        next.visited = true;
        map.guard_location = *next;
    }
}

fn rotate_90_degrees(map: &mut Map) {
    match map.guard_direction {
        Direction::Up => map.guard_direction = Direction::Right,
        Direction::Down => map.guard_direction = Direction::Left,
        Direction::Left => map.guard_direction = Direction::Up,
        Direction::Right => map.guard_direction = Direction::Down,
    }
}

fn get_guard_next_location(map: &mut Map) -> Option<&mut Location> {
    match map.guard_direction {
        Direction::Up => map
            .locations
            .get_mut(&(map.guard_location.values.0 - 1, map.guard_location.values.1)),
        Direction::Down => map
            .locations
            .get_mut(&(map.guard_location.values.0 + 1, map.guard_location.values.1)),
        Direction::Left => map
            .locations
            .get_mut(&(map.guard_location.values.0, map.guard_location.values.1 - 1)),
        Direction::Right => map
            .locations
            .get_mut(&(map.guard_location.values.0, map.guard_location.values.1 + 1)),
    }
}

fn get_total_visited(map: &Map) -> i32 {
    map.locations
        .iter()
        .map(|(_, l)| if l.visited { return 1 } else { return 0 })
        .sum()
}

fn part_one() {
    let mut map = get_map();
    walk_through_map(&mut map);
    println!(
        "Day 6 part 1 locations visited: {:?}",
        get_total_visited(&map)
    );
}

pub fn get_answers() {
    part_one();
    return;
}
