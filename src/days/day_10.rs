use regex::Regex;
use std::{collections::VecDeque, fs};

/// Structs
#[derive(Debug, Clone)]
struct Button {
    lights_affected: Vec<usize>,
}

impl Button {
    fn new(lights_affected: Vec<usize>) -> Self {
        return Button { lights_affected };
    }

    fn press(&self, lights: &Vec<bool>) -> Vec<bool> {
        let mut lights = lights.clone();
        for light in &self.lights_affected {
            lights[*light] = !lights[*light];
        }
        return lights;
    }

    fn would_underflow(&self, voltages: &Vec<usize>) -> bool {
        for voltage in &self.lights_affected {
            if voltages[*voltage] == 0 {
                return true;
            }
        }
        return false;
    }

    fn press_voltage(&self, voltages: &Vec<usize>) -> Vec<usize> {
        let mut voltages = voltages.clone();
        for voltage in &self.lights_affected {
            voltages[*voltage] -= 1;
        }
        return voltages;
    }
}

#[derive(Debug)]
struct Node {
    lights: Vec<bool>,
    voltages: Vec<usize>,
    pressed_buttons: Vec<usize>,
}

impl Node {
    fn new(lights: Vec<bool>, voltages: Vec<usize>, pressed_buttons: Vec<usize>) -> Self {
        return Node {
            lights,
            voltages,
            pressed_buttons,
        };
    }
}

/// Functions
pub fn print_solution() {
    let buf = fs::read_to_string("inputs/day_10.txt").unwrap();
    let lines = buf.lines().collect();

    println!(
        "Day 10: Part 1: {}, Part 2: {}",
        solve(&lines, false),
        solve(&lines, true)
    );
}

fn solve(lines: &Vec<&str>, is_part_2: bool) -> usize {
    let mut total = 0;
    let bracket_regex = Regex::new(r"\[([#.]+)\]").unwrap();
    let parentheses_regex = Regex::new(r"\(([^)]*)\)").unwrap();
    let brace_regex = Regex::new(r"\{([^}]+)\}").unwrap();

    for line in lines {
        let (lights, buttons, voltages) =
            sanitize_line(line, &bracket_regex, &parentheses_regex, &brace_regex);
        if !is_part_2 {
            total += traverse_tree_part_1(Node::new(lights, Vec::new(), Vec::new()), buttons);
        } else {
            total += traverse_tree_part_2(Node::new(lights, voltages, Vec::new()), buttons);
        }
    }

    return total;
}

fn traverse_tree_part_2(root: Node, buttons: Vec<Button>) -> usize {
    let mut queue: VecDeque<(Node, usize)> = VecDeque::new();
    queue.push_back((root, 0));

    while !queue.is_empty() {
        let (node, mut depth) = queue.pop_front().unwrap();
        depth += 1;

        for button in &buttons {
            if button.would_underflow(&node.voltages) {
                continue;
            }

            let pressed = button.press_voltage(&node.voltages);

            if pressed.iter().all(|f| *f == 0) {
                return depth;
            } else {
                queue.push_back((Node::new(node.lights.clone(), pressed, Vec::new()), depth));
            }
        }
    }

    return 0;
}

fn traverse_tree_part_1(root: Node, buttons: Vec<Button>) -> usize {
    let mut queue: VecDeque<(Node, usize)> = VecDeque::new();
    queue.push_back((root, 0));

    while !queue.is_empty() {
        let (node, mut depth) = queue.pop_front().unwrap();
        depth += 1;

        if &node.pressed_buttons.len() == &buttons.len() {
            continue;
        }

        for i in 0..buttons.len() {
            if node.pressed_buttons.contains(&i) {
                continue;
            }

            let button = &buttons[i];
            let after_pressed = button.press(&node.lights);
            let mut pressed_buttons = node.pressed_buttons.clone();
            pressed_buttons.push(i);
            if after_pressed.iter().all(|f| !f) {
                return depth;
            } else {
                queue.push_back((Node::new(after_pressed, Vec::new(), pressed_buttons), depth));
            }
        }
    }

    return 0;
}

fn sanitize_line(
    line: &str,
    bracket_regex: &Regex,
    parentheses_regex: &Regex,
    brace_regex: &Regex,
) -> (Vec<bool>, Vec<Button>, Vec<usize>) {
    let lights: Vec<bool> = bracket_regex
        .captures_iter(line)
        .flat_map(|capture| {
            capture
                .get(1)
                .unwrap()
                .as_str()
                .chars()
                .map(|char| match char {
                    '#' => true,
                    _ => false,
                })
        })
        .collect();
    let buttons: Vec<Button> = parentheses_regex
        .captures_iter(line)
        .map(|capture| {
            capture
                .get(1)
                .unwrap()
                .as_str()
                .split(',')
                .map(|char| char.parse::<usize>().unwrap())
                .collect()
        })
        .map(|lights| Button::new(lights))
        .collect();

    let voltages: Vec<usize> = brace_regex
        .captures_iter(line)
        .flat_map(|capture| {
            capture
                .get(1)
                .unwrap()
                .as_str()
                .split(',')
                .map(|char| char.parse::<usize>().unwrap())
        })
        .collect();

    return (lights, buttons, voltages);
}
