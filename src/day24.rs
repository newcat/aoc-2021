use crate::readfile::readfile;
use regex::Regex;
use std::collections::{HashSet, VecDeque};

type Registers = [HashSet<isize>; 4];

fn get_register_index(register: &str) -> usize {
    match register {
        "w" => 0,
        "x" => 1,
        "y" => 2,
        "z" => 3,
        _ => panic!("Invalid register: {}", register),
    }
}

fn perform_operation<F>(registers: &mut Registers, op1: &str, op2: &str, f: F)
where
    F: Fn(isize, isize) -> isize,
{
    let r1 = &registers[get_register_index(op1)];
    let mut temp_hs = HashSet::new();
    let r2: &HashSet<isize>;
    if ["w", "x", "y", "z"].contains(&op2) {
        r2 = &registers[get_register_index(op2)];
    } else {
        temp_hs.insert(op2.parse().unwrap());
        r2 = &temp_hs;
    };

    let mut result: HashSet<isize> = HashSet::new();
    for a in r1 {
        for b in r2 {
            result.insert(f(*a, *b));
        }
    }

    registers[get_register_index(op1)] = result;
}

fn run_program(lines: &readfile::Lines, inputs: Vec<HashSet<isize>>) -> Registers {
    let re = Regex::new("(\\w+) ([wxyz]|-?\\d+) ?([wxyz]|-?\\d+)?").unwrap();
    let mut registers: Registers = [
        HashSet::from_iter([0]),
        HashSet::from_iter([0]),
        HashSet::from_iter([0]),
        HashSet::from_iter([0]),
    ];
    let mut input_index = 0;

    for l in lines.lines() {
        let groups = re.captures(l).unwrap();
        if &groups[1] == "inp" {
            registers[get_register_index(&groups[2])] = inputs[input_index].clone();
            input_index += 1;
        } else {
            let f: fn(isize, isize) -> isize = match &groups[1] {
                "add" => |a, b| a + b,
                "mul" => |a, b| a * b,
                "div" => |a, b| a / b,
                "mod" => |a, b| a % b,
                "eql" => |a, b| {
                    if a == b {
                        1
                    } else {
                        0
                    }
                },
                _ => panic!("Invalid operation: {}", &groups[1]),
            };
            perform_operation(&mut registers, &groups[2], &groups[3], f);
        }
    }

    return registers;
}

fn part1(lines: &readfile::Lines) {
    let mut found_numbers: Vec<isize> = vec![];
    for x in 0..14 {
        for i in (0..=9).rev() {
            if i == 0 {
                panic!("Could not find value for {}", x);
            }
            let mut inputs: Vec<HashSet<isize>> = vec![];
            for n in &found_numbers {
                inputs.push(HashSet::from_iter([*n]));
            }
            inputs.push(HashSet::from_iter([i]));
            while inputs.len() < 14 {
                inputs.push(HashSet::from_iter(1..=9));
            }
            let result = run_program(lines, inputs);
            if result[3].contains(&0) {
                found_numbers.push(i);
                break;
            }
        }
    }
    println!(
        "{}",
        found_numbers
            .iter()
            .map(|v| v.to_string())
            .collect::<Vec<String>>()
            .join("")
    );
}

fn part2(lines: &readfile::Lines) {
    let mut found_numbers: VecDeque<isize> = VecDeque::new();
    for x in (0..14).rev() {
        for i in 1..=10 {
            if i == 10 {
                panic!("Could not find value for {}", x);
            }
            let mut inputs: Vec<HashSet<isize>> = vec![];
            while inputs.len() < 14 - 1 - found_numbers.len() {
                inputs.push(HashSet::from_iter(1..=9));
            }
            inputs.push(HashSet::from_iter([i]));
            for n in &found_numbers {
                inputs.push(HashSet::from_iter([*n]));
            }
            let result = run_program(lines, inputs);
            if result[3].contains(&0) {
                found_numbers.push_front(i);
                break;
            }
        }
    }
    println!(
        "{}",
        found_numbers
            .iter()
            .map(|v| v.to_string())
            .collect::<Vec<String>>()
            .join("")
    );
}

pub fn run() {
    let lines = readfile::Lines::new("day24.txt");
    part1(&lines);
    part2(&lines);
}
