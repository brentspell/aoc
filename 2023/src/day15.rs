const HASH_MOD: u64 = 256;

pub fn solve() {
    let data = std::fs::read_to_string("data/day15.txt").unwrap();

    // part 1
    println!("part 1: {}", part1(&data));

    // part 2
    println!("part 2: {}", part2(&data));
}

fn part1(data: &str) -> u64 {
    data.trim().split(',').map(hash).sum()
}

fn part2(data: &str) -> u64 {
    let mut boxes: Vec<_> = vec![Vec::new(); HASH_MOD as usize];
    for instr in data.trim().split(',') {
        if let Some(n) = instr.strip_suffix('-') {
            // for removal instructions, search for the lens by name and remove it if found
            let b = &mut boxes[hash(n) as usize];
            if let Some(pos) = b.iter().position(|(x, _)| x == n) {
                b.remove(pos);
            }
        } else {
            // parse the assignment instruction
            let s: Vec<_> = instr.split('=').collect();
            let n = s[0];
            let f: u64 = s[1].parse().unwrap();

            // for assignments, replace the lense if found, otherwise push
            let b = &mut boxes[hash(n) as usize];
            if let Some(pos) = b.iter().position(|(x, _)| x == n) {
                b[pos] = (n.to_string(), f);
            } else {
                b.push((n.to_string(), f));
            }
        }
    }

    // calculate the scores of each box's slots and accumulate
    boxes
        .iter()
        .enumerate()
        .flat_map(|(i, b)| {
            b.iter()
                .enumerate()
                .map(move |(j, (_, f))| (i as u64 + 1) * (j as u64 + 1) * f)
        })
        .sum()
}

fn hash(data: &str) -> u64 {
    data.chars().fold(0, |a, c| (a + c as u64) * 17 % HASH_MOD)
}

#[cfg(test)]
mod tests {
    use super::*;

    const DATA: &str = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";

    #[test]
    fn test_part1() {
        assert_eq!(part1(DATA), 1320);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(DATA), 145);
    }
}
