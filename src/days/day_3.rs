use std::fs;

fn solve(k: usize) -> usize {
    let buf = fs::read_to_string("inputs/day_3.txt").unwrap();

    let mut total: usize = 0;

    for bank in buf.split("\n") {
        total += solve_with_stack(bank, k);
    }

    return total;
}

fn solve_with_stack(bank: &str, k: usize) -> usize {
    let bank_as_digits: Vec<usize> = bank
            .chars()
            .filter_map(|c| Some((c as u8 - b'0') as usize))
            .collect();
    let mut deletions_left = bank.len() - k;
    let mut stack: Vec<usize> = Vec::with_capacity(k);

    for digit in bank_as_digits {
            while !stack.is_empty() && deletions_left > 0 && *stack.last().unwrap() < digit {
                stack.pop();
                deletions_left -= 1;
            }
            stack.push(digit);
        }

    stack.truncate(k);

    let mut val: usize = 0;
        for d in stack {
            val = val * 10 + d;
        }
    return val;
}

pub fn print_solution() {
    println!("Day 3: Part 1: {}, Part 2: {}", solve(2), solve(12));
}