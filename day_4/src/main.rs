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

    lines.iter().map(|line| line.trim());

    lines.sort();

    println!("{}", part_one(&lines));;
}

fn part_one(lines: &Vec<String>) -> usize {
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
        let month = cap_number(&date_time_caps, 1);
        let day = cap_number(&date_time_caps, 2);
        let hour = cap_number(&date_time_caps, 3);
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

    for (k, v) in sleeping.iter() {
        let peak = v
            .iter()
            .enumerate()
            .max_by_key(|(_, v)| *v)
            .map(|(i, _)| i)
            .unwrap();
        println!(
            "{} - sum {}, peak {}",
            k,
            //v.iter().fold(0, |sum, s| sum + if s > &0 { 1 } else { 0 }),
            v.iter().fold(0, |sum, s| sum + s),
            peak
        );
        println!(
            "{} {}",
            k,
            v.iter()
                .map(|c| match c {
                    0 => ".",
                    _ => "#",
                }).collect::<Vec<&str>>()
                .join("")
        )
    }

    return sleeping
        .iter()
        .max_by_key(|(_, v)| v.iter().fold(0, |sum, s| sum + s))
        .map(|(k, v)| {
            (
                k,
                v.iter()
                    .enumerate()
                    .max_by_key(|(_, v)| *v)
                    .map(|(i, _)| i)
                    .unwrap(),
            )
        }).map(|(k, v)| k * v)
        .unwrap();
}
