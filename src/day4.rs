use crate::readfile;

struct Board {
    nums: [[u8; 5]; 5],
    marked: [[bool; 5]; 5],
}

impl Board {
    fn new(lines: &[&str], range: std::ops::Range<usize>) -> Board {
        let mut nums: [[u8; 5]; 5] = [[0; 5]; 5];
        for (local_index, i) in range.enumerate() {
            let values: [u8; 5] = lines[i]
                .split_whitespace()
                .map(|v| v.parse::<u8>().unwrap())
                .collect::<Vec<u8>>()
                .try_into()
                .unwrap();
            nums[local_index] = values;
        }
        return Board {
            nums,
            marked: [[false; 5]; 5],
        };
    }

    fn mark_value(self: &mut Board, value: u8) {
        for x in 0..5 {
            for y in 0..5 {
                if self.nums[y][x] == value {
                    self.marked[y][x] = true;
                }
            }
        }
    }

    fn has_won(self: &Board) -> bool {
        // check columns
        for x in 0..5 {
            let mut v = true;
            for y in 0..5 {
                v = v && self.marked[y][x];
            }
            if v {
                return true;
            };
        }

        // check rows
        for y in 0..5 {
            let mut v = true;
            for x in 0..5 {
                v = v && self.marked[y][x];
            }
            if v {
                return true;
            };
        }

        false
    }
}

struct Game {
    values: Vec<u8>,
    boards: Vec<Board>,
}

impl Game {
    fn new(lines: &readfile::Lines) -> Game {
        let l: Vec<&str> = lines.lines().collect();
        let values = l[0].split(',').map(|v| v.parse::<u8>().unwrap()).collect();
        let mut game = Game {
            boards: Vec::new(),
            values,
        };

        let mut index = 1;
        while index < l.len() {
            index += 1;
            game.boards.push(Board::new(&l, index..index + 5));
            index += 5;
        }

        game
    }
}

fn get_score(board: &Board, value: u8) -> u32 {
    let mut sum: u32 = 0;
    for x in 0..5 {
        for y in 0..5 {
            if !board.marked[y][x] {
                sum += u32::from(board.nums[y][x]);
            }
        }
    }
    return sum * u32::from(value);
}

fn part1(lines: &readfile::Lines) {
    let mut game = Game::new(lines);
    for value in game.values {
        for board in &mut game.boards {
            board.mark_value(value);
            if board.has_won() {
                println!("Part 1: {}", get_score(board, value));
                return;
            }
        }
    }
}

fn part2(lines: &readfile::Lines) {
    let mut game = Game::new(lines);
    let total_boards = game.boards.len();
    let mut won_board_indices: Vec<usize> = Vec::new();
    for value in game.values {
        for (i, board) in game.boards.iter_mut().enumerate() {
            if won_board_indices.iter().any(|bi| *bi == i) {
                continue;
            }
            board.mark_value(value);
            if board.has_won() {
                won_board_indices.push(i);
                if won_board_indices.len() == total_boards {
                    println!("Part 2: {}", get_score(board, value));
                    return;
                }
            }
        }
    }
}

pub fn run() {
    let lines = readfile::Lines::new("day4.txt");
    part1(&lines);
    part2(&lines);
}
