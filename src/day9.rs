use crate::readfile;
use colored::*;

const PRINT_DEBUG: bool = false;

type Point = [usize; 2];
type Map = Vec<Vec<u32>>;

fn parse(lines: &readfile::Lines) -> Vec<Vec<u32>> {
    lines
        .lines()
        .map(|l| l.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect()
}

fn find_lowest_points(map: &Map) -> Vec<Point> {
    let mut lowest_points: Vec<Point> = Vec::new();
    for y in 0..map.len() {
        let vec = &map[y];
        for x in 0..vec.len() {
            let val = vec[x];
            let mut is_lowest = true;
            if y > 0 && map[y - 1][x] <= val {
                is_lowest = false;
            }
            if y < map.len() - 1 && map[y + 1][x] <= val {
                is_lowest = false;
            }
            if x > 0 && vec[x - 1] <= val {
                is_lowest = false;
            }
            if x < vec.len() - 1 && vec[x + 1] <= val {
                is_lowest = false;
            }
            if is_lowest {
                lowest_points.push([x, y]);
            }
        }
    }
    return lowest_points;
}

fn part1(lines: &readfile::Lines) {
    let map = parse(lines);
    let lowest_points = find_lowest_points(&map);
    let risk_level_sum: u32 = lowest_points.iter().map(|[x, y]| map[*y][*x] + 1).sum();

    if PRINT_DEBUG {
        for (y, map_y) in map.iter().enumerate() {
            for (x, v) in map_y.iter().enumerate() {
                if lowest_points.contains(&[x, y]) {
                    print!("{}", v.to_string().cyan());
                } else if *v < 9 {
                    print!("{}", v.to_string().bright_black());
                } else {
                    print!(" ");
                }
            }
            println!();
        }
    }

    println!("Part 1: {}", risk_level_sum);
}

fn find_points_to_visit(
    map: &Map,
    visited: &[Point],
    to_visit: &[Point],
    [x, y]: Point,
) -> Vec<Point> {
    let mut neighbours: Vec<Point> = Vec::new();
    if y > 0 {
        neighbours.push([x, y - 1]);
    }
    if y < map.len() - 1 {
        neighbours.push([x, y + 1]);
    }
    if x > 0 {
        neighbours.push([x - 1, y]);
    }
    if x < map[y].len() - 1 {
        neighbours.push([x + 1, y]);
    }
    return neighbours
        .into_iter()
        .filter(|p| map[p[1]][p[0]] != 9 && !visited.contains(p) && !to_visit.contains(p))
        .collect();
}

fn part2(lines: &readfile::Lines) {
    let map = parse(lines);
    let lowest_points = find_lowest_points(&map);
    let mut basins: Vec<usize> = Vec::new();

    for low_point in lowest_points {
        let mut visited: Vec<Point> = Vec::new();
        let mut to_visit: Vec<Point> = vec![low_point];
        let mut basin_size = 0;
        while !to_visit.is_empty() {
            let p = to_visit.pop().unwrap();
            basin_size += 1;
            visited.push(p);
            to_visit.extend(find_points_to_visit(&map, &visited, &to_visit, p));
        }
        basins.push(basin_size);
    }
    basins.sort_unstable();
    basins.reverse();
    let largest_sum: usize = basins.iter().take(3).product();
    println!("Part 2: {}", largest_sum);
}

pub fn run() {
    let lines = readfile::Lines::new("day9.txt");
    part1(&lines);
    part2(&lines);
}
