use crate::file_handling;
use std::str::Split;

pub fn first_puzzle() -> u64 {
    let equations = parse_input();
    println!("{:?}", equations);
    
    1
}

fn parse_input() -> Vec<Equation> {
    let file_content = file_handling::read_day_input(7);
    file_content
        .lines()
        .map(|line| {
            let mut split: Split<&str> = line.split(":");
            let result = split.next().unwrap().parse::<u64>().unwrap();
            let numbers: Vec<u64> = split
                .next()
                .unwrap()
                .split_whitespace()
                .map(|num| num.parse::<u64>().unwrap())
                .collect();
            Equation { result, numbers }
        })
        .collect()
}

#[derive(Debug)]
struct Equation {
    pub result: u64,
    pub numbers: Vec<u64>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_first_puzzle() {
        assert_eq!(first_puzzle(), 1);
    }
}
