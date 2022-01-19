use crate::readfile;

fn parse(s: &str) -> Vec<usize> {
    s.split(',')
        .map(|v| v.parse::<usize>().unwrap())
        .collect::<Vec<usize>>()
}

fn calculate_optimum(positions: &[usize], fuel_fn: impl Fn(usize, usize) -> usize) -> usize {
    let min = *positions.iter().min().unwrap();
    let max = *positions.iter().max().unwrap();
    let mut min_fuel = usize::MAX;
    for target_pos in min..=max {
        let fuel_needed: usize = positions.iter().map(|p| fuel_fn(*p, target_pos)).sum();
        if fuel_needed < min_fuel {
            min_fuel = fuel_needed;
        }
    }
    return min_fuel;
}

fn part1(positions: &[usize]) {
    let min_fuel = calculate_optimum(positions, |p, t| p.abs_diff(t));
    println!("Part 1: {}", min_fuel);
}

fn part2(positions: &[usize]) {
    let min_fuel = calculate_optimum(positions, |p, t| {
        let d = p.abs_diff(t);
        (d * (d + 1)) / 2
    });
    println!("Part 2: {}", min_fuel);
}

pub fn run() {
    let lines = readfile::Lines::new("day7.txt");
    let positions = parse(lines.lines().next().unwrap());
    part1(&positions);
    part2(&positions);
}
