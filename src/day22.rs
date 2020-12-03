use std::cmp;
use std::collections::VecDeque;

const BOSS_HP: i32 = 55;
const BOSS_DP: i32 = 8;
const PLAY_HP: i32 = 50;
const PLAY_MA: i32 = 500;
const MISSILE_MC: i32 = 53;
const MISSILE_DP: i32 = 4;
const DRAIN_MC: i32 = 73;
const DRAIN_DP: i32 = 2;
const DRAIN_HP: i32 = 2;
const SHIELD_MC: i32 = 113;
const SHIELD_AP: i32 = 7;
const SHIELD_TO: i32 = 6;
const POISON_MC: i32 = 173;
const POISON_DP: i32 = 3;
const POISON_TO: i32 = 6;
const RECHARGE_MC: i32 = 229;
const RECHARGE_MA: i32 = 101;
const RECHARGE_TO: i32 = 5;

#[derive(Clone, Copy, Debug)]
struct Game {
    spent: i32,
    play_hp: i32,
    play_ma: i32,
    boss_hp: i32,
    shield: i32,
    poison: i32,
    recharge: i32,
}

impl Game {
    fn new() -> Game {
        Game {
            spent: 0,
            play_hp: PLAY_HP,
            play_ma: PLAY_MA,
            boss_hp: BOSS_HP,
            shield: 0,
            poison: 0,
            recharge: 0,
        }
    }
}

#[derive(PartialEq)]
enum Difficulty {
    Normal,
    Hard,
}

pub fn solve() {
    // part 1
    let result = simulate(Difficulty::Normal);
    println!("part 1: {}", result);

    // part 2
    let result = simulate(Difficulty::Hard);
    println!("part 1: {}", result);
}

fn simulate(difficulty: Difficulty) -> i32 {
    let mut result = i32::MAX;

    // BFS over game states, with early stopping to minimize total mana spent
    let mut queue: VecDeque<Game> = VecDeque::new();
    queue.push_back(Game::new());
    while !queue.is_empty() {
        let mut game = queue.pop_front().unwrap();
        // boss turn (skip first so player goes first)
        if game.spent > 0 {
            apply_effects(&mut game);
            if game.boss_hp <= 0 {
                result = cmp::min(result, game.spent);
                continue;
            }
            let play_ap = if game.shield > 0 { SHIELD_AP } else { 0 };
            game.play_hp -= cmp::max(BOSS_DP - play_ap, 1);
        }
        // player turn
        if difficulty == Difficulty::Hard {
            game.play_hp -= 1;
        }
        if game.play_hp <= 0 {
            continue;
        }
        apply_effects(&mut game);
        if game.boss_hp <= 0 {
            result = cmp::min(result, game.spent);
            continue;
        }
        // early stopping if we can no longer beat the minimum
        if game.spent > result {
            continue;
        }
        // expand the next game state via player decisions
        if game.play_ma >= MISSILE_MC {
            let mut game = game;
            game.spent += MISSILE_MC;
            game.play_ma -= MISSILE_MC;
            game.boss_hp -= MISSILE_DP;
            if game.boss_hp <= 0 {
                result = cmp::min(result, game.spent);
            } else {
                queue.push_back(game);
            }
        }
        if game.play_ma >= DRAIN_MC {
            let mut game = game;
            game.spent += DRAIN_MC;
            game.play_ma -= DRAIN_MC;
            game.boss_hp -= DRAIN_DP;
            if game.boss_hp <= 0 {
                result = cmp::min(result, game.spent);
            } else {
                game.play_hp += DRAIN_HP;
                queue.push_back(game);
            }
        }
        if game.shield == 0 && game.play_ma >= SHIELD_MC {
            let mut game = game;
            game.spent += SHIELD_MC;
            game.play_ma -= SHIELD_MC;
            game.shield = SHIELD_TO;
            queue.push_back(game);
        }
        if game.poison == 0 && game.play_ma >= POISON_MC {
            let mut game = game;
            game.spent += POISON_MC;
            game.play_ma -= POISON_MC;
            game.poison = POISON_TO;
            queue.push_back(game);
        }
        if game.recharge == 0 && game.play_ma >= RECHARGE_MC {
            let mut game = game;
            game.spent += RECHARGE_MC;
            game.play_ma -= RECHARGE_MC;
            game.recharge = RECHARGE_TO;
            queue.push_back(game);
        }
    }

    result
}

fn apply_effects(game: &mut Game) {
    if game.shield > 0 {
        game.shield -= 1;
    }
    if game.poison > 0 {
        game.boss_hp -= POISON_DP;
        game.poison -= 1;
    }
    if game.recharge > 0 {
        game.play_ma += RECHARGE_MA;
        game.recharge -= 1;
    }
}
