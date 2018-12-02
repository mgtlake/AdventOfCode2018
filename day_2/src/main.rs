use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

extern crate itertools;

use itertools::Itertools;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let file = File::open(filename).expect("Could not read file");
    let lines: Vec<String> = BufReader::new(file)
        .lines()
        .map(|line| line.expect("Line could not be read"))
        .filter(|line| line.len() > 0)
        .collect();

    part_one(&lines);
    part_two(&lines);
}

fn part_one(lines: &Vec<String>) {
    let count_2 = lines.iter().filter(|line| n_repeats(line, 2)).count();
    let count_3 = lines.iter().filter(|line| n_repeats(line, 3)).count();

    println!("{}", count_2 * count_3);
}

fn n_repeats(str: &String, n: i32) -> bool {
    let mut freq: HashMap<char, i32> = HashMap::new();

    for c in str.chars() {
        let cur = *freq.entry(c).or_insert(0);
        freq.insert(c, cur + 1);
    }

    return freq.values().any(|count| *count == n);
}

fn part_two(lines: &Vec<String>) {
    for line in lines {
        for other in lines {
            if almost_equals(line, other) {
                println!(
                    "{}",
                    line.chars()
                        .zip(other.chars())
                        .filter(|(c1, c2)| c1 == c2)
                        .map(|(c, _)| c)
                        .format("")
                );
                return;
            }
        }
    }
}

fn almost_equals(str1: &String, str2: &String) -> bool {
    return str1
        .chars()
        .zip(str2.chars())
        .filter(|(c1, c2)| c1 != c2)
        .count()
        == 1;
}
