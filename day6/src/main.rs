use std::collections::HashMap;

fn main() {
    let input = include_str!("../input");

    let parsed = parse_input(input);

    let result = walk(&parsed);
    println!("result : {result}");
}

pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}
pub fn walk(data: &[Vec<u8>]) -> usize {
    let (mut x, mut y) = find_cursor(data).unwrap();
    let mut visited: HashMap<(usize, usize), ()> = HashMap::new();
    visited.entry((x, y)).or_default();

    let mut direction = Direction::Up;

    while !is_end(data, (x, y)) {
        visited.entry((x, y)).or_default();

        match direction {
            Direction::Up => {
                if can_walk(data, (x, y - 1)) {
                    y -= 1;
                } else {
                    x += 1;
                    direction = Direction::Right;
                }
            }
            Direction::Down => {
                if can_walk(data, (x, y + 1)) {
                    y += 1;
                } else {
                    x -= 1;
                    direction = Direction::Left;
                }
            }
            Direction::Left => {
                if can_walk(data, (x - 1, y)) {
                    x -= 1;
                } else {
                    y -= 1;
                    direction = Direction::Up;
                }
            }
            Direction::Right => {
                if can_walk(data, (x + 1, y)) {
                    x += 1;
                } else {
                    y += 1;
                    direction = Direction::Down
                }
            }
        }
    }

    visited.entry((x, y)).or_default();
    visited.len()
}

pub fn can_walk(data: &[Vec<u8>], (x, y): (usize, usize)) -> bool {
    x < data[0].len() && y < data.len() && data[y][x] != b'#'
}

pub fn is_end(data: &[Vec<u8>], (x, y): (usize, usize)) -> bool {
    (x == 0 || y == 0 || x == data[0].len() || y == data.len()) && data[y][x] != b'#'
}

pub fn parse_input(source: &str) -> Vec<Vec<u8>> {
    source
        .lines()
        .map(|line| line.trim().as_bytes().to_vec())
        .filter(|line| !line.is_empty())
        .collect()
}

pub fn find_cursor(data: &[Vec<u8>]) -> Option<(usize, usize)> {
    for x in 0..data[0].len() {
        for y in 0..data.len() {
            if data[y][x] == b'^' {
                return Some((x, y));
            }
        }
    }

    None
}
