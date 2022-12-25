pub fn solve() {
    let data = std::fs::read_to_string("data/day25.txt").expect("data-file");

    // part 1
    println!("part 1: {}", part1(&data));
}

fn part1(data: &str) -> String {
    // convert the snafus to numbers and sum
    let mut s: i64 = 0;
    for line in data.lines() {
        let mut p = 1;
        let mut n = 0;
        for c in line.chars().rev() {
            n += match c {
                '-' => -p,
                '=' => -2 * p,
                _ => c.to_digit(10).expect("digit") as i64 * p,
            };
            p *= 5;
        }

        // accumulate the snafus
        s += n;
    }

    // convert the sum back to snafu
    let mut snafu = vec![];
    while s > 0 {
        let mut r = 0;
        match s % 5 {
            4 => {
                snafu.push('-');
                r = 1;
            }
            3 => {
                snafu.push('=');
                r = 1;
            }
            d => snafu.push(char::from_digit(d as u32, 10).expect("digit")),
        }
        s /= 5;
        s += r;
    }

    snafu.iter().rev().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    const DATA: &str = "\
1=-0-2
12111
2=0=
21
2=01
111
20012
112
1=-1=
1-12
12
1=
122
";

    #[test]
    fn test_part1() {
        assert_eq!(part1(DATA), "2=-1=0");
    }
}
