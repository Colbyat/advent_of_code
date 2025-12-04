use std::fs;

fn get_at(grid: &Vec<Vec<char>>, row: isize, col: isize) -> Option<char> {
    let unsized_row = usize::try_from(row).ok()?;
    let unsized_col = usize::try_from(col).ok()?;
    return grid.get(unsized_row)?.get(unsized_col).copied();
}

fn solve(char_array: &Vec<Vec<char>>, recursive: bool, mut total: usize) -> usize {
    let mut new_char_array: Vec<Vec<char>> = char_array.clone();
    let starting_total = total;

    for row in 0..char_array.len() {
        for col in 0..char_array[row].len() {
            if char_array[row][col] == '.' {
                continue;
            }

            let new_row = row as isize;
            let new_col = col as isize;

            let mut adjacent = 0;

            if get_at(&char_array, new_row - 1, new_col - 1) == Some('@') {
                adjacent += 1;
            }
            if get_at(&char_array, new_row - 1, new_col) == Some('@') {
                adjacent += 1;
            }
            if get_at(&char_array, new_row - 1, new_col + 1) == Some('@') {
                adjacent += 1;
            }

            if get_at(&char_array, new_row, new_col - 1) == Some('@') {
                adjacent += 1;
            }
            if get_at(&char_array, new_row, new_col + 1) == Some('@') {
                adjacent += 1;
            }

            if get_at(&char_array, new_row + 1, new_col - 1) == Some('@') {
                adjacent += 1;
            }
            if get_at(&char_array, new_row + 1, new_col) == Some('@') {
                adjacent += 1;
            }
            if get_at(&char_array, new_row + 1, new_col + 1) == Some('@') {
                adjacent += 1;
            }

            if adjacent < 4 {
                total += 1;
                new_char_array[row][col] = '.';
            }
        }
    }

    if recursive && total != starting_total {
        return solve(&new_char_array, recursive, total);
    } else {
        return total;
    }
}

pub fn print_solution() {
    let buf = fs::read_to_string("inputs/day_4.txt").unwrap();

    let char_array: Vec<Vec<char>> = buf
        .split("\n")
        .map(|row| row.chars().collect::<Vec<char>>())
        .collect();

    println!(
        "Day 4: Part 1: {}, Part 2: {}",
        solve(&char_array, false, 0),
        solve(&char_array, true, 0)
    );
}
