use crate::readfile;
use std::collections::HashMap;

/*
  ttt
u     v
u     v
u     v
  www
x     y
x     y
x     y
  zzz
*/

//                  tuvwxyz
const SEG0: u8 = 0b01110111;
const SEG1: u8 = 0b00010010;
const SEG2: u8 = 0b01011101;
const SEG3: u8 = 0b01011011;
const SEG4: u8 = 0b00111010;
const SEG5: u8 = 0b01101011;
const SEG6: u8 = 0b01101111;
const SEG7: u8 = 0b01010010;
const SEG8: u8 = 0b01111111;
const SEG9: u8 = 0b01111011;
const SEGS: [u8; 10] = [SEG0, SEG1, SEG2, SEG3, SEG4, SEG5, SEG6, SEG7, SEG8, SEG9];

macro_rules! bit {
    ($index:expr) => {
        (1 << (6 - $index))
    };
}

fn part1(lines: &readfile::Lines) {
    let count = lines
        .lines()
        .map(|l| l.split(" | ").nth(1).unwrap())
        .flat_map(|l| l.split_whitespace())
        .filter(|g| g.len() == 2 || g.len() == 4 || g.len() == 3 || g.len() == 7)
        .count();
    println!("Part 1: {}", count);
}

fn check_without_mask(segment_bitmask: u8, index: usize, test: char, input: &str) -> bool {
    ((segment_bitmask & bit!(index)) != 0) == (input.contains(test))
}

fn check_with_mask(segment_bitmask: u8, index: usize, test: char, input: &str) -> bool {
    if (bit!(index) & !segment_bitmask) != 0 {
        true
    } else {
        check_without_mask(segment_bitmask, index, test, input)
    }
}

/* groups:
    length 2: 1 -> SEG1
    length 3: 7 -> SEG7
    length 4: 4 -> SEG4
    length 5: 235 -> 0b01--1--1
    length 6: 069 -> 0b011---11
    length 7: 7 -> SEG7
    length 8: 8 -> SEG8
*/
fn is_possible(index: usize, test: char, input: &str) -> bool {
    let retval = match input.len() {
        2 => check_without_mask(SEG1, index, test, input),
        3 => check_without_mask(SEG7, index, test, input),
        4 => check_without_mask(SEG4, index, test, input),
        5 => check_with_mask(SEG2 & SEG3 & SEG5, index, test, input),
        6 => check_with_mask(SEG0 & SEG6 & SEG9, index, test, input),
        7 => true,
        _ => panic!("Invalid input length '{}'", input),
    };
    // println!("{}/{}: {} -> {}", index, test, input, retval);
    return retval;
}

fn clean_possibilities(possibilities: &mut [Vec<char>; 7]) {
    let mut changed = true;
    while changed {
        changed = false;
        let mut fixed: Vec<char> = Vec::new();
        for i in 0..possibilities.len() {
            if possibilities[i].len() == 1 {
                fixed.push(possibilities[i][0]);
            }
        }
        for i in 0..possibilities.len() {
            let vec = &possibilities[i];
            if vec.len() > 1 {
                let removed_elements = possibilities[i].drain_filter(|c| fixed.contains(c));
                if removed_elements.count() > 0 {
                    changed = true;
                }
            }
        }
    }
}

fn str_to_num(mappings: &HashMap<char, usize>, s: &str) -> usize {
    let mut bitmask = 0;
    for c in s.chars() {
        let pos = mappings.get(&c).unwrap();
        bitmask |= bit!(pos);
    }
    match SEGS.iter().position(|&seg| seg == bitmask) {
        Some(value) => return value,
        None => panic!("Could not find segment for {} (bitmask {})", s, bitmask),
    }
}

fn get_output_value(line: &str) -> usize {
    let split: Vec<&str> = line.split(" | ").collect();
    let inputs: Vec<&str> = split[0].split_whitespace().collect();
    let outputs: Vec<&str> = split[1].split_whitespace().collect();

    let mut possibilities: [Vec<char>; 7] =
        [vec![], vec![], vec![], vec![], vec![], vec![], vec![]];
    for i in 0..possibilities.len() {
        for c in ['a', 'b', 'c', 'd', 'e', 'f', 'g'] {
            if inputs.iter().all(|input| is_possible(i, c, input)) {
                possibilities[i].push(c);
            }
        }
        clean_possibilities(&mut possibilities);
    }

    if possibilities.iter().any(|v| v.len() != 1) {
        println!("{}", line);
        for i in 0..possibilities.len() {
            println!("{}: {:?}", i, possibilities[i])
        }
        panic!("Could not determine mapping");
    }

    let mut mappings = HashMap::<char, usize>::new();
    for i in 0..possibilities.len() {
        mappings.insert(possibilities[i][0], i);
    }

    let mut output_value: usize = 0;
    for output in outputs {
        output_value = 10 * output_value + str_to_num(&mappings, output);
    }

    return output_value;
}

fn part2(lines: &readfile::Lines) {
    let mut sum = 0;
    for l in lines.lines() {
        sum += get_output_value(l);
    }
    println!("Part 2: {}", sum);
}

pub fn run() {
    let lines = readfile::Lines::new("day8.txt");
    part1(&lines);
    part2(&lines);
}
