use crate::readfile;
use std::collections::HashMap;

struct AdjacencyList {
    list: HashMap<String, Vec<String>>,
}

impl AdjacencyList {
    pub fn new(lines: &readfile::Lines) -> AdjacencyList {
        let mut adjacency: HashMap<String, Vec<String>> = HashMap::new();
        for l in lines.lines() {
            let parts: Vec<&str> = l.split('-').collect();
            let from = parts[0];
            let to = parts[1];
            if !adjacency.contains_key(from) {
                adjacency.insert(String::from(from), Vec::new());
            }
            adjacency.get_mut(from).unwrap().push(String::from(to));
            if !adjacency.contains_key(to) {
                adjacency.insert(String::from(to), Vec::new());
            }
            adjacency.get_mut(to).unwrap().push(String::from(from));
        }
        return AdjacencyList { list: adjacency };
    }

    pub fn find_paths(&self, node: &str, visited: &[String], allow_twice: bool) -> usize {
        if node == "end" {
            return 1;
        }

        let mut vclone = visited.to_owned();
        vclone.push(String::from(node));
        let empty_vec: Vec<String> = Vec::new();
        let adjacency_for_node: &Vec<String> = match self.list.get(node) {
            Some(v) => v,
            None => &empty_vec,
        };
        let mut sum = 0;
        for n in adjacency_for_node {
            if n == "start" {
                continue;
            }

            let mut still_allow_twice = allow_twice;
            if n.chars().next().unwrap().is_ascii_lowercase() && vclone.contains(n) {
                if allow_twice {
                    still_allow_twice = false;
                } else {
                    continue;
                }
            }
            sum += self.find_paths(n, &vclone, still_allow_twice);
        }

        return sum;
    }
}

fn part1(lines: &readfile::Lines) {
    let adjacency = AdjacencyList::new(lines);
    let visited: Vec<String> = Vec::new();
    let num_paths = adjacency.find_paths("start", &visited, false);
    println!("Part 1: {}", num_paths);
}

fn part2(lines: &readfile::Lines) {
    let adjacency = AdjacencyList::new(lines);
    let visited: Vec<String> = Vec::new();
    let num_paths = adjacency.find_paths("start", &visited, true);
    println!("Part 2: {}", num_paths);
}

pub fn run() {
    let lines = readfile::Lines::new("day12.txt");
    part1(&lines);
    part2(&lines);
}
