use crate::readfile::readfile;

fn part1(lines: &readfile::Lines) {
    let mut increases = 0;
    let mut current_value = 0;
    for (i, line) in lines.lines().enumerate() {
        let value: i32 = line.parse().unwrap();
        if i > 0 && value > current_value {
            increases += 1;
        }
        current_value = value;
    }
    println!("Part 1: {}", increases);
}

fn part2(lines: &readfile::Lines) {
    let num_values: Vec<i32> = lines.lines().map(|l| l.parse().unwrap()).collect();
    let mut increases = 0;
    let mut current_value = 0;
    for i in 0..num_values.len() - 2 {
        let value = num_values[i] + num_values[i + 1] + num_values[i + 2];
        if i > 0 && value > current_value {
            increases += 1;
        }
        current_value = value;
    }
    println!("Part 2: {}", increases);
}

pub fn run() {
    let lines = readfile::Lines::new("day1.txt");
    part1(&lines);
    part2(&lines);
}
