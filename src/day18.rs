use crate::readfile::readfile;
use indextree::{Arena, NodeId};

unsafe fn to_mut<T>(reference: &T) -> &mut T {
    let const_ptr = reference as *const T;
    let mut_ptr = const_ptr as *mut T;
    &mut *mut_ptr
}

#[derive(Copy, Clone)]
enum SnailfishNumberType {
    Value,
    Pair,
}

#[derive(Copy, Clone)]
struct SnailfishNumber {
    node_type: SnailfishNumberType,
    index: usize,
    value: usize,
}

impl SnailfishNumber {
    pub fn new_value(value: usize) -> SnailfishNumber {
        SnailfishNumber {
            node_type: SnailfishNumberType::Value,
            index: 0,
            value: value,
        }
    }

    pub fn new_pair() -> SnailfishNumber {
        SnailfishNumber {
            node_type: SnailfishNumberType::Pair,
            index: 0,
            value: 0,
        }
    }
}

impl std::fmt::Display for SnailfishNumber {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.node_type {
            SnailfishNumberType::Value => write!(f, "{}", self.value),
            SnailfishNumberType::Pair => write!(f, ""),
        }
    }
}

struct Tree {
    arena: Arena<SnailfishNumber>,
    root_node: Option<NodeId>,
}

impl Tree {
    pub fn new(s: &str) -> Tree {
        let mut tree = Tree {
            arena: Arena::new(),
            root_node: None,
        };
        let root_node = tree.parse_number(s);
        tree.root_node = Some(root_node);
        tree.reduce();
        return tree;
    }

    fn parse_number(&mut self, s: &str) -> NodeId {
        if s.chars().all(|c| c.is_digit(10)) {
            // number literal
            return self
                .arena
                .new_node(SnailfishNumber::new_value(s.parse().unwrap()));
        } else {
            // pair
            let chars: Vec<char> = s.chars().collect();
            if chars[0] != '[' || *chars.last().unwrap() != ']' {
                panic!("Invalid snailfish number: {}", s);
            }
            let mut level = 0;
            let mut comma_position = 0;
            for (i, c) in chars.iter().enumerate() {
                match *c {
                    '[' => {
                        level += 1;
                    }
                    ']' => {
                        level -= 1;
                    }
                    ',' => {
                        if level == 1 {
                            comma_position = i;
                        }
                    }
                    _ => {}
                }
            }
            let num1 = &s[1..comma_position];
            let num2 = &s[comma_position + 1..s.len() - 1];
            let node1 = self.parse_number(num1);
            let node2 = self.parse_number(num2);
            let node = self.arena.new_node(SnailfishNumber::new_pair());
            node.append(node1, &mut self.arena);
            node.append(node2, &mut self.arena);
            return node;
        }
    }

    pub fn get_children(&self, node: NodeId) -> Option<(NodeId, NodeId)> {
        let data = self.arena.get(node).unwrap().get();
        match data.node_type {
            SnailfishNumberType::Value => None,
            SnailfishNumberType::Pair => {
                let mut children = node.children(&self.arena);
                let node1 = children.next().unwrap();
                let node2 = children.next().unwrap();
                Some((node1, node2))
            }
        }
    }

    pub fn find_index(&self, index: usize) -> Option<NodeId> {
        return self.find_index_recursive(self.root_node.unwrap(), index);
    }

    pub fn find_index_left(&self, index: usize) -> Option<NodeId> {
        match index {
            0 => None,
            _ => self.find_index(index - 1),
        }
    }

    pub fn find_index_right(&self, index: usize) -> Option<NodeId> {
        self.find_index(index + 1)
    }

    pub fn update_index(&mut self) {
        let mut index: usize = 0;
        let arena = &mut self.arena;
        for n in self.root_node.unwrap().descendants(arena) {
            let node = arena.get(n).unwrap();
            if matches!(node.get().node_type, SnailfishNumberType::Value) {
                unsafe { to_mut(node.get()) }.index = index;
                index = index + 1;
            }
        }
    }

    pub fn get_value(&self, node_id: NodeId) -> Option<usize> {
        self.arena.get(node_id).and_then(|n| Some(n.get().value))
    }

    pub fn set_value(&mut self, node_id: NodeId, value: usize) {
        if let Some(node) = self.arena.get_mut(node_id) {
            node.get_mut().value = value;
        }
    }

    pub fn get_magnitude(&self, node_id: NodeId) -> usize {
        let data = self.arena.get(node_id).unwrap().get();
        match data.node_type {
            SnailfishNumberType::Value => data.value,
            SnailfishNumberType::Pair => {
                let (node1, node2) = self.get_children(node_id).unwrap();
                3 * self.get_magnitude(node1) + 2 * self.get_magnitude(node2)
            }
        }
    }

    pub fn add(&mut self, other: &str) {
        let node1 = self.root_node.unwrap();
        let node2 = self.parse_number(other);

        let new_node = self.arena.new_node(SnailfishNumber::new_pair());
        new_node.append(node1, &mut self.arena);
        new_node.append(node2, &mut self.arena);

        self.root_node = Some(new_node);
        self.reduce();
    }

    fn find_index_recursive(&self, node: NodeId, index: usize) -> Option<NodeId> {
        let node_ref = self.arena.get(node).unwrap();
        match node_ref.get().node_type {
            SnailfishNumberType::Value => {
                if node_ref.get().index == index {
                    return Some(node);
                } else {
                    return None;
                }
            }
            SnailfishNumberType::Pair => {
                for n in node.children(&self.arena) {
                    match self.find_index_recursive(n, index) {
                        Some(result) => return Some(result),
                        None => {}
                    }
                }
                return None;
            }
        }
    }

    fn to_string(&self, node: NodeId) -> String {
        let data = self.arena.get(node).unwrap().get();
        match data.node_type {
            SnailfishNumberType::Value => format!("{}", data.value),
            SnailfishNumberType::Pair => {
                let (node1, node2) = self.get_children(node).unwrap();
                format!("[{},{}]", self.to_string(node1), self.to_string(node2))
            }
        }
    }

    // REDUCING

    fn explode(&mut self, node_id: NodeId) {
        let (node1, node2) = {
            let (n1, n2) = self.get_children(node_id).unwrap();
            (
                *self.arena.get(n1).unwrap().get(),
                *self.arena.get(n2).unwrap().get(),
            )
        };
        if let Some(left) = self.find_index_left(node1.index) {
            let v = self.get_value(left).unwrap();
            self.set_value(left, v + node1.value);
        }
        if let Some(right) = self.find_index_right(node2.index) {
            let v = self.get_value(right).unwrap();
            self.set_value(right, v + node2.value);
        }
        let parent = self.arena.get(node_id).unwrap().parent().unwrap();
        let (index, _n) = parent
            .children(&self.arena)
            .enumerate()
            .find(|(_i, n)| *n == node_id)
            .unwrap();
        node_id.remove_subtree(&mut self.arena);
        let new_node = self.arena.new_node(SnailfishNumber::new_value(0));
        if index == 0 {
            parent.prepend(new_node, &mut self.arena);
        } else {
            parent.append(new_node, &mut self.arena);
        }
    }

    fn reduce_dfs_explode(&mut self, node_id: NodeId, level: usize) -> bool {
        match self.arena.get(node_id).unwrap().get().node_type {
            SnailfishNumberType::Value => false,
            SnailfishNumberType::Pair => {
                if level == 4 {
                    self.explode(node_id);
                    true
                } else {
                    let mut any_true = false;
                    for n in node_id.children(&self.arena) {
                        any_true =
                            any_true || unsafe { to_mut(self) }.reduce_dfs_explode(n, level + 1);
                    }
                    any_true
                }
            }
        }
    }

    fn find_node_to_split(&self) -> Option<NodeId> {
        for n in self.root_node.unwrap().descendants(&self.arena) {
            let node = self.arena.get(n).unwrap();
            if node.get().value > 9 {
                return Some(n);
            }
        }
        None
    }

    fn split(&mut self) -> bool {
        if let Some(node_id) = self.find_node_to_split() {
            let node = self.arena.get_mut(node_id).unwrap();
            let mut data = node.get_mut();
            let left = num::integer::div_floor(data.value, 2);
            let right = num::integer::div_ceil(data.value, 2);
            data.value = 0;
            data.node_type = SnailfishNumberType::Pair;
            node_id.append(
                self.arena.new_node(SnailfishNumber::new_value(left)),
                &mut self.arena,
            );
            node_id.append(
                self.arena.new_node(SnailfishNumber::new_value(right)),
                &mut self.arena,
            );
            true
        } else {
            false
        }
    }

    pub fn reduce(&mut self) {
        self.update_index();
        while self.reduce_dfs_explode(self.root_node.unwrap(), 0) || self.split() {
            self.update_index();
        }
    }
}

impl std::fmt::Display for Tree {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_string(self.root_node.unwrap()))
    }
}

fn part1(lines: &readfile::Lines) {
    let mut lines_iter = lines.lines();
    let mut tree = Tree::new(lines_iter.next().unwrap());
    tree.update_index();
    for l in lines_iter {
        tree.add(l);
    }
    println!("{}", tree.get_magnitude(tree.root_node.unwrap()));
}

fn part2(lines: &readfile::Lines) {
    let line_vec: Vec<&str> = lines.lines().collect();
    let mut max = 0;
    for ai in 0..line_vec.len() {
        for bi in 0..line_vec.len() {
            if ai == bi {
                continue;
            }
            let mut t = Tree::new(line_vec[ai]);
            t.add(line_vec[bi]);
            let mag = t.get_magnitude(t.root_node.unwrap());
            if mag > max {
                max = mag;
            }
        }
    }
    println!("{}", max);
}

pub fn run() {
    let lines = readfile::Lines::new("day18.txt");
    part1(&lines);
    part2(&lines);
}
