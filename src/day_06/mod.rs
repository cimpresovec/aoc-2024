use crate::file_handling;
use std::sync::{Arc, Mutex};
use std::thread::{spawn, JoinHandle};

pub fn second_puzzle() -> u32 {
    let grid = parse_input();
    let position = find_starting_position(&grid);

    // try all of them
    let count = Arc::new(Mutex::new(0));
    let mut handles: Vec<JoinHandle<()>> = vec![];
    for y in 0..grid.len() {
        for x in 0..grid[y].len() {
            if grid[y][x] == b'.' {
                let mut c_grid = grid.clone();
                let c_pos = position.clone();
                let c_count = Arc::clone(&count);
                handles.push(spawn(move || {
                    c_grid[y][x] = b'#';

                    if navigate_loop(&mut c_grid, c_pos) {
                        let mut c = c_count.lock().unwrap();
                        *c += 1;
                        // count += 1;
                    }
                }));
            }
        }
    }

    for x in handles {
        x.join().unwrap();
    }

    let count = count.lock().unwrap();
    *count
}

pub fn first_puzzle() -> u32 {
    let mut grid = parse_input();
    // print_grid(&grid);

    let position = find_starting_position(&grid);

    navigate(&mut grid, position);
    // print_grid(&grid);

    let i: u32 = grid
        .iter()
        .map(|line| line.iter().filter(|c| **c == b'X').count() as u32)
        .sum();
    i
}

#[warn(dead_code)]
fn print_grid(grid: &Vec<Vec<u8>>) {
    for y in 0..grid.len() {
        for x in 0..grid[y].len() {
            print!("{}", grid[y][x] as char);
        }
        println!();
    }
}

fn navigate_loop(grid: &mut Vec<Vec<u8>>, mut position: Position) -> bool {
    let mut direction = Direction::Up;
    let mut steps = 0;

    // If it takes too long I guess we are in a loop
    while steps < 20000 {
        steps += 1;
        // Mark as visited
        grid[position.1][position.0] = b'X';

        match direction {
            Direction::Up => {
                if position.1 == 0 {
                    return false;
                }
                if grid[position.1 - 1][position.0] == b'#' {
                    direction = Direction::Right;
                } else {
                    position.1 -= 1;
                }
            }
            Direction::Down => {
                if position.1 == grid.len() - 1 {
                    return false;
                }
                if grid[position.1 + 1][position.0] == b'#' {
                    direction = Direction::Left;
                } else {
                    position.1 += 1;
                }
            }
            Direction::Left => {
                if position.0 == 0 {
                    return false;
                }
                if grid[position.1][position.0 - 1] == b'#' {
                    direction = Direction::Up;
                } else {
                    position.0 -= 1;
                }
            }
            Direction::Right => {
                if position.0 == grid[0].len() - 1 {
                    return false;
                }
                if grid[position.1][position.0 + 1] == b'#' {
                    direction = Direction::Down;
                } else {
                    position.0 += 1;
                }
            }
        }
    }

    true
}

fn navigate(grid: &mut Vec<Vec<u8>>, mut position: Position) {
    let mut direction = Direction::Up;

    loop {
        // Mark as visited
        grid[position.1][position.0] = b'X';

        match direction {
            Direction::Up => {
                if position.1 == 0 {
                    return;
                }
                if grid[position.1 - 1][position.0] == b'#' {
                    direction = Direction::Right;
                } else {
                    position.1 -= 1;
                }
            }
            Direction::Down => {
                if position.1 == grid.len() - 1 {
                    return;
                }
                if grid[position.1 + 1][position.0] == b'#' {
                    direction = Direction::Left;
                } else {
                    position.1 += 1;
                }
            }
            Direction::Left => {
                if position.0 == 0 {
                    return;
                }
                if grid[position.1][position.0 - 1] == b'#' {
                    direction = Direction::Up;
                } else {
                    position.0 -= 1;
                }
            }
            Direction::Right => {
                if position.0 == grid[0].len() - 1 {
                    return;
                }
                if grid[position.1][position.0 + 1] == b'#' {
                    direction = Direction::Down;
                } else {
                    position.0 += 1;
                }
            }
        }
    }
}

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

type Position = (usize, usize);
fn find_starting_position(grid: &Vec<Vec<u8>>) -> Position {
    for y in 0..grid.len() {
        for x in 0..grid[y].len() {
            if grid[y][x] == b'^' {
                return (x, y);
            }
        }
    }
    unreachable!("No starting position found!")
}

fn parse_input() -> Vec<Vec<u8>> {
    let file_content = file_handling::read_day_input(6);
    let grid: Vec<Vec<u8>> = file_content.lines().map(|l| l.bytes().collect()).collect();
    grid
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_first_puzzle() {
        assert_eq!(first_puzzle(), 5461)
    }

    #[test]
    fn test_second_puzzle() {
        assert_eq!(second_puzzle(), 1836)
    }
}
