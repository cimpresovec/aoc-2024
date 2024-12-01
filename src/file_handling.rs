use std::fs;

pub fn read_day_input(day_number: i8) -> String {
    let file_name = format!("input/day_{:02}.txt", day_number);
    let input_content = fs::read_to_string(file_name)
        .expect("File not found");
    input_content
}