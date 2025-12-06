pub fn solve() {
    let data = std::fs::read_to_string("data/day06.txt").unwrap();
    let lines: Vec<&str> = data.lines().collect();

    // part 1
    println!("part 1: {}", part1(&lines));

    // part 2
    println!("part 2: {}", part2(&lines));
}

fn part1(lines: &[&str]) -> u64 {
    // extract the operand matrix (operands on rows, expressions on columns)
    let operands = lines[..lines.len() - 1]
        .iter()
        .map(|l| {
            l.split_whitespace()
                .map(|s| s.parse::<u64>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    // extract the list of operators, one per expression
    let operators = lines[lines.len() - 1]
        .split_whitespace()
        .collect::<Vec<_>>();

    // sum the results over all expression columns
    (0..operands[0].len())
        .map(|j| {
            let o = (0..operands.len()).map(|i| operands[i][j]);
            match operators[j] {
                "+" => o.sum::<u64>(),
                "*" => o.product::<u64>(),
                _ => panic!("invalid operator"),
            }
        })
        .sum()
}

fn part2(lines: &[&str]) -> u64 {
    // first, pad all lines so they are the same length
    let maxlen = lines.iter().map(|l| l.len()).max().unwrap() + 1;
    let lines = lines
        .iter()
        .map(|l| format!("{: <maxlen$}", l))
        .collect::<Vec<_>>();

    // find the length of each expression operand, including any whitespace padding
    // this is done by splitting the operator string on the operator characters
    let lengths = regex::Regex::new(r"[+*]\s*")
        .unwrap()
        .find_iter(&lines[lines.len() - 1])
        .map(|s| s.as_str().len())
        .collect::<Vec<_>>();

    // extract the operand row strings and the list of operators
    let operands = lines[..lines.len() - 1]
        .iter()
        .map(|l| l.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let operators = lines[lines.len() - 1]
        .split_whitespace()
        .collect::<Vec<_>>();

    let mut total = 0;
    let mut offset = 0;
    for (&operator, &length) in operators.iter().zip(lengths.iter()) {
        // extract the operands for the current expression
        // this is done by iterating columns from right to left and trimming spaces
        let operands = (offset..offset + length - 1).rev().map(|k| {
            (0..operands.len())
                .map(|i| operands[i][k])
                .collect::<String>()
                .trim()
                .parse::<u64>()
                .unwrap()
        });

        // apply the operator to the operands
        match operator {
            "+" => total += operands.sum::<u64>(),
            "*" => total += operands.product::<u64>(),
            _ => panic!("invalid operator"),
        }

        // update the running offset into the next expression column
        offset += length;
    }

    total
}

#[cfg(test)]
mod tests {
    use super::*;

    const DATA: &[&str] = &[
        "123 328  51 64 ",
        " 45 64  387 23 ",
        "  6 98  215 314",
        "*   +   *   +  ",
    ];

    #[test]
    fn test_part1() {
        assert_eq!(part1(DATA), 4277556);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(DATA), 3263827);
    }
}
