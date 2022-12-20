pub fn solve() {
    let data = parse(&std::fs::read_to_string("data/day20.txt").expect("data-file"));

    // part 1
    println!("part 1: {}", part1(&data));

    // part 2
    println!("part 2: {}", part2(&data));
}

fn parse(data: &str) -> Vec<i64> {
    data.lines().map(|l| l.parse().expect("number")).collect()
}

fn part1(data: &[i64]) -> i64 {
    // part 1 naively assumes the data vector is unique, which doesn't cause any trouble
    let mut n = data.to_vec();

    for &x in data {
        // find the index of the value to move
        let i = n.iter().position(|&v| v == x).expect("value");

        // rotate the vector so that the successor is at the head
        n = n
            .iter()
            .skip(i + 1)
            .chain(n.iter().take(i))
            .copied()
            .collect();

        // reinsert the value in the desired position modulo the shortened length
        n.insert(x.rem_euclid(n.len() as i64) as usize, x);
    }

    // compute the result based on the wrapped offsets from zero
    let z = n.iter().position(|&v| v == 0).expect("zero");
    n[(z + 1000) % n.len()] + n[(z + 2000) % n.len()] + n[(z + 3000) % n.len()]
}

fn part2(data: &[i64]) -> i64 {
    // in part 2, we can't assume the data vector is unique,
    // so we must augment it with each value's index for unique lookups
    let data: Vec<_> = data.iter().map(|v| 811589153 * v).enumerate().collect();
    let mut n: Vec<_> = data.clone();

    for _ in 0..10 {
        for x in data.iter().copied() {
            // reinsert the value in the desired position modulo the shortened length
            let i = n.iter().position(|&v| v == x).expect("value");
            n.remove(i);
            n.insert((i as i64 + x.1).rem_euclid(n.len() as i64) as usize, x);
        }
    }

    // compute the result based on the wrapped offsets from zero
    let z = n.iter().position(|&(_i, v)| v == 0).expect("zero");
    n[(z + 1000) % n.len()].1 + n[(z + 2000) % n.len()].1 + n[(z + 3000) % n.len()].1
}

#[cfg(test)]
mod tests {
    use super::*;

    const DATA: &str = "\
        1\n\
        2\n\
        -3\n\
        3\n\
        -2\n\
        0\n\
        4\
    ";

    #[test]
    fn test_part1() {
        assert_eq!(part1(&parse(DATA)), 3);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&parse(DATA)), 1623178306);
    }
}
