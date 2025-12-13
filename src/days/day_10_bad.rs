use regex::Regex;
use std::collections::{HashSet, VecDeque};
use std::fs;

pub fn print_solution() {
    let buf = fs::read_to_string("inputs/day_10.txt").unwrap();
    println!(
        "Day 10: Part 1: {}, Part 2: {}",
        solve(&buf, false),
        solve(&buf, true)
    );
}

fn parse_line(
    line: &str,
    bracket_re: &Regex,
    paren_re: &Regex,
    brace_re: &Regex,
) -> (usize, u128, Vec<u128>, Vec<u8>) {
    // Target lights ('.' off, '#' on)
    let b_caps = bracket_re.captures(line).expect("missing [ ... ]");
    let target_str = b_caps.get(1).unwrap().as_str();
    let n = target_str.chars().count();
    assert!(n <= 128, "Use a larger bitset if lights > 128");

    let mut target_mask: u128 = 0;
    for (i, ch) in target_str.chars().enumerate() {
        if ch == '#' {
            target_mask |= 1u128 << i;
        }
    }

    // Buttons → bitmasks (dedupe indices within each button, drop no-op buttons)
    let mut button_masks: Vec<u128> = Vec::new();
    for caps in paren_re.captures_iter(line) {
        let inside = caps.get(1).unwrap().as_str();
        let mut idxs: Vec<usize> = inside
            .split(',')
            .filter_map(|s| s.trim().parse::<usize>().ok())
            .collect();
        idxs.sort_unstable();
        idxs.dedup();

        let mut mask: u128 = 0;
        for idx in idxs {
            assert!(idx < n, "button refers to light index out of range");
            mask ^= 1u128 << idx; // XOR sets this bit
        }
        if mask != 0 {
            button_masks.push(mask);
        }
    }

    // Voltages (Part 2 input), parsed as u8 to save memory
    let mut voltages: Vec<u8> = Vec::new();
    if let Some(caps) = brace_re.captures(line) {
        voltages = caps
            .get(1)
            .unwrap()
            .as_str()
            .split(',')
            .filter_map(|s| s.trim().parse::<u16>().ok()) // allow bigger in text
            .map(|v| v.min(u8::MAX as u16) as u8)
            .collect();
        assert!(
            voltages.len() == n,
            "brace vector length must match number of lights"
        );
    }

    (n, target_mask, button_masks, voltages)
}

fn min_presses_part1(target: u128, button_masks: &[u128]) -> Option<usize> {
    let start: u128 = 0;
    if target == start {
        return Some(0);
    }

    let mut visited: HashSet<u128> = HashSet::new();
    let mut q: VecDeque<(u128, usize)> = VecDeque::new();
    visited.insert(start);
    q.push_back((start, 0));

    while let Some((state, dist)) = q.pop_front() {
        for &b in button_masks {
            let next = state ^ b;
            if visited.insert(next) {
                if next == target {
                    return Some(dist + 1);
                }
                q.push_back((next, dist + 1));
            }
        }
    }
    None
}

fn min_presses_part2(start_volts: Vec<u8>, buttons_idx: &[Vec<usize>]) -> Option<usize> {
    if start_volts.iter().all(|&v| v == 0) {
        return Some(0);
    }

    let mut visited: HashSet<Vec<u8>> = HashSet::new();
    let mut q: VecDeque<(Vec<u8>, usize)> = VecDeque::new();
    visited.insert(start_volts.clone());
    q.push_back((start_volts, 0));

    while let Some((state, dist)) = q.pop_front() {
        for btn in buttons_idx {
            // If any affected index is already 0 → pressing would underflow; skip
            if btn.iter().any(|&i| state[i] == 0) {
                continue;
            }
            // Apply press
            let mut next = state.clone();
            for &i in btn {
                next[i] = next[i].saturating_sub(1);
            }
            if visited.insert(next.clone()) {
                if next.iter().all(|&v| v == 0) {
                    return Some(dist + 1);
                }
                q.push_back((next, dist + 1));
            }
        }
    }
    None
}

pub fn solve(input: &str, for_part2: bool) -> usize {
    let bracket_re = Regex::new(r"\[([#.]+)\]").unwrap();
    let paren_re = Regex::new(r"\(([^)]*)\)").unwrap();
    let brace_re = Regex::new(r"\{([^}]+)\}").unwrap();

    input
        .lines()
        .filter(|l| !l.trim().is_empty())
        .map(|line| {
            let (_n, target, button_masks, volts) =
                parse_line(line, &bracket_re, &paren_re, &brace_re);

            if !for_part2 {
                min_presses_part1(target, &button_masks).unwrap();
            } else {
                let mut buttons_idx: Vec<Vec<usize>> = Vec::new();
                for caps in paren_re.captures_iter(line) {
                    let mut idxs: Vec<usize> = caps
                        .get(1)
                        .unwrap()
                        .as_str()
                        .split(',')
                        .filter_map(|s| s.trim().parse::<usize>().ok())
                        .collect();
                    idxs.sort_unstable();
                    idxs.dedup();
                    if !idxs.is_empty() {
                        buttons_idx.push(idxs);
                    }
                }
                min_presses_part2(volts, &buttons_idx).expect("no way to zero voltages")
            }
        })
        .sum()
}
