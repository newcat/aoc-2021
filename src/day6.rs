use crate::readfile::readfile;

type State = [u64; 9];

fn parse_state(line: &str) -> State {
    let mut state: State = [0; 9];
    let timers = line.split(',').map(|v| usize::from_str_radix(v, 10).unwrap());
    for t in timers {
        // let's just ignore that an easy access out of bounds is possible here
        state[t] += 1;
    }
    return state;
}

fn tick(state: &mut State) {
    let num_fish_to_add = state[0];
    for i in 1..9 {
        state[i - 1] = state[i];
        state[i] = 0;
    }
    state[6] += num_fish_to_add;
    state[8] += num_fish_to_add;
}

fn part1(lines: &readfile::Lines) {
    let mut state = parse_state(lines.lines().next().unwrap());
    for _ in 0..80 {
        tick(&mut state);
    }
    let num_fish: u64 = state.into_iter().sum();
    println!("Part 1: {}", num_fish);
}

fn part2(lines: &readfile::Lines) {
    let mut state = parse_state(lines.lines().next().unwrap());
    for _ in 0..256 {
        tick(&mut state);
    }
    let num_fish: u64 = state.into_iter().sum();
    println!("Part 2: {}", num_fish);
}

pub fn run() {
    let lines = readfile::Lines::new("day6.txt");
    part1(&lines);
    part2(&lines);
}
