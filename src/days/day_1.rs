use std::fs;

fn solve(f: fn(i32, i32) -> i32) -> i32 {
    let buf = fs::read_to_string("inputs/day_1.txt").unwrap();

    let operations: Vec<&str> = buf.split("\n").collect();

    let mut total = 0;
    let mut dial = 50;

    for operation in operations {
        if operation.is_empty() {
            continue;
        }

        let first = operation.chars().next().unwrap();
        let rest = &operation[1..];

        let movement = match first {
            'R' => 1,
            'L' => -1,
            _ => continue,
        };

        let step: i32 = rest.parse().unwrap();

        let new_value = dial + movement * step;

        dial = new_value.rem_euclid(100);
        total += f(dial, new_value);
    }

    return total;
}

fn match_if_equals_0(dial: i32, _: i32) -> i32 {
    return match dial {
        0 => 1,
        _ => 0,
    }
}

fn get_remainder(_: i32, new_value: i32) -> i32 {
    return (new_value.div_euclid(100)).abs()
}

pub fn print_solution() {
    println!("Day 1: Part 1: {}, Part 2: {}", solve(match_if_equals_0), solve(get_remainder));
}

