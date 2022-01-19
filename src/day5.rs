use crate::readfile;
use regex::Regex;
use std::cmp::Ordering;
use std::collections::HashMap;

#[derive(Hash, PartialEq, Eq)]
struct Point {
    x: u16,
    y: u16,
}

struct Line {
    x1: u16,
    y1: u16,
    x2: u16,
    y2: u16,
}

impl Line {
    fn is_diagonal(&self) -> bool {
        self.x1 != self.x2 && self.y1 != self.y2
    }
}

fn abs_diff(a: u16, b: u16) -> u16 {
    if a > b {
        a - b
    } else {
        b - a
    }
}

fn parse_lines(str_lines: &readfile::Lines) -> Vec<Line> {
    let reg: Regex = Regex::new("(\\d+),(\\d+) -> (\\d+),(\\d+)").unwrap();
    let mut lines = Vec::new();
    for line in str_lines.lines() {
        let groups = reg.captures(line).unwrap();
        lines.push(Line {
            x1: groups[1].parse().unwrap(),
            y1: groups[2].parse().unwrap(),
            x2: groups[3].parse().unwrap(),
            y2: groups[4].parse().unwrap(),
        });
    }
    return lines;
}

fn line_to_points(line: &Line) -> Vec<Point> {
    let mut points = Vec::new();

    let step_x: i16 = match line.x1.cmp(&line.x2) {
        Ordering::Less => 1,
        Ordering::Equal => 0,
        Ordering::Greater => -1,
    };

    let step_y: i16 = match line.y1.cmp(&line.y2) {
        Ordering::Less => 1,
        Ordering::Equal => 0,
        Ordering::Greater => -1,
    };

    let steps = match step_x {
        0 => abs_diff(line.y1, line.y2) + 1,
        _ => abs_diff(line.x1, line.x2) + 1,
    };

    let mut x = line.x1;
    let mut y = line.y1;
    for _ in 0..steps {
        points.push(Point { x, y });
        x = x.checked_add_signed(step_x).unwrap();
        y = y.checked_add_signed(step_y).unwrap();
    }

    return points;
}

fn get_intersecting_point_count(lines: &[Line], include_diagonals: bool) -> usize {
    let mut points: HashMap<Point, u16> = HashMap::new();
    for l in lines {
        if include_diagonals || !l.is_diagonal() {
            for p in line_to_points(l) {
                match points.get_mut(&p) {
                    Some(v) => {
                        *v += 1;
                    }
                    None => {
                        points.insert(p, 1);
                    }
                }
            }
        }
    }
    return points.values().filter(|v| **v > 1).count();
}

pub fn run() {
    let str_lines = readfile::Lines::new("day5.txt");
    let lines = parse_lines(&str_lines);
    println!("Part 1: {}", get_intersecting_point_count(&lines, false));
    println!("Part 2: {}", get_intersecting_point_count(&lines, true));
}
