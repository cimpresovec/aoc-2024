use crate::file_handling;

pub fn first_puzzle() -> u32 {
    let char_grid = parse_input();

    let mut finds = 0;
    for y in 0..char_grid.len() {
        for x in 0..char_grid[y].len() {
            if char_grid[y][x] == 'X' {
                finds += count_xmas(&char_grid, x as i32, y as i32)
            }
        }
    }

    finds
}

fn count_xmas(grid: &Vec<Vec<char>>, x: i32, y: i32) -> u32 {
    let mut counts = 0;
    for dir_x in -1..2 {
        for dir_y in -1..2 {
            if dir_x == 0 && dir_y == 0 {
                continue;
            }
            if find_word(grid, x, y, dir_x, dir_y, "XMAS", 1) {
                counts += 1;
            }
        }
    }

    counts
}

fn find_word(
    grid: &Vec<Vec<char>>,
    pos_x: i32,
    pos_y: i32,
    dir_x: i32,
    dir_y: i32,
    word: &str,
    char_index: usize,
) -> bool {
    // Ending
    if char_index >= word.len() {
        return true;
    }
    // Boundaries
    if pos_x + dir_x < 0 || pos_y + dir_y < 0 {
        return false;
    }
    if pos_x + dir_x >= grid[0].len() as i32 || pos_y + dir_y >= grid.len() as i32 {
        return false;
    }

    // Check next char
    if grid[(pos_y + dir_y) as usize][(pos_x + dir_x) as usize]
        != word.chars().nth(char_index).unwrap()
    {
        return false;
    }

    // Recurse find
    find_word(
        grid,
        pos_x + dir_x,
        pos_y + dir_y,
        dir_x,
        dir_y,
        word,
        char_index + 1,
    )
}

pub fn second_puzzle() -> u32 {
    let char_grid = parse_input();

    let mut finds = 0;
    for y in 0..char_grid.len() {
        for x in 0..char_grid[y].len() {
            if char_grid[y][x] == 'A' {
                if count_x_mas(&char_grid, x as i32, y as i32) {
                    finds += 1
                }
            }
        }
    }

    finds
}

fn count_x_mas(grid: &Vec<Vec<char>>, x: i32, y: i32) -> bool {
    if x <= 0 || y <= 0 || x + 1 >= grid[0].len() as i32 || y + 1 >= grid.len() as i32 {
        return false;
    }

    // Ugly, but it works
    if (grid[(y - 1) as usize][(x - 1) as usize] == 'M'
        && grid[(y + 1) as usize][(x + 1) as usize] == 'S')
        || (grid[(y - 1) as usize][(x - 1) as usize] == 'S'
            && grid[(y + 1) as usize][(x + 1) as usize] == 'M')
    {
        if (grid[(y - 1) as usize][(x + 1) as usize] == 'M'
            && grid[(y + 1) as usize][(x - 1) as usize] == 'S')
            || (grid[(y - 1) as usize][(x + 1) as usize] == 'S'
                && grid[(y + 1) as usize][(x - 1) as usize] == 'M')
        {
            return true;
        }
    }

    return false;
}

fn parse_input() -> Vec<Vec<char>> {
    let input = file_handling::read_day_input(4);
    input.lines().map(|line| line.chars().collect()).collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_first_puzzle() {
        assert_eq!(first_puzzle(), 2551);
    }

    #[test]
    fn test_second_puzzle() {
        assert_eq!(second_puzzle(), 1985);
    }
}
