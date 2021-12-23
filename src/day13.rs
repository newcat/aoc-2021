use crate::readfile::readfile;
use regex::Regex;
use std::collections::VecDeque;

type Point = [usize; 2];

enum FoldDirection {
    Horizontal = 0,
    Vertical = 1,
}

struct FoldInstruction {
    direction: FoldDirection,
    coordinate: usize,
}

struct Game {
    points: Vec<Point>,
    folds: VecDeque<FoldInstruction>,
}

impl Game {
    pub fn new(lines: &readfile::Lines) -> Game {
        let re = Regex::new("fold along (x|y)=(\\d+)").unwrap();
        let mut points: Vec<Point> = Vec::new();
        let mut folds: VecDeque<FoldInstruction> = VecDeque::new();

        // true if parsing points, false if parsing fold instructions
        let mut parse_mode = true;
        for l in lines.lines() {
            if l == "" {
                parse_mode = false;
                continue;
            }

            if parse_mode {
                let coordinates: Vec<&str> = l.split(",").collect();
                let p = [
                    coordinates[0].parse().unwrap(),
                    coordinates[1].parse().unwrap(),
                ];
                points.push(p);
            } else {
                let groups = re.captures(l).unwrap();
                let direction = match &groups[1] {
                    "x" => FoldDirection::Horizontal,
                    "y" => FoldDirection::Vertical,
                    _ => panic!("Invalid fold direction"),
                };
                let coordinate: usize = groups[2].parse().unwrap();
                folds.push_back(FoldInstruction {
                    direction: direction,
                    coordinate: coordinate,
                });
            }
        }
        points.sort();

        return Game {
            points: points,
            folds: folds,
        };
    }

    pub fn fold(&mut self) {
        let instruction = self.folds.pop_front().unwrap();
        let coord_index = instruction.direction as usize;
        let coordinate = instruction.coordinate;
        let mut new_points: Vec<Point> = self
            .points
            .iter()
            .map(|p| {
                if p[coord_index] > coordinate {
                    let mut new_point = p.clone();
                    new_point[coord_index] = 2 * coordinate - p[coord_index];
                    return new_point;
                } else {
                    return *p;
                }
            })
            .collect();
        new_points.sort();
        new_points.dedup();
        self.points = new_points;
    }

    pub fn print(&self) {
        let max_x = self.points.iter().map(|p| p[0]).max().unwrap();
        let max_y = self.points.iter().map(|p| p[1]).max().unwrap();
        for y in 0..=max_y {
            for x in 0..=max_x {
                if self.points.contains(&[x, y]) {
                    print!("#");
                } else {
                    print!(" ");
                }
            }
            println!("");
        }
    }
}

fn part1(lines: &readfile::Lines) {
    let mut game = Game::new(lines);
    game.fold();
    println!("Part 1: {}", game.points.len());
}

fn part2(lines: &readfile::Lines) {
    let mut game = Game::new(lines);
    while game.folds.len() > 0 {
        game.fold();
    }
    game.print();
}

pub fn run() {
    let lines = readfile::Lines::new("day13.txt");
    part1(&lines);
    part2(&lines);
}
