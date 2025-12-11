use std::fs;

pub fn solve_part_1(
    nums: impl AsRef<Vec<Vec<usize>>>,
    operations: Vec<&str>,
) -> usize {
    let nums = nums.as_ref();

    let mut total = 0;

    for i in 0..operations.len() {
        let mut row_nums: Vec<usize> = Vec::new();
        for j in 0..nums.len() {
            row_nums.push(nums[j][i]);
        }
        total += calculate_row_total(row_nums, operations[i]);
    }

    return total;
}

fn solve_part_2(char_array: impl AsRef<Vec<Vec<char>>>) -> usize {
    let char_array = char_array.as_ref();
    let mut total: usize = 0;
    let mut stack: Vec<usize> = Vec::new();
    let num_digits = char_array.len();

    for col in (0..char_array[0].len()).rev() {
        let mut col_total: String = String::new();
        for row in 0..num_digits {
            let character = char_array[row][col];
            if character == '+' || character == '*' {
                if !col_total.is_empty() {
                    stack.push(col_total.parse::<usize>().unwrap());
                }
                total += calculate_row_total(&stack, &*character.to_string()) as usize;
                stack.clear();
                col_total = String::new();
            }

            if let Some(_n) = character.to_digit(10) {
                col_total += &character.to_string();
            }
        }
        if !col_total.is_empty() {
            stack.push(col_total.parse::<usize>().unwrap());
        }
    }

    return total;
}

fn calculate_row_total(nums: impl AsRef<Vec<usize>>, operation: &str) -> usize {
    let nums = nums.as_ref();

    let mut row_total = nums[0];

    for n in nums.iter().skip(1) {
        if operation == "+" {
            row_total += n;
        } else if operation == "*" {
            row_total *= n;
        }
    }

    return row_total;
}

pub fn print_solution() {
    let buf = fs::read_to_string("inputs/day_6.txt").unwrap();
    let mut lines: Vec<&str> = buf.lines().collect();

    let char_array: Vec<Vec<char>> = lines
        .clone()
        .into_iter()
        .map(|line| line.chars().collect())
        .collect();

    let operations: Vec<&str> = lines.pop().unwrap().split_whitespace().collect();
    let nums: Vec<Vec<usize>> = lines
        .clone()
        .into_iter()
        .map(|line| {
            line.split_whitespace()
                .map(|entry| return entry.parse::<usize>().unwrap())
                .collect()
        })
        .collect();

    println!(
        "Day 6: Part 1: {}, Part 2: {}",
        solve_part_1(&nums, operations),
        solve_part_2(&char_array)
    );
}
