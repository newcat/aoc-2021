use crate::readfile;
use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::collections::HashMap;

const PRINT_PATH: bool = false;

type Point = [isize; 2];

struct Grid {
    multiplier: usize,
    costs: Vec<usize>,
    len_x: usize,
    len_y: usize,
}

impl Grid {
    pub fn new(lines: &readfile::Lines, multiplier: usize) -> Grid {
        let all: Vec<&str> = lines.lines().collect();
        let costs = all
            .iter()
            .flat_map(|l| l.chars().map(|c| c.to_digit(10).unwrap() as usize))
            .collect();
        return Grid {
            multiplier,
            costs,
            len_x: all[0].len(),
            len_y: all.len(),
        };
    }

    pub fn get(&self, x: isize, y: isize) -> Option<usize> {
        if x < 0
            || y < 0
            || x >= (self.multiplier * self.len_x) as isize
            || y >= (self.multiplier * self.len_y) as isize
        {
            return None;
        };

        let add_x = x as usize / self.len_x;
        let add_y = y as usize / self.len_y;
        let mod_x = x as usize % self.len_x;
        let mod_y = y as usize % self.len_y;

        let pos = mod_y * self.len_x + mod_x;
        if pos >= self.costs.len() {
            panic!("This shouldn't happen");
        }
        let mut value = self.costs[pos] + add_x + add_y;
        if value > 9 {
            value = (value % 10) + 1;
        }
        return Some(value);
    }
}

#[derive(Copy, Clone, Eq, PartialEq)]
struct DijkstraPointData {
    distance: usize,
    point: Point,
    // prev: Option<Point>,
}

impl Ord for DijkstraPointData {
    fn cmp(&self, other: &Self) -> Ordering {
        other
            .distance
            .cmp(&self.distance)
            .then_with(|| self.point.cmp(&other.point))
    }
}

impl PartialOrd for DijkstraPointData {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn get_neighbors(all_points: &Grid, point: Point) -> Vec<Point> {
    let mut neighbors: Vec<Point> = Vec::new();
    let candidates: [Point; 4] = [
        [point[0] + 1, point[1]],
        [point[0] - 1, point[1]],
        [point[0], point[1] + 1],
        [point[0], point[1] - 1],
    ];
    for p in candidates {
        if all_points.get(p[0], p[1]).is_some() {
            neighbors.push(p);
        }
    }
    return neighbors;
}

fn run_dijkstra(all_points: &Grid) -> usize {
    let mut dist: HashMap<Point, usize> = HashMap::new();
    let mut heap: BinaryHeap<DijkstraPointData> = BinaryHeap::new();
    let target: Point = [
        (all_points.multiplier * all_points.len_x) as isize - 1,
        (all_points.multiplier * all_points.len_y) as isize - 1,
    ];

    for y in 0..=target[1] {
        for x in 0..=target[0] {
            let p: Point = [x as isize, y as isize];
            dist.insert(p, usize::MAX);
        }
    }

    *dist.get_mut(&[0, 0]).unwrap() = 0;
    heap.push(DijkstraPointData {
        point: [0, 0],
        distance: 0,
    });

    while let Some(data) = heap.pop() {
        if data.point[0] == target[0] && data.point[1] == target[1] {
            // return data.distance;
            break;
        }

        if data.distance > *dist.get(&data.point).unwrap() {
            continue;
        }

        for n in get_neighbors(all_points, data.point) {
            let next = DijkstraPointData {
                point: n,
                distance: data.distance + all_points.get(n[0], n[1]).unwrap(),
            };
            if next.distance < *dist.get(&n).unwrap() {
                heap.push(next);
                *dist.get_mut(&n).unwrap() = next.distance;
            }
        }
    }

    if PRINT_PATH {
        let mut p = target;
        let mut path: Vec<Point> = vec![target];
        loop {
            if p[0] == 0 && p[1] == 0 {
                break;
            }
            let (best_neighbor, _) = get_neighbors(all_points, p)
                .iter()
                .map(|n| (*n, *dist.get(n).unwrap()))
                .filter(|(_, d)| *d < dist[&p])
                .min_by(|(_, a), (_, b)| a.cmp(b))
                .expect(&format!("{},{}: {}", p[0], p[1], dist[&p])[..]);
            path.push(best_neighbor);
            p = best_neighbor;
        }
        path.reverse();
        for p in path {
            println!("{},{}: {}", p[0], p[1], dist[&p]);
        }
    }

    return *dist.get(&target).unwrap();
}

fn part1(lines: &readfile::Lines) {
    let grid = Grid::new(lines, 1);
    let distance = run_dijkstra(&grid);
    println!("Part 1: {}", distance);
}

fn part2(lines: &readfile::Lines) {
    let grid = Grid::new(lines, 5);
    let distance = run_dijkstra(&grid);
    println!("Part 2: {}", distance);
}

pub fn run() {
    let lines = readfile::Lines::new("day15.txt");
    part1(&lines);
    part2(&lines);
}
