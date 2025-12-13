use std::{collections::HashSet, fs, i64, usize};

pub fn print_solution() {
    let buf = fs::read_to_string("inputs/day_8.txt").unwrap();
    let boxes = buf
        .lines()
        .map(|line| {
            let nums: Vec<i64> = line.split(",").map(|s| s.parse::<i64>().unwrap()).collect();
            return (nums[0], nums[1], nums[2]);
        })
        .collect();

    println!("Day 8: Part 1: {}, Part 2: {}", solve(&boxes, 10), 0);
}

fn solve(boxes: &Vec<(i64, i64, i64)>, num_iterations: usize) -> usize {
    let mut distances: Vec<(usize, usize, f64)> = Vec::new();

    for i in 0..boxes.len() {
        for j in i..boxes.len() {
            if i == j {
                continue;
            }

            distances.push((i, j, find_distance(&boxes[i], &boxes[j])));
        }
    }

    distances.sort_by(|a, b| b.2.partial_cmp(&a.2).unwrap());
    let mut groups: Vec<HashSet<usize>> = Vec::new();

    for _n in 1..=num_iterations {
        let (i, j, _distance) = distances.pop().unwrap();

        let (are_in_a_set, set_number) = are_in_a_set(i, j, &groups);

        if are_in_a_set {
            let mut new_set = groups[set_number].clone();
            new_set.insert(i);
            new_set.insert(j);
            groups[set_number] = new_set;
            println!(
                "Adding pair {:?}-{:?} to group {}, group now: {:?}",
                boxes[i],
                boxes[j],
                set_number,
                groups[set_number]
                    .iter()
                    .map(|i| boxes[*i])
                    .collect::<Vec<(i64, i64, i64)>>()
            );
        } else {
            groups.push([i, j].into());
            println!("Adding pair {:?}-{:?} to new group", boxes[i], boxes[j]);
        }
    }
    println!("{:?}", groups);

    let mut sizes: Vec<usize> = groups.iter().map(|set| set.len()).collect();
    sizes.sort_by(|a, b| b.cmp(a));
    sizes.truncate(3);
    return sizes.iter().product();
}

fn find_distance(box_1: &(i64, i64, i64), box_2: &(i64, i64, i64)) -> f64 {
    let distance_squared = i64::pow(box_1.0 - box_2.0, 2)
        + i64::pow(box_1.1 - box_2.1, 2)
        + i64::pow(box_1.2 - box_2.2, 2);

    return (distance_squared as f64).sqrt();
}

fn are_in_a_set(i: usize, j: usize, sets: &Vec<HashSet<usize>>) -> (bool, usize) {
    for set_num in 0..sets.len() {
        if sets[set_num].contains(&i) || sets[set_num].contains(&j) {
            return (true, set_num);
        }
    }

    return (false, 0);
}
