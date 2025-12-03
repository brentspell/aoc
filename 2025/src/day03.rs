pub fn solve() {
    let data = std::fs::read_to_string("data/day03.txt").unwrap();
    let lines: Vec<&str> = data.lines().collect();

    // part 1
    println!("part 1: {}", part1(&lines));

    // part 2
    println!("part 2: {}", part2(&lines));
}

fn part1(lines: &[&str]) -> u64 {
    let mut total = 0;
    for line in lines {
        // parse the joltage bank
        let bank: Vec<_> = line
            .chars()
            .map(|c| c.to_digit(10).unwrap() as u64)
            .collect();

        // find the indices of the two largest ordered digits
        // digit 0 is the largest digit before the final digit
        // digit 1 is the largest digit after digit 0
        // find the index of the first maximum value
        let len = bank.len();
        let d0 = (0..len - 1).max_by_key(|&i| (bank[i], len - i)).unwrap();
        let d1 = (d0 + 1..len).max_by_key(|&i| (bank[i], len - i)).unwrap();

        // calculate the joltage of the bank
        total += bank[d0] * 10 + bank[d1];
    }

    total
}

fn part2(lines: &[&str]) -> u64 {
    const DIGITS: usize = 12;

    let mut total = 0;
    for line in lines {
        // parse the joltage bank
        let bank: Vec<_> = line
            .chars()
            .map(|c| c.to_digit(10).unwrap() as u64)
            .collect();

        // find the indices of the largest ordered digits
        // digit i is the largest digit after the preivous digit before the i + 1 final digits
        let mut indices = [0; DIGITS];
        for i in 0..DIGITS {
            let beg = if i > 0 { indices[i - 1] + 1 } else { 0 };
            let end = bank.len() - (DIGITS - i - 1);

            // find the index of the first maximum value
            indices[i] = (beg..end).max_by_key(|&j| (bank[j], end - j)).unwrap();
        }

        // calculate the joltage of the bank
        let mut jolts = 0;
        for i in indices {
            jolts = jolts * 10 + bank[i];
        }
        total += jolts;
    }

    total
}

#[cfg(test)]
mod tests {
    use super::*;

    const DATA: &[&str] = &[
        "987654321111111",
        "811111111111119",
        "234234234234278",
        "818181911112111",
    ];

    #[test]
    fn test_part1() {
        assert_eq!(part1(DATA), 357);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(DATA), 3121910778619);
    }
}
