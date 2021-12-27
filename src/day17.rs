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

fn get_pos_for_step(vx_0: i32, vy_0: i32, step: i32) -> [i32; 2] {
  let mut vx = vx_0;
  let mut vy = vy_0;
  let mut x = 0;
  let mut y = 0;
  for _ in 1..=step {
    x += vx;
    y += vy;
    if vx > 0 {
      vx -= 1;
    }
    vy -= 1;
  }
  return [x, y];
}

fn get_min_steps_for_x(target: &TargetArea) -> i32 {
  let mut s = 1;
  loop {
    let x = s * s - (((s - 1) * (s - 1) + s - 1) / 2);
    if x > target.xmin {
      return s;
    }
    s += 1;
  }
}

/**
 * (s, vy_0)
 */
fn get_steps_for_max_y(target: &TargetArea, vx_0: i32) -> (i32, i32) {
  let mut y_guess = 1;
  loop {
    let mut s = vx_0;
    let mut prev_y = 0;
    'inner: loop {
      let x_res = s * vx_0 - (((s - 1) * (s - 1) + s - 1) / 2);
      let y_res = s * y_guess - (((s - 1) * (s - 1) + s - 1) / 2);
      if prev_y > target.ymax && y_res < target.ymin {
        return (s, y_guess - 1);
      } else if x_res >= target.xmin
        && x_res <= target.xmax
        && y_res >= target.ymin
        && y_res <= target.ymax
      {
        break 'inner;
      } else if y_res < target.ymin {
        break 'inner;
      }
      prev_y = y_res;
      s += 1;
    }
    y_guess += 1;
  }
}

fn part1(lines: &readfile::Lines) {
  let target = TargetArea::new(lines.lines().next().unwrap());
  // let target = TargetArea::new("target area: x=20..30, y=-10..-5");

  let vx_0 = get_min_steps_for_x(&target);
  // let (steps, vy_0) = get_steps_for_max_y(&target, vx_0);

  /*let max_y = (1..=steps)
  .map(|s| s * vy_0 - (((s - 1) * (s - 1) + s - 1) / 2))
  .max()
  .unwrap();*/

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
