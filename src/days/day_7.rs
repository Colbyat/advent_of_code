use std::fs;

fn solve(
    grid: &Vec<Vec<char>>,
    start_row: usize,
    start_col: usize,
    count_duplicates: bool,
) -> usize {
    let mut splits = 0;
    let height = grid.len();
    let width = grid[0].len();

    let mut beams: Vec<usize> = vec![0; width];
    beams[start_col] = 1;

    for row_num in start_row + 1..height {
        let mut next: Vec<usize> = vec![0; width];

        for i in 0..width {
            if grid[row_num][i] == '^' && beams[i] > 0 {
                splits += 1;

                if i != 0 {
                    next[i - 1] += beams[i];
                }

                if i != width - 1 {
                    next[i + 1] += beams[i];
                }
            } else {
                next[i] += beams[i];
            }
        }

        beams = next;
    }

    return match count_duplicates {
        true => beams.iter().sum(),
        false => splits,
    };
}

fn find_start(char_array: &Vec<Vec<char>>) -> (usize, usize) {
    for row in 0..char_array.len() {
        for col in 0..char_array[0].len() {
            if char_array[row][col] == 'S' {
                return (row, col);
            }
        }
    }

    return (0, 0);
}

pub fn print_solution() {
    let buf = fs::read_to_string("inputs/day_7.txt").unwrap();

    let grid: Vec<Vec<char>> = buf.lines().map(|line| line.chars().collect()).collect();
    let (start_row, start_col) = find_start(&grid);

    println!(
        "Day 7: Part 1: {}, Part 2: {}",
        solve(&grid, start_row, start_col, false),
        solve(&grid, start_row, start_col, true)
    );
}
