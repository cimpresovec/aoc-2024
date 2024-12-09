use crate::file_handling;
use itertools::{repeat_n, Itertools};
use std::str::Split;

pub fn first_puzzle() -> u64 {
    let equations = parse_input();
    let mut sum_of_valid: u64 = 0;

    // println!("{:?}", equations);
    let operators = vec!["+", "*"];
    for equation in equations {
        let perms =
            repeat_n(operators.iter(), equation.numbers.len() - 1).multi_cartesian_product();

        let mut calculates = false;
        for perm in perms {
            // println!("{:?}", perm);
            if result_calculates(&equation, &perm) {
                // println!("{} = {:?} {:?}", equation.result, equation.numbers, perm);
                calculates = true;
                break;
            }
        }

        if calculates {
            sum_of_valid += equation.result
        }
    }

    sum_of_valid
}

pub fn second_puzzle() -> u64 {
    let equations = parse_input();
    let mut sum_of_valid: u64 = 0;

    // println!("{:?}", equations);
    let operators = vec!["+", "*", "||"];
    for equation in equations {
        let perms =
            repeat_n(operators.iter(), equation.numbers.len() - 1).multi_cartesian_product();

        let mut calculates = false;
        for perm in perms {
            if result_calculates(&equation, &perm) {
                calculates = true;
                break;
            }
        }

        if calculates {
            sum_of_valid += equation.result
        }
    }

    sum_of_valid   
}

fn result_calculates(equation: &Equation, operators: &Vec<&&str>) -> bool {
    let mut calculation: u64 = equation.numbers[0];

    for i in 1..equation.numbers.len() {
        if *operators[i-1] == "*" {
            calculation *= equation.numbers[i];
        } else if *operators[i-1] == "+" {
            calculation += equation.numbers[i];
        } else {
            let concat = format!("{}{}", calculation, equation.numbers[i].to_string());
            calculation = concat.parse::<u64>().unwrap();
        }
        if calculation > equation.result {
            return false
        }
    }

    calculation == equation.result
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
        assert_eq!(first_puzzle(), 3119088655389);
    }

    #[test]
    fn test_second_puzzle() {
        assert_eq!(second_puzzle(), 264184041398847);
    }
}
