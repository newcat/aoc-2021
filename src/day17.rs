use crate::readfile::readfile;
use regex::Regex;

struct TargetArea {
  xmin: i32,
  xmax: i32,
  ymin: i32,
  ymax: i32,
}

impl TargetArea {
  pub fn new(s: &str) -> TargetArea {
    let re = Regex::new("target area: x=(-?\\d+)..(-?\\d+), y=(-?\\d+)..(-?\\d+)").unwrap();
    let groups = re.captures(s).unwrap();
    return TargetArea {
      xmin: groups[1].parse().unwrap(),
      xmax: groups[2].parse().unwrap(),
      ymin: groups[3].parse().unwrap(),
      ymax: groups[4].parse().unwrap(),
    };
  }
}

#[derive(Copy, Clone)]
struct SimulationResult {
  vx_0: i32,
  vy_0: i32,
  steps: i32,
  max_y: i32,
}

fn simulate(target: &TargetArea, vx_0: i32, vy_0: i32) -> Option<SimulationResult> {
  let mut vx = vx_0;
  let mut vy = vy_0;
  let mut x = 0;
  let mut y = 0;
  let mut step = 0;
  let mut max_y = 0;

  loop {
    step += 1;
    x += vx;
    y += vy;

    if y > max_y {
      max_y = y;
    }

    if x >= target.xmin && x <= target.xmax && y >= target.ymin && y <= target.ymax {
      // we are in the target area
      return Some(SimulationResult {
        vx_0: vx_0,
        vy_0: vy_0,
        steps: step,
        max_y: max_y,
      });
    } else if x > target.xmax || y < target.ymin {
      return None;
    }

    if vx > 0 {
      vx -= 1;
    }
    vy -= 1;
  }
}

fn fuzz(target: &TargetArea, min_x_guess: i32) -> (SimulationResult, usize) {
  let mut best_simulation: Option<SimulationResult> = None;
  let mut valid_simulations = 0;

  for x_guess in min_x_guess..=target.xmax {
    for y_guess in 0.min(target.ymin)..1000 {
      let simulation = simulate(target, x_guess, y_guess);
      if simulation.is_some() {
        valid_simulations += 1;
        if best_simulation.is_none() || simulation.unwrap().max_y > best_simulation.unwrap().max_y {
          best_simulation = simulation;
        }
      }
    }
  }

  return (best_simulation.unwrap(), valid_simulations);
}

fn part1(lines: &readfile::Lines) {
  let target = TargetArea::new(lines.lines().next().unwrap());

  let fuzz_result = fuzz(&target, 0);
  let SimulationResult {
    max_y,
    steps,
    vx_0,
    vy_0,
  } = fuzz_result.0;

  println!("{},{},{}", steps, vx_0, vy_0);
  println!("Part 1: {}", max_y);
}

fn part2(lines: &readfile::Lines) {
  let target = TargetArea::new(lines.lines().next().unwrap());
  let fuzz_result = fuzz(&target, 0);
  println!("Part 2: {}", fuzz_result.1);
}

pub fn run() {
  let lines = readfile::Lines::new("day17.txt");
  part1(&lines);
  part2(&lines);
}
