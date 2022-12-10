#[derive(Clone, Copy, Debug)]
enum Op {
    NoOp,
    AddX(i32),
}

pub fn solve() {
    let data = parse(&std::fs::read_to_string("data/day10.txt").expect("data-file"));

    // part 1
    println!("part 1: {}", part1(&data));

    // part 2
    println!("part 2:\n{}", part2(&data));
}

fn parse(data: &str) -> Vec<Op> {
    data.lines()
        .map(|l| {
            if l == "noop" {
                Op::NoOp
            } else if l.starts_with("addx ") {
                let (_, n) = l.split_at(5);
                Op::AddX(n.parse().expect("integer"))
            } else {
                panic!("invalid line: {}", l);
            }
        })
        .collect()
}

fn part1(program: &[Op]) -> u32 {
    let mut c = 1;
    let mut x = 1;
    program
        .iter()
        .map(|op| {
            // calculate the number of cycles per operator
            let n = match op {
                Op::NoOp => 1,
                Op::AddX(_) => 2,
            };

            // spin the number of cycles, accumulating signal strengths
            let mut s = 0;
            for _ in 0..n {
                if let 20 | 60 | 100 | 140 | 180 | 220 = c {
                    s += c * x as u32;
                }
                c += 1;
            }

            // apply the operator at the end of its cycles
            if let Op::AddX(n) = op {
                x += n;
            }

            s
        })
        .sum()
}

fn part2(program: &[Op]) -> String {
    let mut x = 1;
    let mut p = 0;
    let mut c: Vec<char> = Vec::new();
    for op in program {
        // calculate the number of cycles per operator
        let n = match op {
            Op::NoOp => 1,
            Op::AddX(_) => 2,
        };

        // spin the number of cycles, drawing the sprite if it is visible
        for _ in 0..n {
            if (x - 1..=x + 1).contains(&p) {
                c.push('#');
            } else {
                c.push('.');
            }
            p += 1;
            if p == 40 {
                c.push('\n');
                p = 0;
            }
        }

        // apply the operator at the end of its cycles
        if let Op::AddX(n) = op {
            x += n;
        }
    }
    c.iter().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    const DATA: &str = "\
        addx 15\n\
        addx -11\n\
        addx 6\n\
        addx -3\n\
        addx 5\n\
        addx -1\n\
        addx -8\n\
        addx 13\n\
        addx 4\n\
        noop\n\
        addx -1\n\
        addx 5\n\
        addx -1\n\
        addx 5\n\
        addx -1\n\
        addx 5\n\
        addx -1\n\
        addx 5\n\
        addx -1\n\
        addx -35\n\
        addx 1\n\
        addx 24\n\
        addx -19\n\
        addx 1\n\
        addx 16\n\
        addx -11\n\
        noop\n\
        noop\n\
        addx 21\n\
        addx -15\n\
        noop\n\
        noop\n\
        addx -3\n\
        addx 9\n\
        addx 1\n\
        addx -3\n\
        addx 8\n\
        addx 1\n\
        addx 5\n\
        noop\n\
        noop\n\
        noop\n\
        noop\n\
        noop\n\
        addx -36\n\
        noop\n\
        addx 1\n\
        addx 7\n\
        noop\n\
        noop\n\
        noop\n\
        addx 2\n\
        addx 6\n\
        noop\n\
        noop\n\
        noop\n\
        noop\n\
        noop\n\
        addx 1\n\
        noop\n\
        noop\n\
        addx 7\n\
        addx 1\n\
        noop\n\
        addx -13\n\
        addx 13\n\
        addx 7\n\
        noop\n\
        addx 1\n\
        addx -33\n\
        noop\n\
        noop\n\
        noop\n\
        addx 2\n\
        noop\n\
        noop\n\
        noop\n\
        addx 8\n\
        noop\n\
        addx -1\n\
        addx 2\n\
        addx 1\n\
        noop\n\
        addx 17\n\
        addx -9\n\
        addx 1\n\
        addx 1\n\
        addx -3\n\
        addx 11\n\
        noop\n\
        noop\n\
        addx 1\n\
        noop\n\
        addx 1\n\
        noop\n\
        noop\n\
        addx -13\n\
        addx -19\n\
        addx 1\n\
        addx 3\n\
        addx 26\n\
        addx -30\n\
        addx 12\n\
        addx -1\n\
        addx 3\n\
        addx 1\n\
        noop\n\
        noop\n\
        noop\n\
        addx -9\n\
        addx 18\n\
        addx 1\n\
        addx 2\n\
        noop\n\
        noop\n\
        addx 9\n\
        noop\n\
        noop\n\
        noop\n\
        addx -1\n\
        addx 2\n\
        addx -37\n\
        addx 1\n\
        addx 3\n\
        noop\n\
        addx 15\n\
        addx -21\n\
        addx 22\n\
        addx -6\n\
        addx 1\n\
        noop\n\
        addx 2\n\
        addx 1\n\
        noop\n\
        addx -10\n\
        noop\n\
        noop\n\
        addx 20\n\
        addx 1\n\
        addx 2\n\
        addx 2\n\
        addx -6\n\
        addx -11\n\
        noop\n\
        noop\n\
        noop\n\
    ";

    #[test]
    fn test_part1() {
        assert_eq!(part1(&parse(DATA)), 13140);
    }

    #[test]
    fn test_part2() {
        assert_eq!(
            part2(&parse(DATA)),
            "\
                ##..##..##..##..##..##..##..##..##..##..\n\
                ###...###...###...###...###...###...###.\n\
                ####....####....####....####....####....\n\
                #####.....#####.....#####.....#####.....\n\
                ######......######......######......####\n\
                #######.......#######.......#######.....\n\
            "
        );
    }
}
