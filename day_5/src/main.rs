use std::env;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let file = File::open(filename).expect("Could not read file");
    let lines: Vec<String> = BufReader::new(file)
        .lines()
        .map(|line| line.expect("Line could not be read"))
        .filter(|line| line.len() > 0)
        .collect();

    let input = lines[0].trim();

    println!("{}", part_one(input));
    println!("{}", part_two(input));
}

fn part_one(input: &str) -> usize {
    return react_reduce(input).len();
}

fn part_two(input: &str) -> usize {
    let mut elems = input
        .chars()
        .map(|e| e.to_lowercase().collect::<Vec<_>>()[0])
        .collect::<Vec<char>>();
    elems.sort();
    elems.dedup();

    return elems
        .iter()
        .map(|e| {
            react_reduce(
                &input
                    .chars()
                    .filter(|c| c.to_lowercase().collect::<Vec<_>>()[0] != *e)
                    .collect::<String>(),
            ).len()
        }).min()
        .unwrap();
}

fn react_reduce(input: &str) -> String {
    let mut input = input.chars().collect::<Vec<char>>();

    let react = |c1: char, c2: char| {
        (c1 != c2) && (c1.to_lowercase().to_string() == c2.to_lowercase().to_string())
    };

    loop {
        let mut new = input.clone();

        let mut i = 0;
        while i < new.len() - 1 {
            if react(new[i], new[i + 1]) {
                new.remove(i + 1);
                new.remove(i);
            }
            i += 1;
        }

        if input == new {
            break;
        } else {
            input = new;
        }
    }

    return input.iter().collect::<String>();
}
