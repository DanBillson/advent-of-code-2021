use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

fn main() {
    one();
    two();
}

fn get_measurements() -> Vec<i32> {
    // Read entries from file and convert to an i32 vector
    let path = Path::new("input.txt");
    let display = path.display();

    let file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", display, why),
        Ok(file) => file,
    };

    let reader = BufReader::new(file);

    let v: Vec<i32> = reader
        .lines()
        .map(|line| line.unwrap().parse::<i32>().unwrap())
        .collect();

    v
}

fn count_increases(v: Vec<i32>) -> i32 {
    let mut c = 0;

    for (i, x) in v.iter().enumerate() {
        if i > 0 && v[i - 1] < *x {
            c = c + 1;
        }
    }

    c
}

pub fn one() {
    let measurements = get_measurements();
    let count = count_increases(measurements);

    print!("{} ", count);
}

pub fn two() {
    let v = get_measurements();
    let sums: Vec<i32> = v
        .iter()
        .enumerate()
        .map(|(i, x)| {
            if i <= v.len() - 3 {
                return *x + v[i + 1] + v[i + 2];
            }
            return 0;
        })
        .collect();
    let count = count_increases(sums);

    print!("{} ", count);
}
