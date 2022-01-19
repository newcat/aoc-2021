#[derive(Clone, Copy)]
struct Player {
    position: u8,
    score: usize,
}

impl Player {
    pub fn new(position: u8) -> Player {
        return Player {
            position,
            score: 0,
        };
    }

    pub fn move_ahead(&mut self, positions: usize) {
        self.position = ((self.position as usize + positions) % 10) as u8;
        self.score += (self.position + 1) as usize;
    }
}

struct Dice {
    sides: usize,
    value: usize,
    rolled: usize,
}

impl Dice {
    pub fn new(sides: usize) -> Dice {
        return Dice {
            sides,
            value: 0,
            rolled: 0,
        };
    }

    pub fn roll(&mut self) -> usize {
        let v = self.value + 1;
        self.value = (self.value + 1) % self.sides;
        self.rolled += 1;
        return v;
    }
}

fn part1() {
    let player1 = Player::new(6);
    let player2 = Player::new(4);
    let mut players = [player1, player2];

    let mut dice = Dice::new(100);

    let mut current_player_index = 0;
    loop {
        let player = &mut players[current_player_index];
        let dice_value = dice.roll() + dice.roll() + dice.roll();
        player.move_ahead(dice_value);
        if current_player_index == 0 {
            current_player_index = 1;
        } else {
            current_player_index = 0
        }

        if player.score >= 1000 {
            break;
        }
    }

    let loosing_player = &players[current_player_index];
    println!("Part 1: {}", loosing_player.score * dice.rolled);
}

fn get_factor(dice_sum: usize) -> usize {
    match dice_sum {
        3 => 1,
        4 => 3,
        5 => 6,
        6 => 7,
        7 => 6,
        8 => 3,
        9 => 1,
        _ => panic!("Invalid dice sum"),
    }
}

fn simulate_single(mut active_player: Player, other_player: Player, dice_sum: usize) -> [usize; 2] {
    let mut active_player_wins = 0;
    let mut other_player_wins = 0;

    active_player.move_ahead(dice_sum);
    let factor = get_factor(dice_sum);
    if active_player.score >= 21 {
        active_player_wins += factor;
    } else {
        let wins = simulate(other_player, active_player);
        let factor = get_factor(dice_sum);
        active_player_wins += factor * wins[1];
        other_player_wins += factor * wins[0];
    }

    return [active_player_wins, other_player_wins];
}

fn simulate(active_player: Player, other_player: Player) -> [usize; 2] {
    let mut active_player_wins = 0;
    let mut other_player_wins = 0;

    for dice_sum in 3..=9 {
        let wins = simulate_single(active_player, other_player, dice_sum);
        active_player_wins += wins[0];
        other_player_wins += wins[1];
    }

    return [active_player_wins, other_player_wins];
}

fn part2() {
    let player1 = Player::new(6);
    let player2 = Player::new(4);
    let [p1_wins, p2_wins] = simulate(player1, player2);
    println!("Part 2: {}", p1_wins.max(p2_wins));
}

pub fn run() {
    part1();
    part2();
}
