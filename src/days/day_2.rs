use std::fs;

fn solve(f: fn(&str) -> bool) -> u64 {
    let buf = fs::read_to_string("inputs/day_2.txt").unwrap();

    let mut num_invalid: u64 = 0;

    for range in buf.split(",") {
        let bounds: Vec<&str> = range.split("-").collect();

        for i in bounds[0].parse::<u64>().unwrap()..=bounds[1].parse::<u64>().unwrap() {
            let i_str = i.to_string();
            num_invalid += match f(&i_str) {
                true => i,
                false => 0,
            }
        }
    }

    return num_invalid;
}

fn is_invalid_for_part_1(i_str: &str) -> bool {
    if i_str.len() % 2 != 0 {
        return false;
    }

    let (first_half, second_half) = i_str.split_at(i_str.len() / 2);

    return first_half == second_half;
}

fn is_invalid_for_part_2(i_str: &str) -> bool {
    return (i_str.to_string() + i_str)[1..(2 * i_str.len() - 1)].contains(i_str);
}

pub fn print_solution() {
    println!("Day 2: Part 1: {}, Part 2: {}", solve(is_invalid_for_part_1), solve(is_invalid_for_part_2));
}