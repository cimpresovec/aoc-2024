use std::fs;

fn main() {
    let input_content = fs::read_to_string("input/day_01.txt")
        .expect("File not found");

    for line in input_content.lines() {
        println!("{}", line);
    }
}
