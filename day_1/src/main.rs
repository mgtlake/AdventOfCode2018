use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    println!("{}", part_one(filename));
    println!("{}", part_two(filename));
}

fn part_one(filename: &String) -> i32 {
    let file = File::open(filename).expect("Could not read file");
    let reader = BufReader::new(file);

    return reader.lines().fold(0, |sum, line| {
        sum + line
            .expect("line could not be read")
            .parse::<i32>()
            .expect("line was not an integer")
    });
}

fn part_two(filename: &String) -> i32 {
    let file = File::open(filename).expect("Could not read file");
    let lines: Vec<String> = BufReader::new(file)
        .lines()
        .map(|line| line.expect("Line could not be read"))
        .collect();

    let mut sum_freqs = HashMap::new();
    let mut sum = 0;
    loop {
        for line in &lines {
            sum += line.parse::<i32>().expect("line was not an integer");

            if !sum_freqs.contains_key(&sum) {
                sum_freqs.insert(sum, 1);
            } else {
                return sum;
            }
        }
    }
}
