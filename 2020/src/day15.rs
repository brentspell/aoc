pub fn solve() {
    let input = [16, 1, 0, 18, 12, 14, 19];

    println!("part 1: {}", simulate(&input, 2020));
    println!("part 2: {}", simulate(&input, 30000000));
}

fn simulate(input: &[usize], count: usize) -> usize {
    let mut seen: Vec<usize> = vec![0; count];
    for (i, &x) in input[..input.len() - 1].iter().enumerate() {
        seen[x] = i + 1;
    }

    let mut last = input[input.len() - 1];
    for i in input.len()..count {
        let next = if seen[last] == 0 { 0 } else { i - seen[last] };
        seen[last] = i;
        last = next;
    }

    last
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let input = [0, 3, 6];

        assert_eq!(simulate(&input, 2020), 436);
    }
}
