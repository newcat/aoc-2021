use crate::readfile::readfile;
use lazy_static::lazy_static;
use regex::Regex;

enum Direction {
    Forward(u32),
    Down(u32),
    Up(u32),
}

fn parse_action(s: &str) -> Direction {
    lazy_static! {
        static ref RE: Regex = Regex::new("(\\w+) (\\d+)").unwrap();
    }
    let groups = RE.captures(s).unwrap();
    let value: u32 = groups[2].parse().unwrap();
    match &groups[1] {
        "forward" => Direction::Forward(value),
        "down" => Direction::Down(value),
        "up" => Direction::Up(value),
        _ => panic!("Invalid command {}", &groups[1])
    }
}

fn part1(lines: &readfile::Lines) {
    let mut x = 0;
    let mut y = 0;
    for line in lines.lines() {
        match parse_action(line) {
            Direction::Forward(v) => x += v,
            Direction::Down(v) => y += v,
            Direction::Up(v) => y -= v
        }
    }
    println!("Part 1: {}", x * y);
}

fn part2(lines: &readfile::Lines) {
    let mut x = 0;
    let mut y = 0;
    let mut aim = 0;
    for line in lines.lines() {
        match parse_action(line) {
            Direction::Forward(v) => {
                x += v;
                y += aim * v;
            },
            Direction::Down(v) => aim += v,
            Direction::Up(v) => aim -= v
        }
    }
    println!("Part 1: {}", x * y);
}

pub fn run() {
    let lines = readfile::Lines::new("day2.txt");
    part1(&lines);
    part2(&lines);
}
