use crate::file_handling;
use std::collections::{HashMap, HashSet};

pub fn first_puzzle() -> u32 {
    let grid = parse_input();

    let sets = find_sets(&grid);
    let y_size = grid.len() as i32;
    let x_size = grid[0].len() as i32;

    let mut antinodes: HashSet<(i32, i32)> = HashSet::new();
    for entry in sets {
        let list = entry.1;
        for i in 0..list.len() - 1 {
            for j in i + 1..list.len() {
                let a = list[i];
                let b = list[j];
                let distance = (b.0 - a.0, b.1 - a.1);
                let first = (a.0 - distance.0, a.1 - distance.1);
                let second = (b.0 + distance.0, b.1 + distance.1);
                if first.0 >= 0 && first.0 < x_size && first.1 >= 0 && first.1 < y_size {
                    antinodes.insert((first.0, first.1));
                }
                if second.0 >= 0 && second.0 < x_size && second.1 >= 0 && second.1 < y_size {
                    antinodes.insert((second.0, second.1));
                }
            }
        }
    }


    // Printouts
    // for line in &grid {
    //     for c in line {
    //         print!("{}", *c as char)
    //     }
    //     println!();
    // }
    // println!();
    // for x in &antinodes {
    //     grid[x.1 as usize][x.0 as usize] = b'#';
    // }
    // for line in grid {
    //     for c in line {
    //         print!("{}", c as char)
    //     }
    //     println!();
    // }

    antinodes.len() as u32
}

pub fn second_puzzle() -> u32 {
    let grid = parse_input();

    let sets = find_sets(&grid);
    let y_size = grid.len() as i32;
    let x_size = grid[0].len() as i32;

    let mut antinodes: HashSet<(i32, i32)> = HashSet::new();
    for entry in sets {
        let list = entry.1;
        for i in 0..list.len() - 1 {
            for j in i + 1..list.len() {
                let a = list[i];
                let b = list[j];
                let distance = (b.0 - a.0, b.1 - a.1);
                let mut start_pos = a;
                // Move to one end of the grid
                loop {
                    let pos = (start_pos.0 - distance.0, start_pos.1 - distance.1);
                    if pos.0 >= 0 && pos.0 < x_size && pos.1 >= 0 && pos.1 < y_size {
                        start_pos = pos;
                    } else {
                        break;
                    }
                }
                // Go through and add them all
                antinodes.insert((start_pos.0, start_pos.1));
                loop {
                    let pos = (start_pos.0 + distance.0, start_pos.1 + distance.1);
                    if pos.0 >= 0 && pos.0 < x_size && pos.1 >= 0 && pos.1 < y_size {
                        antinodes.insert((pos.0, pos.1));
                        start_pos = pos;
                    } else {
                        break;
                    }
                }
            }
        }
    }


    // Printouts
    // for line in &grid {
    //     for c in line {
    //         print!("{}", *c as char)
    //     }
    //     println!();
    // }
    // println!();
    // for x in &antinodes {
    //     grid[x.1 as usize][x.0 as usize] = b'#';
    // }
    // for line in grid {
    //     for c in line {
    //         print!("{}", c as char)
    //     }
    //     println!();
    // }

    antinodes.len() as u32
}

fn find_sets(grid: &Vec<Vec<u8>>) -> HashMap<u8, Vec<Position>> {
    let mut sets: HashMap<u8, Vec<Position>> = HashMap::new();

    for y in 0..grid.len() {
        for x in 0..grid[y].len() {
            if grid[y][x] != b'.' {
                let set = sets.entry(grid[y][x]).or_insert(vec![]);
                set.push((x as i32, y as i32));
            }
        }
    }

    sets
}

type Position = (i32, i32);

fn parse_input() -> Vec<Vec<u8>> {
    let file_content = file_handling::read_day_input(8);
    let grid: Vec<Vec<u8>> = file_content.lines().map(|l| l.bytes().collect()).collect();
    grid
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_first_puzzle() {
        assert_eq!(first_puzzle(), 392);
    }

    #[test]
    fn test_second_puzzle() {
        assert_eq!(second_puzzle(), 1235);
    }
}
