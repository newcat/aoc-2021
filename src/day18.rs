use crate::readfile::readfile;
use trees::{Node, Tree};

enum SnailfishNumberType {
    Value,
    Pair,
}

struct SnailfishNumber {
    node_type: SnailfishNumberType,
    index: usize,
    value: usize,
}

impl std::fmt::Display for SnailfishNumber {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.node_type {
            SnailfishNumberType::Value => write!(f, "{}", self.value),
            SnailfishNumberType::Pair => write!(f, ""),
        }
    }
}

unsafe fn to_mut<T>(reference: &T) -> &mut T {
    let const_ptr = reference as *const T;
    let mut_ptr = const_ptr as *mut T;
    &mut *mut_ptr
}

fn find_index_recursive(
    node: &mut Node<SnailfishNumber>,
    index: usize,
) -> Option<&mut Node<SnailfishNumber>> {
    match node.data().node_type {
        SnailfishNumberType::Value => {
            if node.data().index == index {
                return Some(node);
            } else {
                return None;
            }
        }
        SnailfishNumberType::Pair => {
            for n in node.iter_mut() {
                match find_index_recursive(n.get_mut(), index) {
                    Some(result) => return Some(result),
                    None => {}
                }
            }
            return None;
        }
    }
}

fn find_index(
    node: &mut Node<SnailfishNumber>,
    index: usize,
) -> Option<&mut Node<SnailfishNumber>> {
    let mut root_node: &Node<SnailfishNumber> = node;
    loop {
        match node.parent() {
            Some(n) => {
                if std::ptr::eq(root_node, n) {
                    println!("Searching for {} at {}", index, root_node);
                    return find_index_recursive(unsafe { to_mut(root_node) }, index);
                } else {
                    root_node = n;
                }
            },
            None => return find_index_recursive(unsafe { to_mut(root_node) }, index),
        }
    }
}

fn update_index_dfs(node: &mut Node<SnailfishNumber>, index: &mut usize) {
    match node.data().node_type {
        SnailfishNumberType::Value => {
            node.data_mut().index = *index;
            *index = *index + 1;
        }
        SnailfishNumberType::Pair => node
            .iter_mut()
            .for_each(|n| update_index_dfs(n.get_mut(), index)),
    };
}

fn update_index(tree: &mut Tree<SnailfishNumber>) {
    let mut index: usize = 0;
    update_index_dfs(tree.root_mut().get_mut(), &mut index);
}

fn parse_number(s: &str) -> Tree<SnailfishNumber> {
    if s.chars().all(|c| c.is_digit(10)) {
        // number literal
        return Tree::new(SnailfishNumber {
            node_type: SnailfishNumberType::Value,
            index: 0,
            value: s.parse().unwrap(),
        });
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

        let node1 = parse_number(num1);
        let node2 = parse_number(num2);

        let mut tree = Tree::new(SnailfishNumber {
            node_type: SnailfishNumberType::Pair,
            index: 0,
            value: 0,
        });
        tree.push_back(node1);
        tree.push_back(node2);

        return tree;
    }
}

fn explode(node: &mut Node<SnailfishNumber>) {
    let node1 = node.pop_front().unwrap();
    let node2 = node.pop_front().unwrap();

    if node1.data().index > 0 {
        println!("{}", node1.data().index);
        if let Some(left_number) = find_index(node, node1.data().index - 1) {
            left_number.data_mut().value += node1.data().value;
        }
    }

    if let Some(right_number) = find_index(node, node2.data().index + 1) {
        right_number.data_mut().value += node2.data().value;
    }

    let parent: &mut Node<SnailfishNumber> = unsafe { to_mut(node.parent().unwrap()) };
    let (index, _n) = parent
        .iter()
        .enumerate()
        .find(|(_i, n)| std::ptr::eq(*n, node))
        .unwrap();

    let new_tree = Tree::new(SnailfishNumber {
        node_type: SnailfishNumberType::Value,
        index: 0,
        value: 0,
    });

    if index == 0 {
        parent.push_front(new_tree);
    } else {
        parent.push_back(new_tree);
    }

    node.detach();
}

fn reduce_dfs_explode(node: &mut Node<SnailfishNumber>, level: usize) -> bool {
    match node.data().node_type {
        SnailfishNumberType::Value => false,
        SnailfishNumberType::Pair => {
            if level == 4 {
                explode(node);
                true
            } else {
                node.iter_mut()
                    .any(|n| reduce_dfs_explode(n.get_mut(), level + 1))
            }
        }
    }
}

fn reduce_step(tree: &mut Tree<SnailfishNumber>) -> bool {
    if reduce_dfs_explode(&mut tree.root_mut(), 0) {
        return true;
    }
    return false;
}

fn reduce(tree: &mut Tree<SnailfishNumber>) {
    while reduce_step(tree) {
        update_index(tree);
        println!("{}", tree.to_string());
    }
}

fn part1() {
    let mut tree = parse_number("[[[[[4,3],4],4],[7,[[8,4],9]]],[1,1]]");
    update_index(&mut tree);
    reduce(&mut tree);
    println!("{}", tree.to_string());
}

fn part2(lines: &readfile::Lines) {}

pub fn run() {
    part1();
    // let lines = readfile::Lines::new("day18.txt");
    // part1(&lines);
    // part2(&lines);
}
