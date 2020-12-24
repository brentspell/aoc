use std::collections::HashSet;
use std::io::prelude::*;
use std::{fs, io};

pub fn solve() {
    let file = fs::File::open("data/day24.txt").unwrap();
    let lines: Vec<Vec<char>> = io::BufReader::new(file)
        .lines()
        .map(|l| l.unwrap().chars().collect())
        .collect();

    println!("part 1: {}", part1(&lines));
    println!("part 2: {}", part2(&lines));
}

fn part1(paths: &[Vec<char>]) -> usize {
    walk(paths).len()
}

fn part2(paths: &[Vec<char>]) -> usize {
    let mut black = walk(paths);
    for _ in 0..100 {
        let old = black.clone();
        let mut todo: Vec<(i32, i32, i32)> = old.iter().copied().collect();
        let mut done: HashSet<(i32, i32, i32)> = old.clone();
        while !todo.is_empty() {
            let tile = todo.pop().unwrap();
            let was_black = old.contains(&tile);

            // enumerate immediate neighbors, queueing unique work and counting black neighbors
            let mut neighbors = 0;
            for dx in -1..=1 {
                for dy in -1..=1 {
                    for dz in -1..=1 {
                        let neighbor = (tile.0 + dx, tile.1 + dy, tile.2 + dz);
                        if neighbor != tile && dx + dy + dz == 0 {
                            neighbors += old.contains(&neighbor) as u32;
                            if was_black && done.insert(neighbor) {
                                todo.push(neighbor);
                            }
                        }
                    }
                }
            }

            // apply the cellular automata rules
            if was_black && (neighbors == 0 || neighbors > 2) {
                black.remove(&tile);
            }
            if !was_black && neighbors == 2 {
                black.insert(tile);
            }
        }
    }

    black.len()
}

fn walk(paths: &[Vec<char>]) -> HashSet<(i32, i32, i32)> {
    // map each hexagonal position to a cubic coordinate
    // https://www.redblobgames.com/grids/hexagons/#coordinates
    let mut black: HashSet<(i32, i32, i32)> = HashSet::new();
    for path in paths {
        let mut i = 0;
        let mut x = 0;
        let mut y = 0;
        let mut z = 0;
        while i < path.len() {
            match path[i..].iter().take(2).collect::<String>().as_str() {
                "ne" => {
                    x += 1;
                    z -= 1;
                    i += 2;
                }
                "se" => {
                    y -= 1;
                    z += 1;
                    i += 2;
                }
                "sw" => {
                    x -= 1;
                    z += 1;
                    i += 2;
                }
                "nw" => {
                    y += 1;
                    z -= 1;
                    i += 2;
                }
                _ => match path[i] {
                    'e' => {
                        x += 1;
                        y -= 1;
                        i += 1;
                    }
                    'w' => {
                        x -= 1;
                        y += 1;
                        i += 1;
                    }
                    _ => panic!(),
                },
            }
        }
        if !black.insert((x, y, z)) {
            black.remove(&(x, y, z));
        }
    }

    black
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let paths: Vec<Vec<char>> = [
            "sesenwnenenewseeswwswswwnenewsewsw",
            "neeenesenwnwwswnenewnwwsewnenwseswesw",
            "seswneswswsenwwnwse",
            "nwnwneseeswswnenewneswwnewseswneseene",
            "swweswneswnenwsewnwneneseenw",
            "eesenwseswswnenwswnwnwsewwnwsene",
            "sewnenenenesenwsewnenwwwse",
            "wenwwweseeeweswwwnwwe",
            "wsweesenenewnwwnwsenewsenwwsesesenwne",
            "neeswseenwwswnwswswnw",
            "nenwswwsewswnenenewsenwsenwnesesenew",
            "enewnwewneswsewnwswenweswnenwsenwsw",
            "sweneswneswneneenwnewenewwneswswnese",
            "swwesenesewenwneswnwwneseswwne",
            "enesenwswwswneneswsenwnewswseenwsese",
            "wnwnesenesenenwwnenwsewesewsesesew",
            "nenewswnwewswnenesenwnesewesw",
            "eneswnwswnwsenenwnwnwwseeswneewsenese",
            "neswnwewnwnwseenwseesewsenwsweewe",
            "wseweeenwnesenwwwswnew",
        ]
        .iter()
        .map(|l| l.chars().collect())
        .collect();

        assert_eq!(part1(&paths), 10);
        assert_eq!(part2(&paths), 2208);
    }
}
