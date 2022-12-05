type Stacks = Vec<Vec<char>>;

#[derive(Debug)]
struct Move {
    count: usize,
    from: usize,
    to: usize,
}

pub fn solve() {
    let data = std::fs::read_to_string("data/day05.txt").unwrap();
    let (stacks, moves) = parse(&data);

    // part 1
    println!("part 1: {}", part1(&stacks, &moves));

    // part 2
    println!("part 2: {}", part2(&stacks, &moves));
}

fn parse(string: &str) -> (Stacks, Vec<Move>) {
    let (layers, moves) = string.split_once("\n\n").unwrap();
    // parse the stacks
    let mut stacks: Stacks = Vec::new();
    for line in layers.lines() {
        for (i, ch) in line.chars().skip(1).step_by(4).enumerate() {
            if ch != ' ' && !('1'..'9').contains(&ch) {
                while i >= stacks.len() {
                    stacks.push(Vec::new());
                }
                stacks[i].push(ch);
            }
        }
    }
    for stack in stacks.iter_mut() {
        stack.reverse();
    }
    // parse the moves
    let moves = moves
        .lines()
        .map(|line| {
            let tokens: Vec<&str> = line.split(' ').collect();
            Move {
                count: tokens[1].parse().unwrap(),
                from: tokens[3].parse().unwrap(),
                to: tokens[5].parse().unwrap(),
            }
        })
        .collect();

    (stacks, moves)
}

fn part1(stacks: &Stacks, moves: &[Move]) -> String {
    let mut stacks = stacks.clone();
    for m in moves {
        for _ in 0..m.count {
            let c = stacks[m.from - 1].pop().unwrap();
            stacks[m.to - 1].push(c);
        }
    }
    stacks.iter().map(|s| s[s.len() - 1]).collect()
}

fn part2(stacks: &Stacks, moves: &[Move]) -> String {
    let mut stacks = stacks.clone();
    for m in moves {
        let i = stacks[m.from - 1].len() - m.count;
        let mut c: Vec<char> = stacks[m.from - 1].drain(i..).collect();
        stacks[m.to - 1].append(&mut c);
    }
    stacks.iter().map(|s| s[s.len() - 1]).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    const DATA: &str = "
    [D]
[N] [C]
[Z] [M] [P]
 1   2   3

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2
";

    #[test]
    fn test_part1() {
        let (stacks, moves) = parse(DATA);
        assert_eq!(stacks.len(), 3);
        assert_eq!(moves.len(), 4);
        assert_eq!(part1(&stacks, &moves), "CMZ");
    }

    #[test]
    fn test_part2() {
        let (stacks, moves) = parse(DATA);
        assert_eq!(part2(&stacks, &moves), "MCD");
    }
}
