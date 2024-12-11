use crate::file_handling;

pub fn first_puzzle() -> u64 {
    let (mut blocks, _) = parse_input();

    defrag(&mut blocks);
    // println!("{:?}", blocks);

    calculate_checksum(&blocks)
}

fn defrag(blocks: &mut Vec<i32>) {
    let mut left: usize = 0;
    let mut right: usize = blocks.len() - 1;

    while left < right {
        if blocks[right] == -1 {
            right -= 1;
            continue;
        } else {
            while blocks[left] != -1 {
                left += 1;
            }
            let temp = blocks[left];
            blocks[left] = blocks[right];
            blocks[right] = temp;

            right -= 1;
            left += 1;
        }
    }
}

pub fn second_puzzle() -> u64 {
    let (mut blocks, last_id) = parse_input();

    defrag_whole_files(&mut blocks, last_id);

    // Print
    // for i in 0..blocks.len() {
    //     if blocks[i] == -1 {
    //         print!(".")
    //     } else {
    //         print!("{}", blocks[i].to_string())
    //     }
    // }
    // println!();

    calculate_checksum(&blocks)
}

fn defrag_whole_files(blocks: &mut Vec<i32>, mut last_id: i32) {
    while last_id >= 0 {
        let mut right_start: usize = 0;
        let mut right_len = 0;

        // Find start of file block
        for i in 0..blocks.len() {
            if blocks[i] == last_id {
                right_start = i;
                break;
            }
        }
        // Find end of file block and calc len
        for i in (0..blocks.len()).rev() {
            if blocks[i] == last_id {
                right_len = i - right_start + 1;
                break;
            }
        }

        // Start going through blocks searching for a space large enough to fit the file
        let mut left_len = 0;
        for left in 0..blocks.len() {
            if blocks[left] != -1 {
                left_len = 0;
                continue;
            }
            left_len += 1;
            if left_len >= right_len {
                // We went too far, only moving left
                if left >= right_start {
                    break;
                }
                // Copy file over
                for i in 0..right_len {
                    let temp = blocks[left - i];
                    blocks[left - i] = blocks[right_start + i];
                    blocks[right_start + i] = temp;
                }
                break;
            }
        }

        last_id -= 1;
    }
}

/*
The final step of this file-compacting process is to update the filesystem checksum.
To calculate the checksum, add up the result of multiplying each of these blocks' position with the
file ID number it contains. The leftmost block is in position 0. If a block contains free space,
skip it instead.

Continuing the first example, the first few blocks' position multiplied by its file ID number
are 0 * 0 = 0, 1 * 0 = 0, 2 * 9 = 18, 3 * 9 = 27, 4 * 8 = 32, and so on.
In this example, the checksum is the sum of these, 1928.
 */
fn calculate_checksum(blocks: &Vec<i32>) -> u64 {
    let mut sum: u64 = 0;
    for i in 0..blocks.len() {
        if blocks[i] == -1 {
            continue;
        }
        sum += blocks[i] as u64 * i as u64;
    }

    sum
}

fn parse_input() -> (Vec<i32>, i32) {
    let file_content = file_handling::read_day_input(9);

    let mut blocks: Vec<i32> = vec![];
    let mut file = true;
    let mut id = 0;
    for char in file_content.chars() {
        let spaces = char.to_digit(10).expect("Char was not a digit!!");
        // Inserting file
        if file {
            file = false;
            for _ in 0..spaces {
                blocks.push(id);
            }
            id += 1;
        }
        // Inserting free space
        else {
            file = true;
            for _ in 0..spaces {
                blocks.push(-1)
            }
        }
    }

    (blocks, id - 1)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_first_puzzle() {
        assert_eq!(first_puzzle(), 6288707484810);
    }

    #[test]
    fn test_second_puzzle() {
        assert_eq!(second_puzzle(), 6311837662089);
    }
}
