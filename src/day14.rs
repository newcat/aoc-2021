use crate::readfile::readfile;
use std::collections::HashMap;

type Pair = [char; 2];

trait UpsertMapTrait<K> {
    fn insert_or_increase(&mut self, pair: &K, amount: usize);
    fn upsert_with_value(&mut self, pair: &K, value: usize);
}

type PairMap = HashMap<Pair, usize>;
impl<K> UpsertMapTrait<K> for HashMap<K, usize>
where
    K: std::cmp::Eq + std::hash::Hash + Copy + std::fmt::Debug,
{
    fn insert_or_increase(&mut self, k: &K, amount: usize) {
        if self.contains_key(k) {
            let val_ref = self.get_mut(k).unwrap();
            *val_ref = *val_ref + amount;
        } else {
            self.insert(*k, amount);
        }
    }

    fn upsert_with_value(&mut self, k: &K, value: usize) {
        if self.contains_key(k) {
            let val_ref = self.get_mut(k).unwrap();
            *val_ref = value;
        } else {
            self.insert(*k, value);
        }
    }
}

struct Chemistry {
    pairs: PairMap,
    rules: HashMap<Pair, char>,
    last_element: char,
}

impl Chemistry {
    pub fn new(lines: &readfile::Lines) -> Chemistry {
        let mut iter = lines.lines();
        let polymer: Vec<char> = String::from(iter.next().unwrap()).chars().collect();
        let mut pairs = PairMap::new();
        for i in 0..polymer.len() - 1 {
            pairs.insert_or_increase(&[polymer[i], polymer[i + 1]], 1);
        }

        let mut rules: HashMap<Pair, char> = HashMap::new();
        iter.next();
        for l in iter {
            let parts: Vec<&str> = l.split(" -> ").collect();
            let pair: Vec<char> = parts[0].chars().collect();
            rules.insert([pair[0], pair[1]], parts[1].chars().next().unwrap());
        }

        return Chemistry {
            pairs: pairs,
            rules: rules,
            last_element: polymer[polymer.len() - 1],
        };
    }

    pub fn step(&mut self) {
        let current_pairs = self.pairs.clone();
        for (pair, num) in current_pairs.iter() {
            if self.rules.contains_key(pair) {
                let occurences = *num;
                let insert = *self.rules.get(pair).unwrap();
                *self.pairs.get_mut(pair).unwrap() -= occurences;
                self.pairs
                    .insert_or_increase(&[pair[0], insert], occurences);
                self.pairs
                    .insert_or_increase(&[insert, pair[1]], occurences);
            }
        }

        // remove all pairs with value 0
        self.pairs.retain(|_k, v| *v > 0);
    }
}

fn iterate(lines: &readfile::Lines, steps: usize) -> usize {
    let mut chem = Chemistry::new(lines);

    for _ in 0..steps {
        chem.step();
    }

    let mut char_occurences: HashMap<char, usize> = HashMap::new();
    for (pair, num) in chem.pairs.iter() {
        char_occurences.insert_or_increase(&pair[0], *num);
    }
    char_occurences.insert_or_increase(&chem.last_element, 1);

    let mut occurences: Vec<usize> = char_occurences.values().map(|v| *v).collect();
    occurences.sort();
    return occurences[occurences.len() - 1] - occurences[0];
}

fn part1(lines: &readfile::Lines) {
    let result = iterate(lines, 10);
    println!("Part 1: {}", result);
}

fn part2(lines: &readfile::Lines) {
    let result = iterate(lines, 40);
    println!("Part 2: {}", result);
}

pub fn run() {
    let lines = readfile::Lines::new("day14.txt");
    part1(&lines);
    part2(&lines);
}
