use std::fs;

pub fn solve_part_1() {
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
        total += match dial {
            0 => 1,
            _ => 0,
        };
    }

    println!("{}", total);
}

pub fn solve_part_2() {
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
        total += (new_value.div_euclid(100)).abs();
    }

    println!("{}", total);
}
