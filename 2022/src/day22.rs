use std::collections::HashMap;

// puzzle configuration, these values were found with paper and tape
const W: usize = 50;
const FACE_COORDS: &[(usize, usize)] = &[
    (3 * W, 0),
    (2 * W, 0),
    (2 * W, W),
    (W, W),
    (0, W),
    (0, 2 * W),
];

// face transitions (one per face) define what to do when running over a face's edge
// the map key is the direction in the source face, and the values are the next
// face, the linear projection of u/v coordinates, and the next direction
type FaceTransition = (usize, (i32, i32, i32), (i32, i32, i32), char);

lazy_static! {
    static ref FACE_TRANSITIONS: Vec<HashMap<char, FaceTransition>> = vec![
        HashMap::from([
            ('^', (1, (0, 0, W as i32 - 1), (0, 1, 0), '^')),
            ('>', (2, (0, 0, W as i32 - 1), (1, 0, 0), '^')),
            ('v', (5, (0, 0, 0), (0, 1, 0), 'v')),
            ('<', (4, (0, 0, 0), (1, 0, 0), 'v')),
        ]),
        HashMap::from([
            ('^', (3, (0, 1, 0), (0, 0, 0), '>')),
            ('>', (2, (1, 0, 0), (0, 0, 0), '>')),
            ('v', (0, (0, 0, 0), (0, 1, 0), 'v')),
            ('<', (4, (-1, 0, W as i32 - 1), (0, 0, 0), '>')),
        ]),
        HashMap::from([
            ('^', (3, (0, 0, W as i32 - 1), (0, 1, 0), '^')),
            ('>', (5, (-1, 0, W as i32 - 1), (0, 0, W as i32 - 1), '<')),
            ('v', (0, (0, 1, 0), (0, 0, W as i32 - 1), '<')),
            ('<', (1, (1, 0, 0), (0, 0, W as i32 - 1), '<')),
        ]),
        HashMap::from([
            ('^', (4, (0, 0, W as i32 - 1), (0, 1, 0), '^')),
            ('>', (5, (0, 0, W as i32 - 1), (1, 0, 0), '^')),
            ('v', (2, (0, 0, 0), (0, 1, 0), 'v')),
            ('<', (1, (0, 0, 0), (1, 0, 0), 'v')),
        ]),
        HashMap::from([
            ('^', (0, (0, 1, 0), (0, 0, 0), '>')),
            ('>', (5, (1, 0, 0), (0, 0, 0), '>')),
            ('v', (3, (0, 0, 0), (0, 1, 0), 'v')),
            ('<', (1, (-1, 0, W as i32 - 1), (0, 0, 0), '>')),
        ]),
        HashMap::from([
            ('^', (0, (0, 0, W as i32 - 1), (0, 1, 0), '^')),
            ('>', (2, (-1, 0, W as i32 - 1), (0, 0, W as i32 - 1), '<')),
            ('v', (3, (0, 1, 0), (0, 0, W as i32 - 1), '<')),
            ('<', (4, (1, 0, 0), (0, 0, W as i32 - 1), '<')),
        ]),
    ];
}

pub fn solve() {
    let (map, steps, turns) = parse(&std::fs::read_to_string("data/day22.txt").expect("data-file"));

    // part 1
    println!("part 1: {}", part1(&map, &steps, &turns));

    // part 2
    println!("part 2: {}", part2(&map, &steps, &turns));
}

fn parse(data: &str) -> (Vec<Vec<char>>, Vec<u32>, Vec<char>) {
    // parse the puzzle map
    let map: Vec<Vec<_>> = data
        .lines()
        .take_while(|l| !l.is_empty())
        .map(|l| l.chars().collect())
        .collect();

    // parse the steps and direction changes
    let last = data.lines().last().expect("lines");
    let steps: Vec<u32> = last
        .split(char::is_alphabetic)
        .map(|s| s.parse().expect("number"))
        .collect();
    let turns: Vec<char> = last
        .split(char::is_numeric)
        .flat_map(|s| s.chars())
        .chain(['S'])
        .collect();

    (map, steps, turns)
}

fn part1(map: &[Vec<char>], steps: &[u32], turns: &[char]) -> u32 {
    let mut i = 0;
    let mut j = map[0].iter().position(|&c| c == '.').expect("start-dot");
    let mut d = '>';

    // walk over the map up to the number of steps indicated
    for (s, t) in steps.iter().copied().zip(turns.iter().copied()) {
        for _ in 0..s {
            // attempt to make a step transition
            // just walk through any spaces on the map to wrap around
            let (mut p, mut q) = (i, j);
            match d {
                '^' => loop {
                    p = if p == 0 { map.len() - 1 } else { p - 1 };
                    if q < map[p].len() && map[p][q] != ' ' {
                        break;
                    }
                },
                '>' => loop {
                    q = (q + 1) % map[p].len();
                    if q < map[p].len() && map[p][q] != ' ' {
                        break;
                    }
                },
                'v' => loop {
                    p = (p + 1) % map.len();
                    if q < map[p].len() && map[p][q] != ' ' {
                        break;
                    }
                },
                '<' => loop {
                    q = if q == 0 { map[p].len() - 1 } else { q - 1 };
                    if q < map[p].len() && map[p][q] != ' ' {
                        break;
                    }
                },
                _ => panic!("invalid-dir"),
            }

            // stop whenever we hit a #
            if map[p][q] == '#' {
                break;
            }

            // if we were able to step successfully, commit it to i/j
            (i, j) = (p, q);
        }

        // compute the new direction based on the current turn
        d = map_turn(d, t);
    }

    // compute the score from the current position/orientation
    1000 * (i as u32 + 1) + 4 * (j as u32 + 1) + map_score(d)
}

fn part2(map: &[Vec<char>], steps: &[u32], turns: &[char]) -> u32 {
    // slice all of the individual faces out of the map
    // all steps will be performed using u/v coordinates
    // relative to the current face
    let faces: Vec<Vec<Vec<char>>> = FACE_COORDS
        .iter()
        .copied()
        .map(|(i, j)| map[i..i + W].iter().map(|r| r[j..j + W].to_vec()).collect())
        .collect();

    // problem state is the current face, relative u/v coordinates, and direction
    let (mut f, mut u, mut v, mut d) = (4, 0, 0, '>');

    // walk over the map up to the number of steps indicated
    for (s, t) in steps.iter().copied().zip(turns.iter().copied()) {
        for _ in 0..s {
            // attempt to make a step transition
            let (mut g, mut p, mut q, mut e) = (f, u as i32, v as i32, d);
            match e {
                '^' => p -= 1,
                '>' => q += 1,
                'v' => p += 1,
                '<' => q -= 1,
                _ => panic!("invalid-dir"),
            }

            // if we walked off the face, find the next face from the transition mapping
            if !(0 <= p && p < W as i32 && 0 <= q && q < W as i32) {
                let (gn, (pu, pv, pc), (qu, qv, qc), en) = FACE_TRANSITIONS[g][&e];
                p = pu * u as i32 + pv * v as i32 + pc;
                q = qu * u as i32 + qv * v as i32 + qc;
                g = gn;
                e = en;
            }

            // stop whenever we hit a #
            let (p, q) = (p as usize, q as usize);
            if faces[g][p][q] == '#' {
                break;
            }

            // if we were able to step successfully, commit it to the current state
            (f, u, v, d) = (g, p, q, e);
        }

        // compute the new direction based on the current turn
        d = map_turn(d, t);
    }

    // map the current face and u/v coordinates to original map coordinates
    let (mut i, mut j) = FACE_COORDS[f];
    i += u;
    j += v;

    // compute the score from the current position/orientation
    1000 * (i as u32 + 1) + 4 * (j as u32 + 1) + map_score(d)
}

fn map_turn(dir: char, turn: char) -> char {
    match (dir, turn) {
        ('^', 'L') => '<',
        ('^', 'R') => '>',
        ('v', 'L') => '>',
        ('v', 'R') => '<',
        ('<', 'L') => 'v',
        ('<', 'R') => '^',
        ('>', 'L') => '^',
        ('>', 'R') => 'v',
        (_, 'S') => dir,
        _ => panic!("invalid-turn"),
    }
}

fn map_score(dir: char) -> u32 {
    match dir {
        '>' => 0,
        'v' => 1,
        '<' => 2,
        '^' => 3,
        _ => panic!("invalid-dir"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const DATA: &str = "\
        ...#
        .#..
        #...
        ....
...#.......#
........#...
..#....#....
..........#.
        ...#....
        .....#..
        .#......
        ......#.

10R5L5R10L4R5L5
";

    #[test]
    fn test_part1() {
        let (map, steps, turns) = parse(DATA);
        assert_eq!(part1(&map, &steps, &turns), 6032);
    }
}
