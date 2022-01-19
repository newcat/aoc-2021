use crate::readfile;
use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashSet;

type Point = [isize; 3];

struct Operation {
    target_state: bool,
    x_min: isize,
    x_max: isize,
    y_min: isize,
    y_max: isize,
    z_min: isize,
    z_max: isize,
}

impl Operation {
    pub fn new(s: &str) -> Operation {
        lazy_static! {
            static ref RE: Regex = Regex::new(
                "(on|off) x=(-?\\d+)..(-?\\d+),y=(-?\\d+)..(-?\\d+),z=(-?\\d+)..(-?\\d+)"
            )
            .unwrap();
        }
        let groups = RE.captures(s).unwrap();
        let x1: isize = groups[2].parse().unwrap();
        let x2: isize = groups[3].parse().unwrap();
        let y1: isize = groups[4].parse().unwrap();
        let y2: isize = groups[5].parse().unwrap();
        let z1: isize = groups[6].parse().unwrap();
        let z2: isize = groups[7].parse().unwrap();
        return Operation {
            target_state: &groups[1] == "on",
            x_min: x1.min(x2),
            x_max: x1.max(x2),
            y_min: y1.min(y2),
            y_max: y1.max(y2),
            z_min: z1.min(z2),
            z_max: z1.max(z2),
        };
    }

    pub fn perform(&self, state: &mut HashSet<Point>) {
        let range = -50..=50;
        let is_contained = [
            self.x_min, self.x_max, self.y_min, self.y_max, self.z_min, self.z_max,
        ]
        .iter()
        .all(|v| range.contains(v));
        if !is_contained {
            return;
        }

        for x in self.x_min..=self.x_max {
            for y in self.y_min..=self.y_max {
                for z in self.z_min..=self.z_max {
                    if self.target_state {
                        state.insert([x, y, z]);
                    } else {
                        state.remove(&[x, y, z]);
                    }
                }
            }
        }
    }
}

fn part1(lines: &readfile::Lines) {
    let operations: Vec<Operation> = lines.lines().map(Operation::new).collect();
    let mut state: HashSet<Point> = HashSet::new();
    for op in operations {
        op.perform(&mut state);
    }
    println!("Part 1: {}", state.len());
}

fn part2(_lines: &readfile::Lines) {}

pub fn run() {
    let lines = readfile::Lines::new("day22.txt");
    part1(&lines);
    part2(&lines);
}
