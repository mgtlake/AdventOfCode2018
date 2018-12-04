extern crate regex;

use regex::Regex;
use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let file = File::open(filename).expect("Could not read file");
    let mut lines = BufReader::new(file)
        .lines()
        .map(|line| line.expect("Line could not be read"))
        .filter(|line| line.len() > 0)
        .collect::<Vec<String>>();
    lines.sort();

    part_one_and_two(&lines);
}

fn part_one_and_two(lines: &Vec<String>) {
    let mut sleeping = HashMap::new();

    let date_time = Regex::new(r"\-(\d{2})\-(\d{2}) (\d{2}):(\d{2})").unwrap();
    let begin = Regex::new(r"Guard #(\d*)").unwrap();
    let sleep = Regex::new(r"falls asleep").unwrap();
    let wake = Regex::new(r"wakes up").unwrap();

    let cap_number =
        |ref cap: &regex::Captures, i| cap.get(i).unwrap().as_str().parse::<usize>().unwrap();

    let mut guard = None;
    let mut sleep_time = None;
    for line in lines {
        let date_time_caps = date_time
            .captures(line)
            .expect("Line should contain a date");
        let minute = cap_number(&date_time_caps, 4);

        if begin.is_match(line) {
            guard = Some(cap_number(&(begin.captures(line).unwrap()), 1));
        }
        if sleep.is_match(line) {
            sleep_time = Some(minute);
        }
        if wake.is_match(line) {
            let array = sleeping.entry(guard.unwrap()).or_insert(vec![0; 60]);
            for i in sleep_time.unwrap()..minute {
                array[i] += 1;
            }
        }
    }

    println!(
        "{}",
        sleeping
            .iter()
            // Get guard who spent largest total time asleep
            .max_by_key(|(_, v)| v.iter().fold(0, |sum, s| sum + s))
            // Get the time they were most often asleep
            .map(|(k, v)| (
                k,
                v.iter().position(|r| r == v.iter().max().unwrap()).unwrap(),
            )).map(|(k, v)| k * v)
            .unwrap()
    );

    println!(
        "{}",
        sleeping
            .iter()
            // Get guard who slept most on the same minute across days
            .max_by_key(|(_, v)| v.iter().max())
            // Get that minute
            .map(|(k, v)| (
                k,
                v.iter().position(|r| r == v.iter().max().unwrap()).unwrap()
            )).map(|(k, v)| k * v)
            .unwrap()
    );
}
