use crate::file_handling;
use std::collections::HashMap;

pub fn first_puzzle() -> i32 {
    let (mut left_numbers, mut right_numbers) = parse_input();

    left_numbers.sort();
    right_numbers.sort();

    let mut distance: i32 = 0;

    for i in 0..left_numbers.len() {
        distance += (left_numbers[i] - right_numbers[i]).abs();
    }

    distance
}

pub fn second_puzzle() -> i32 {
    let (left_numbers, right_numbers) = parse_input();

    // Create a map of right numbers
    let mut right_map_count: HashMap<i32, i32> = HashMap::new();
    for x in right_numbers {
        *right_map_count.entry(x).or_insert(0) += 1;
    }

    let mut similarity_score: i32 = 0;

    for x in left_numbers {
        let find = right_map_count.get(&x);
        match find {
            None => {}
            Some(count) => similarity_score += count * &x,
        }
    }

    similarity_score
}

fn parse_input() -> (Vec<i32>, Vec<i32>) {
    let input_content = file_handling::read_day_input(1);

    let mut left_numbers: Vec<i32> = vec![];
    let mut right_numbers: Vec<i32> = vec![];

    for line in input_content.lines() {
        let mut split = line.split_whitespace();
        left_numbers.push(split.next().unwrap().parse().unwrap());
        right_numbers.push(split.next().unwrap().parse().unwrap());
    }

    (left_numbers, right_numbers)
}

#[cfg(test)]
mod tests {
    use crate::day_01::{first_puzzle, second_puzzle};

    #[test]
    fn test_first_puzzle() {
        assert_eq!(first_puzzle(), 1110981);
    }

    #[test]
    fn test_second_puzzle() {
        assert_eq!(second_puzzle(), 24869388);
    }
}
