use std::cmp::max;
use std::{fs, usize};

fn solve_part_1(
    products: impl AsRef<Vec<usize>>,
    ranges: impl AsRef<Vec<(usize, usize)>>,
) -> usize {
    let mut count = 0;

    for product in products.as_ref() {
        let mut is_fresh = false;

        for range in ranges.as_ref() {
            if (range.0..=range.1).contains(&product) {
                is_fresh = true;
            }
        }

        if is_fresh {
            count += 1;
        }
    }

    return count;
}

fn solve_part_2(ranges: impl AsRef<Vec<(usize, usize)>>) -> usize {
    let mut sorted = ranges.as_ref().clone();
    sorted.sort();

    let mut total: usize = 0;
    let mut start = sorted[0].0;
    let mut end = sorted[0].1;

    for &(s, e) in sorted.iter().skip(1) {
        if s <= end + 1 {
            end = max(end, e);
        } else {
            total += end - start + 1;
            start = s;
            end = e;
        }
    }

    total += end - start + 1;

    return total;
}

pub fn print_solution() {
    let buf = fs::read_to_string("inputs/day_5.txt").unwrap();
    let (ranges, products) = buf.split_once("\n\n").unwrap();

    let fresh_ranges: Vec<(usize, usize)> = ranges
        .lines()
        .map(|range| {
            let (lower, upper) = range.split_once("-").unwrap();
            return (
                lower.parse::<usize>().unwrap(),
                upper.parse::<usize>().unwrap(),
            );
        })
        .collect();

    let products: Vec<usize> = products
        .lines()
        .map(|num| return num.parse::<usize>().unwrap())
        .collect();

    println!(
        "Day 5: Part 1: {}, Part 2: {}",
        solve_part_1(&products, &fresh_ranges),
        solve_part_2(&fresh_ranges)
    );
}
