use std::fs;

#[derive(Debug)]
struct Node {
    x: usize,
    y: usize,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}

impl Node {
    fn new(x: usize, y: usize, left: Option<Node>, right: Option<Node>) -> Self {
        return Node {
            x,
            y,
            left: match left {
                Some(l) => Some(Box::new(l)),
                None => None,
            },
            right: match right {
                Some(r) => Some(Box::new(r)),
                None => None,
            },
        };
    }

    fn is_leaf(&self) -> bool {
        return self.left.is_none() && self.right.is_none();
    }

    fn size(&self) -> usize {
        let left = self.left.as_ref();
        let right = self.right.as_ref();
        match (left, right) {
            (Some(l), Some(r)) => {
                if l.is_leaf() && r.is_leaf() {
                    return 1;
                } else {
                    return l.size() + r.size() + 1;
                }
            }
            (Some(l), None) => {
                if l.is_leaf() {
                    return 1;
                } else {
                    return l.size() + 1;
                }
            }
            (None, Some(r)) => {
                if r.is_leaf() {
                    return 1;
                } else {
                    return r.size() + 1;
                }
            }
            (None, None) => 1,
        }
    }
}

fn transform_into_binary_tree(
    mirrors: Vec<(usize, usize)>,
    start_x: usize,
    start_y: usize,
) -> Node {
    let root = Node::new(start_x, start_y, None, None);
    return find_children(&mirrors, &root, true).unwrap();
}

fn find_children(mirrors: &Vec<(usize, usize)>, node: &Node, is_start: bool) -> Option<Node> {
    if is_start {
        let child_coordinates = mirrors
            .iter()
            .filter(|&&(x, y)| x == node.x && y > node.y)
            .min_by_key(|&&(_x, y)| y);

        match child_coordinates {
            Some((x, y)) => {
                let child = find_children(mirrors, &Node::new(*x, *y, None, None), false);
                return Some(Node::new(node.x, node.y, child, None));
            }
            None => return Some(Node::new(node.x, node.y, None, None)),
        }
    } else {
        let child_coordinates_left = mirrors
            .iter()
            .filter(|&&(x, y)| x == node.x - 1 && y > node.y)
            .min_by_key(|&&(_x, y)| y);
        let child_coordinates_right = mirrors
            .iter()
            .filter(|&&(x, y)| x == node.x + 1 && y > node.y)
            .min_by_key(|&&(_x, y)| y);

        /*println!(
            "({}, {}), left: {:#?}, right: {:#?}",
            node.x, node.y, child_coordinates_left, child_coordinates_right
        );*/

        let left_child = match child_coordinates_left {
            Some((x, y)) => find_children(mirrors, &Node::new(*x, *y, None, None), false),
            None => None,
        };

        let right_child = match child_coordinates_right {
            Some((x, y)) => find_children(mirrors, &Node::new(*x, *y, None, None), false),
            None => None,
        };

        return Some(Node::new(node.x, node.y, left_child, right_child));
    }
}

fn find_all_mirrors(char_array: Vec<Vec<char>>) -> (Vec<(usize, usize)>, usize, usize) {
    let mut positions = Vec::new();
    let mut start_x = 0;
    let mut start_y = 0;

    for x in 0..char_array.len() {
        for y in 0..char_array[0].len() {
            let character = char_array[x][y];
            if character == '^' {
                positions.push((y, x));
            } else if character == 'S' {
                start_x = y;
                start_y = x;
            }
        }
    }

    return (positions, start_x, start_y);
}

pub fn print_solution() {
    let buf = fs::read_to_string("inputs/day_7.txt").unwrap();

    let char_array: Vec<Vec<char>> = buf.lines().map(|line| line.chars().collect()).collect();
    let (mirrors, start_x, start_y) = find_all_mirrors(char_array);
    let binary_tree = transform_into_binary_tree(mirrors, start_x, start_y);

    println!("Day 7 bad: Part 1: {}, Part 2: {}", binary_tree.size(), 0);
}
