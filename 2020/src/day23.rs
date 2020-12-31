pub fn solve() {
    let mut data = vec![7, 1, 6, 8, 9, 2, 5, 4, 3];
    println!("part 1: {}", part1(&data, 100));

    for i in 10..=1_000_000 as usize {
        data.push(i);
    }
    println!("part 2: {}", part2(&data));
}

fn part1(cups: &[usize], rounds: usize) -> u64 {
    let next = simulate(cups, rounds);

    let mut result = 0;
    let mut current = next[0];
    for _ in 0..next.len() - 1 {
        result *= 10;
        result += current as u64 + 1;
        current = next[current];
    }

    result
}

fn part2(cups: &[usize]) -> u64 {
    let next = simulate(cups, 10_000_000);
    (next[0] as u64 + 1) * (next[next[0]] as u64 + 1)
}

fn simulate(cups: &[usize], rounds: usize) -> Vec<usize> {
    let mut next: Vec<usize> = vec![0_usize; cups.len()];
    for i in 0..cups.len() {
        next[cups[i] - 1] = cups[(i + 1) % cups.len()] - 1
    }

    let mut current = cups[0] - 1;
    for _ in 0..rounds {
        let next1 = next[current];
        let next2 = next[next1];
        let next3 = next[next2];

        let mut dest = current;
        loop {
            dest = if dest == 0 { next.len() - 1 } else { dest - 1 };
            if !(dest == next1 || dest == next2 || dest == next3) {
                break;
            }
        }

        next[current] = next[next3];
        next[next3] = next[dest];
        next[dest] = next1;

        current = next[current];
    }

    next
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(part1(&[3, 8, 9, 1, 2, 5, 4, 6, 7], 10), 92658374);
        assert_eq!(part1(&[3, 8, 9, 1, 2, 5, 4, 6, 7], 100), 67384529);
    }
}
