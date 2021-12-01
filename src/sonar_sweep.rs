use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

// Day 1 - Part 1

pub fn run() {
    // Read entries from file and convert to an i32 vector
    let path = Path::new("sonar_readings.txt");
    let display = path.display();

    let file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", display, why),
        Ok(file) => file,
    };

    let reader = BufReader::new(file);
    let measurements: Vec<i32> = reader
        .lines()
        .map(|line| line.unwrap().parse::<i32>().unwrap())
        .collect();

    let mut count = 0;

    for (i, value) in measurements.iter().enumerate() {
        if i > 0 && measurements[i - 1] < *value {
            count = count + 1;
        }
    }

    print!("{} ", count);
}
