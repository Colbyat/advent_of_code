use std::{fs::File, io::Read, path::Path};

fn main() {
    let path = Path::new("input.txt");
    let mut file = File::open(path).unwrap();
    let mut buf = String::new();
    file.read_to_string(&mut buf).unwrap();

    let operations: Vec<&str> = buf.split("\n").collect();

    let mut total = 0;
    let mut dial = 50;

    for operation in operations {
        if operation.is_empty() { continue; }

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
