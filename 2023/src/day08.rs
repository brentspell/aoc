use std::collections::HashMap;

pub fn solve() {
    let data = std::fs::read_to_string("data/day08.txt").unwrap();

    // part 1
    println!("part 1: {}", part1(&data));

    // part 2
    println!("part 2: {}", part2(&data));
}

fn part1(data: &str) -> u64 {
    let (moves, net) = parse(data);

    // straightforward walk through the graph, counting the number of steps
    let mut node = "AAA";
    let mut n = 0;
    for m in moves.chars().cycle() {
        let (l, r) = net[node];
        node = if m == 'L' { l } else { r };
        n += 1;
        if node == "ZZZ" {
            break;
        }
    }

    n
}

fn part2(data: &str) -> u64 {
    let (moves, net) = parse(data);

    // since there is only one path through the graph for each source->destination,
    // but that those paths may differ across all of the starting nodes, the total
    // count will be the least common multiple of the length of each path
    let mut n = 1;
    for mut node in net.keys().filter(|k| &k[2..3] == "A").copied() {
        // find the length of the current path
        let mut c = 0;
        for m in moves.chars().cycle() {
            let (l, r) = net[node];
            node = if m == 'L' { l } else { r };
            c += 1;
            if &node[2..3] == "Z" {
                break;
            }
        }

        // calculate the shortest common cycle length
        n = lcm(n, c);
    }

    n
}

fn parse(data: &str) -> (&str, HashMap<&str, (&str, &str)>) {
    let lines: Vec<_> = data.lines().collect();
    let moves = lines[0];
    let net: HashMap<_, _> = lines
        .iter()
        .skip(2)
        .map(|line| (&line[0..3], (&line[7..10], &line[12..15])))
        .collect();
    (moves, net)
}

fn gcd(a: u64, b: u64) -> u64 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

fn lcm(a: u64, b: u64) -> u64 {
    a * b / gcd(a, b)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        const DATA: &str = "\
            RL\n\
            \n\
            AAA = (BBB, CCC)\n\
            BBB = (DDD, EEE)\n\
            CCC = (ZZZ, GGG)\n\
            DDD = (DDD, DDD)\n\
            EEE = (EEE, EEE)\n\
            GGG = (GGG, GGG)\n\
            ZZZ = (ZZZ, ZZZ)\n\
        ";
        assert_eq!(part1(DATA), 2);
    }

    #[test]
    fn test_part2() {
        const DATA: &str = "\
            LR\n\
            \n\
            11A = (11B, XXX)\n\
            11B = (XXX, 11Z)\n\
            11Z = (11B, XXX)\n\
            22A = (22B, XXX)\n\
            22B = (22C, 22C)\n\
            22C = (22Z, 22Z)\n\
            22Z = (22B, 22B)\n\
            XXX = (XXX, XXX)\n\
        ";
        assert_eq!(part2(DATA), 6);
    }
}
