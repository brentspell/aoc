pub fn solve() {
    let data = std::fs::read_to_string("data/day06.txt").unwrap();

    // part 1
    println!("part 1: {}", marker(&data, 4));

    // part 2
    println!("part 2: {}", marker(&data, 14));
}

fn marker(msg: &str, length: usize) -> usize {
    let s = msg.as_bytes();
    for i in 0..s.len() - length {
        let mut bitset = 0;
        let mut j = 0;
        while j < length {
            let b = 1 << (s[i + j] - b'a');
            if bitset & b != 0 {
                break;
            }
            bitset |= b;
            j += 1;
        }
        if j == length {
            return i + j;
        }
    }
    panic!("marker-not-found");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(marker("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 4), 7);
        assert_eq!(marker("bvwbjplbgvbhsrlpgdmjqwftvncz", 4), 5);
        assert_eq!(marker("nppdvjthqldpwncqszvftbrmjlhg", 4), 6);
        assert_eq!(marker("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 4), 10);
        assert_eq!(marker("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 4), 11);
    }

    #[test]
    fn test_part2() {
        assert_eq!(marker("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 14), 19);
        assert_eq!(marker("bvwbjplbgvbhsrlpgdmjqwftvncz", 14), 23);
        assert_eq!(marker("nppdvjthqldpwncqszvftbrmjlhg", 14), 23);
        assert_eq!(marker("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 14), 29);
        assert_eq!(marker("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 14), 26);
    }
}
