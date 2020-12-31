use std::cmp;

const BOSS_HP: i32 = 103;
const BOSS_DP: i32 = 9;
const BOSS_AP: i32 = 2;
const PLAY_HP: i32 = 100;

const WEAPONS: &[(i32, i32, i32)] = &[(8, 4, 0), (10, 5, 0), (25, 6, 0), (40, 7, 0), (74, 8, 0)];

const ARMOR: &[(i32, i32, i32)] = &[
    (0, 0, 0),
    (13, 0, 1),
    (31, 0, 2),
    (53, 0, 3),
    (75, 0, 4),
    (102, 0, 5),
];

const RINGS: &[(i32, i32, i32)] = &[
    (0, 0, 0),
    (25, 1, 0),
    (50, 2, 0),
    (100, 3, 0),
    (20, 0, 1),
    (40, 0, 2),
    (80, 0, 3),
];

pub fn solve() {
    let mut result1 = i32::MAX;
    let mut result2 = 0;
    for &(wc, wd, wa) in WEAPONS {
        for &(ac, ad, aa) in ARMOR {
            for &(rc1, rd1, ra1) in RINGS {
                for &(rc2, rd2, ra2) in RINGS {
                    if (rc1, rd1, ra1) != (rc2, rd2, ra2) || rc1 == 0 {
                        let cost = wc + ac + rc1 + rc2;
                        let play_dp = wd + ad + rd1 + rd2;
                        let play_ap = wa + aa + ra1 + ra2;
                        let mut play_hp = PLAY_HP;
                        let mut boss_hp = BOSS_HP;
                        loop {
                            boss_hp -= cmp::max(play_dp - BOSS_AP, 1);
                            if boss_hp <= 0 {
                                break;
                            }
                            play_hp -= cmp::max(BOSS_DP - play_ap, 1);
                            if play_hp <= 0 {
                                break;
                            }
                        }
                        if play_hp > 0 {
                            result1 = cmp::min(result1, cost);
                        } else {
                            result2 = cmp::max(result2, cost);
                        }
                    }
                }
            }
        }
    }

    // part 1
    println!("part 1: {}", result1);

    // part 2
    println!("part 1: {}", result2);
}
