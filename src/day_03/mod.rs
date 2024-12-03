use crate::day_03::Operation::{Do, Dont, Mod};
use crate::file_handling;
use regex::{Captures, Regex};

pub fn first_puzzle() -> u32 {
    let file_content = file_handling::read_day_input(3);
    let regex = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();

    let operations: Vec<_> = regex
        .captures_iter(&file_content)
        .map(|capture| {
            (
                capture[1].parse::<u32>().unwrap(),
                capture[2].parse::<u32>().unwrap(),
            )
        })
        .collect();

    operations.iter().map(|op| op.0 * op.1).sum()
}

pub fn second_puzzle() -> u32 {
    let file_content = file_handling::read_day_input(3);
    let regex = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)|do\(\)|don't\(\)").unwrap();

    let operations: Vec<Operation> = regex
        .captures_iter(&file_content)
        .map(|capture| parse_capture(capture))
        .collect();

    let mut operations_enabled = true;
    let mut sum = 0;
    for op in operations {
        match op {
            Do => operations_enabled = true,
            Dont => operations_enabled = false,
            Mod(a, b) => {
                if operations_enabled {
                    sum += a * b;
                }
            }
        }
    }

    sum
}

fn parse_capture(capture: Captures) -> Operation {
    match capture.get(0).unwrap().as_str() {
        "do()" => Do,
        "don't()" => Dont,
        _ => Mod(
            capture.get(1).unwrap().as_str().parse::<u32>().unwrap(),
            capture.get(2).unwrap().as_str().parse::<u32>().unwrap(),
        ),
    }
}

enum Operation {
    Mod(u32, u32),
    Do,
    Dont,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_first_puzzle() {
        assert_eq!(first_puzzle(), 184576302);
    }

    #[test]
    fn test_second_puzzle() {
        assert_eq!(second_puzzle(), 118173507);
    }
}
