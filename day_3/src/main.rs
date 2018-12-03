use std::collections::HashMap;
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

    let input = lines
        .iter()
        .map(|line| {
            let line = general_split(line, "@");
            let i = general_split(line[0], "#")[1]
                .parse::<usize>()
                .expect("Not a number");

            let relevent = general_split(line[1], ":");

            let (x, y) = pair_split(relevent[0], ",");
            let (w, h) = pair_split(relevent[1], "x");

            (i, (x, y), (w, h))
        }).collect();

    println!("{}", part_one(&input));
    println!("{}", part_two(&input));
}

fn part_one(lines: &Vec<(usize, (usize, usize), (usize, usize))>) -> usize {
    let mut cloth = [[0; 1000]; 1000];

    for (_, (x, y), (w, h)) in lines {
        for col in (x + 1)..=(x + w) {
            for row in (y + 1)..=(y + h) {
                cloth[row][col] += 1;
            }
        }
    }

    return cloth.iter().fold(0, |sum, row| {
        sum + row.iter().filter(|val| *val > &1).count()
    });
}

fn general_split<'a>(string: &'a str, pivot: &str) -> Vec<&'a str> {
    return string.split(pivot).map(|s| s.trim()).collect::<Vec<&str>>();
}

fn pair_split<'a>(string: &'a str, pivot: &str) -> (usize, usize) {
    let ints = string
        .split(pivot)
        .map(|s| s.trim())
        .map(|s| s.parse::<usize>().expect("Not a number"))
        .collect::<Vec<usize>>();
    return (ints[0], ints[1]);
}

fn part_two(lines: &Vec<(usize, (usize, usize), (usize, usize))>) -> usize {
    let mut cloth = vec![vec![None; 1000]; 1000];
    let mut valid = HashMap::new();

    for (i, (x, y), (w, h)) in lines {
        valid.insert(i, true);
        for col in (x + 1)..=(x + w) {
            for row in (y + 1)..=(y + h) {
                match cloth[row][col] {
                    Some(o) => {
                        valid.insert(o, false);
                        valid.insert(i, false);
                    }
                    None => cloth[row][col] = Some(i),
                }
            }
        }
    }

    return valid
        .iter()
        .filter(|(_, b)| **b)
        .map(|(i, _)| **i)
        .collect::<Vec<usize>>()[0];
}
