use std::fs;

pub fn solve_part_1() {
    let buf = fs::read_to_string("inputs/day_3.txt").unwrap();

    let mut total: usize = 0;

    for bank in buf.split("\n") {
        let mut largest: usize = 0;
        for i in 0..bank.len() {
            for j in i + 1..bank.len() {
                largest = std::cmp::max(
                    largest,
                    (bank.chars().nth(i).unwrap().to_string()
                        + &bank.chars().nth(j).unwrap().to_string())
                        .parse::<usize>()
                        .unwrap(),
                );
            }
        }
        total += largest as usize;
    }

    println!("{total}");
}

pub fn solve_part_2() {
    let buf = fs::read_to_string("inputs/day_3.txt").unwrap();

    let mut total: usize = 0;

    for bank in buf.split("\n") {
        let bank_as_digits: Vec<usize> = bank
            .chars()
            .filter_map(|c| Some((c as u8 - b'0') as usize))
            .collect();
        let mut deletions_left = bank.len() - 12;
        let mut stack: Vec<usize> = Vec::with_capacity(12);

        for digit in bank_as_digits {
            while !stack.is_empty() && deletions_left > 0 && *stack.last().unwrap() < digit {
                stack.pop();
                deletions_left -= 1;
            }
            stack.push(digit);
        }

        stack.truncate(12);

        let mut val: usize = 0;
        for d in stack {
            val = val * 10 + d;
        }
        total += val;
    }

    println!("{total}");
}
