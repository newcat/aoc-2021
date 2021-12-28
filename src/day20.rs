use crate::readfile::readfile;
use std::collections::HashSet;

type Point = [isize; 2];

struct Image {
    are_outside_pixels_on: u8,
    light_pixels: HashSet<Point>,
    min_x: isize,
    max_x: isize,
    min_y: isize,
    max_y: isize,
}

impl Image {
    pub fn enhance(&self, algorithm: &Vec<u8>) -> Image {
        let mut enhanced = HashSet::new();

        for y in self.min_y - 1..=self.max_y + 1 {
            for x in self.min_x - 1..=self.max_x + 1 {
                let pixel_value = self.get_pixel_value(x, y);
                if algorithm[pixel_value] == 1 {
                    enhanced.insert([x, y]);
                }
            }
        }

        let are_outside_pixels_on = if self.are_outside_pixels_on == 1 {
            algorithm[511]
        } else {
            algorithm[0]
        };
        if are_outside_pixels_on == 1 {
            // top and bottom
            for y in [self.min_y - 2, self.max_y + 2] {
                for x in self.min_x - 2..=self.max_x + 2 {
                    enhanced.insert([x, y]);
                }
            }

            // left and right
            for y in self.min_y - 2..=self.max_y + 2 {
                for x in [self.min_x - 2, self.max_x + 2] {
                    enhanced.insert([x, y]);
                }
            }
        }

        return Image {
            are_outside_pixels_on: are_outside_pixels_on,
            light_pixels: enhanced,
            min_x: self.min_x - 1,
            max_x: self.max_x + 1,
            min_y: self.min_y - 1,
            max_y: self.max_y + 1,
        };
    }

    fn get_pixel_value(&self, x: isize, y: isize) -> usize {
        let mut i = 8;
        let mut value: u16 = 0;
        for cy in y - 1..=y + 1 {
            for cx in x - 1..=x + 1 {
                let px = if self.is_pixel_on(cx, cy) { 1 } else { 0 };
                value = value | (px << i);
                i -= 1;
            }
        }

        return value as usize;
    }

    fn is_pixel_on(&self, x: isize, y: isize) -> bool {
        if x < self.min_x || x > self.max_x || y < self.min_y || y > self.max_y {
            return self.are_outside_pixels_on == 1;
        }
        return self.light_pixels.contains(&[x, y]);
    }
}

impl std::fmt::Debug for Image {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        writeln!(
            f,
            "({} .. {}) ({} .. {}) | outside pixels: {}",
            self.min_x, self.max_x, self.min_y, self.max_y, self.are_outside_pixels_on
        )
        .expect("Fail");
        for y in self.min_y - 1..=self.max_y + 1 {
            for x in self.min_x - 1..=self.max_x + 1 {
                if self.light_pixels.contains(&[x, y]) {
                    write!(f, "#").expect("Fail");
                } else {
                    write!(f, ".").expect("Fail");
                }
            }
            writeln!(f, "").expect("Fail");
        }
        Ok(())
    }
}

fn parse_input(lines: &readfile::Lines) -> (Vec<u8>, Image) {
    let mut line_iter = lines.lines();

    let algorithm = line_iter
        .next()
        .unwrap()
        .chars()
        .map(|c| if c == '#' { 1 } else { 0 })
        .collect();

    line_iter.next();

    let mut y: isize = 0;
    let mut max_x: isize = 0;
    let mut light_pixels: HashSet<Point> = HashSet::new();
    for l in line_iter {
        if max_x == 0 {
            max_x = l.chars().count() as isize - 1;
        }
        for (x, c) in l.chars().enumerate() {
            if c == '#' {
                light_pixels.insert([x as isize, y]);
            }
        }
        y += 1;
    }
    let image = Image {
        are_outside_pixels_on: 0,
        light_pixels: light_pixels,
        min_x: 0,
        max_x: max_x,
        min_y: 0,
        max_y: y - 1,
    };

    return (algorithm, image);
}

fn part1(lines: &readfile::Lines) {
    let (algorithm, image) = parse_input(lines);
    let enhanced = image.enhance(&algorithm).enhance(&algorithm);
    println!("Part 1: {}", enhanced.light_pixels.len());
}

fn part2(lines: &readfile::Lines) {
    let (algorithm, image) = parse_input(lines);
    let mut enhanced = image;
    for _ in 0..50 {
        enhanced = enhanced.enhance(&algorithm);
    }
    println!("Part 2: {}", enhanced.light_pixels.len());
}

pub fn run() {
    let lines = readfile::Lines::new("day20.txt");
    part1(&lines);
    part2(&lines);
}
