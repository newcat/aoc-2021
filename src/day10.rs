use crate::readfile;

fn parse_line(line: &str) -> Result<Vec<char>, char> {
    let mut stack: Vec<char> = Vec::new();
    for c in line.chars() {
        match c {
            '(' | '{' | '[' | '<' => stack.push(c),
            ')' => {
                if stack.pop().unwrap() != '(' {
                    return Err(c);
                }
            }
            '}' => {
                if stack.pop().unwrap() != '{' {
                    return Err(c);
                }
            }
            ']' => {
                if stack.pop().unwrap() != '[' {
                    return Err(c);
                }
            }
            '>' => {
                if stack.pop().unwrap() != '<' {
                    return Err(c);
                }
            }
            _ => panic!("Invalid character: {}", c),
        }
    }
    return Ok(stack);
}

fn part1(lines: &readfile::Lines) {
    let mut sum: usize = 0;
    for l in lines.lines() {
        match parse_line(l) {
            Ok(_) => {}
            Err(c) => {
                sum += match c {
                    ')' => 3,
                    ']' => 57,
                    '}' => 1197,
                    '>' => 25137,
                    _ => 0,
                }
            }
        }
    }
    println!("Part 1: {}", sum);
}

fn part2(lines: &readfile::Lines) {
    let mut scores: Vec<usize> = Vec::new();
    for l in lines.lines() {
        if let Ok(mut remaining) = parse_line(l) {
            let mut score: usize = 0;
            while !remaining.is_empty() {
                score *= 5;
                let c = remaining.pop().unwrap();
                score += match c {
                    '(' => 1,
                    '{' => 3,
                    '[' => 2,
                    '<' => 4,
                    _ => 0,
                }
            }
            scores.push(score);
        }
    }
    scores.sort_unstable();
    println!("Part 2: {}", scores[scores.len() / 2]);
}

pub fn run() {
    let lines = readfile::Lines::new("day10.txt");
    part1(&lines);
    part2(&lines);
}
