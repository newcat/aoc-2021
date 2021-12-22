use crate::readfile::readfile;
use colored::*;

#[derive(Copy, Clone)]
struct Octopus {
    energy: u8,
    has_flashed: bool,
}

struct Grid {
    octos: Vec<Octopus>,
    len_x: usize,
    len_y: usize,
}

impl Grid {
    pub fn new(lines: &readfile::Lines) -> Grid {
        let all: Vec<&str> = lines.lines().collect();
        let octos = all
            .iter()
            .flat_map(|l| {
                l.chars().map(|c| Octopus {
                    energy: c.to_digit(10).unwrap() as u8,
                    has_flashed: false,
                })
            })
            .collect();
        return Grid {
            octos: octos,
            len_x: all[0].len(),
            len_y: all.len(),
        };
    }

    pub fn get(&self, x: isize, y: isize) -> Option<&Octopus> {
        if x < 0 || y < 0 {
            return None;
        };

        let pos = (y as usize) * self.len_x + (x as usize);
        if pos >= self.octos.len() {
            return None;
        }
        return Some(&self.octos[pos]);
    }

    pub fn get_mut(&mut self, x: isize, y: isize) -> Option<&mut Octopus> {
        if x < 0 || y < 0 || x >= (self.len_x as isize) || y >= (self.len_y as isize) {
            return None;
        };

        let pos = (y as usize) * self.len_x + (x as usize);
        if pos >= self.octos.len() {
            return None;
        }
        return Some(&mut self.octos[pos]);
    }

    pub fn get_value(&self, x: isize, y: isize) -> Option<Octopus> {
        match self.get(x, y) {
            Some(o) => Some(*o),
            None => None,
        }
    }

    pub fn tick(&mut self) -> usize {
        for i in 0..self.octos.len() {
            self.octos[i].has_flashed = false;
            self.octos[i].energy += 1;
        }

        while self.flash() {}

        let mut n_flashes = 0;
        for i in 0..self.octos.len() {
            if self.octos[i].has_flashed {
                self.octos[i].energy = 0;
                n_flashes += 1;
            }
        }
        return n_flashes;
    }

    #[allow(dead_code)]
    pub fn print(&self) {
        for y in 0..(self.len_y as isize) {
            for x in 0..(self.len_x as isize) {
                let octo = self.get(x, y).unwrap();
                let energy = if octo.energy > 9 {
                    String::from("+")
                } else {
                    octo.energy.to_string()
                };
                if octo.has_flashed {
                    print!("{}", energy);
                } else {
                    print!("{}", energy.bright_black());
                }
            }
            println!("");
        }
    }

    fn flash(&mut self) -> bool {
        let mut flashed_some = false;
        for y in 0..(self.len_y as isize) {
            for x in 0..(self.len_x as isize) {
                if let Some(octo) = self.get_value(x, y) {
                    if octo.energy > 9 && !octo.has_flashed {
                        self.flash_octo(x, y);
                        self.get_mut(x, y).unwrap().has_flashed = true;
                        flashed_some = true;
                    }
                }
            }
        }
        return flashed_some;
    }

    fn flash_octo(&mut self, x: isize, y: isize) {
        for ny in [y - 1, y, y + 1] {
            for nx in [x - 1, x, x + 1] {
                if let Some(o) = self.get_mut(nx, ny) {
                    o.energy += 1
                }
            }
        }
    }
}

fn part1(lines: &readfile::Lines) {
    let mut grid = Grid::new(lines);
    let mut num_flashes = 0;
    for _ in 0..100 {
        num_flashes += grid.tick();
    }
    println!("Part 1: {}", num_flashes);
}

fn part2(lines: &readfile::Lines) {
    let mut grid = Grid::new(lines);
    let mut i = 1;
    while grid.tick() != grid.len_x * grid.len_y {
        i += 1;
    }
    println!("Part 2: {}", i);
}

pub fn run() {
    let lines = readfile::Lines::new("day11.txt");
    part1(&lines);
    part2(&lines);
}
