use std::collections::HashMap;

pub fn solve() {
    let data = std::fs::read_to_string("data/day14.txt").unwrap();

    // part 1
    println!("part 1: {}", part1(&data));

    // part 2
    println!("part 2: {}", part2(&data));
}

fn part1(data: &str) -> u64 {
    let mut plat: Vec<Vec<_>> = data.lines().map(|l| l.chars().collect()).collect();
    tilt_north(&mut plat);
    calculate_load(&plat)
}

fn part2(data: &str) -> u64 {
    let mut plat: Vec<Vec<_>> = data.lines().map(|l| l.chars().collect()).collect();
    let mut seen: HashMap<_, _> = HashMap::new();
    let mut loads: Vec<_> = Vec::new();

    loop {
        // stop when we see the same platform configuration repeat and
        // return the offset within the repeated sequence
        if let Some(start) = seen.get(&plat) {
            return loads[start + (1000000000 - start) % (seen.len() - start)];
        }

        // otherwise, record the offset of the current configuration and its load
        seen.insert(plat.clone(), seen.len());
        loads.push(calculate_load(&plat));

        // tilt through a cycle
        tilt_north(&mut plat);
        tilt_west(&mut plat);
        tilt_south(&mut plat);
        tilt_east(&mut plat);
    }
}

fn tilt_north(p: &mut [Vec<char>]) {
    for i in 1..p.len() {
        for j in 0..p[0].len() {
            if p[i][j] == 'O' {
                for k in (1..=i).rev() {
                    if p[k - 1][j] != '.' {
                        break;
                    }
                    (p[k][j], p[k - 1][j]) = (p[k - 1][j], p[k][j]);
                }
            }
        }
    }
}

fn tilt_west(p: &mut [Vec<char>]) {
    for j in 1..p[0].len() {
        #[allow(clippy::needless_range_loop)]
        for i in 0..p.len() {
            if p[i][j] == 'O' {
                for k in (1..=j).rev() {
                    if p[i][k - 1] != '.' {
                        break;
                    }
                    (p[i][k], p[i][k - 1]) = (p[i][k - 1], p[i][k]);
                }
            }
        }
    }
}

fn tilt_south(p: &mut [Vec<char>]) {
    for i in (0..p.len() - 1).rev() {
        for j in 0..p[0].len() {
            if p[i][j] == 'O' {
                for k in i..p.len() - 1 {
                    if p[k + 1][j] != '.' {
                        break;
                    }
                    (p[k][j], p[k + 1][j]) = (p[k + 1][j], p[k][j]);
                }
            }
        }
    }
}

fn tilt_east(p: &mut [Vec<char>]) {
    for j in (0..p[0].len() - 1).rev() {
        for i in 0..p.len() {
            if p[i][j] == 'O' {
                for k in j..p.len() - 1 {
                    if p[i][k + 1] != '.' {
                        break;
                    }
                    (p[i][k], p[i][k + 1]) = (p[i][k + 1], p[i][k]);
                }
            }
        }
    }
}

fn calculate_load(p: &[Vec<char>]) -> u64 {
    p.iter()
        .enumerate()
        .map(|(i, r)| (r.iter().filter(|&&c| c == 'O').count() * (p.len() - i)) as u64)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const DATA: &str = "\
        O....#....\n\
        O.OO#....#\n\
        .....##...\n\
        OO.#O....O\n\
        .O.....O#.\n\
        O.#..O.#.#\n\
        ..O..#O..O\n\
        .......O..\n\
        #....###..\n\
        #OO..#....\n\
    ";

    #[test]
    fn test_part1() {
        assert_eq!(part1(DATA), 136);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(DATA), 64);
    }
}
