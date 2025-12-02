pub fn solve() {
    let data = std::fs::read_to_string("data/day02.txt").unwrap();

    // part 1
    println!("part 1: {}", part1(&data));

    // part 2
    println!("part 2: {}", part2(&data));
}

fn part1(data: &str) -> u64 {
    parse(data)
        .iter()
        .map(|&(beg, end)| {
            (beg..=end)
                .map(|id| {
                    let s = id.to_string();
                    let n = s.len();

                    // if the first half of the id matches the second, the id is invalid
                    let invalid = s[0..n / 2] == s[n / 2..];

                    if invalid { id } else { 0 }
                })
                .sum::<u64>()
        })
        .sum()
}

fn part2(data: &str) -> u64 {
    parse(data)
        .iter()
        .map(|&(beg, end)| {
            (beg..=end)
                .map(|id| {
                    let s: Vec<char> = id.to_string().chars().collect();
                    let n = s.len();

                    // check all repeat lengths (i)
                    // make sure the string length is a multiple of the repeat length
                    // check all repeats of that length (j)
                    // check all indexes within the current repeat (k)
                    let invalid = (1..=n / 2).any(|i| {
                        n.is_multiple_of(i)
                            && (i..n)
                                .step_by(i)
                                .flat_map(|j| (0..i).map(move |k| (j, k)))
                                .all(|(j, k)| s[j + k] == s[k])
                    });

                    if invalid { id } else { 0 }
                })
                .sum::<u64>()
        })
        .sum()
}

fn parse(data: &str) -> Vec<(u64, u64)> {
    data.split(',')
        .map(|range| {
            let (beg, end) = range.split_once('-').unwrap();
            let beg: u64 = beg.parse().unwrap();
            let end: u64 = end.parse().unwrap();
            (beg, end)
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    const DATA: &str = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";

    #[test]
    fn test_part1() {
        assert_eq!(part1(DATA), 1227775554);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(DATA), 4174379265);
    }
}
