use std::collections::HashMap;

pub fn solve() {
    let data = std::fs::read_to_string("data/day12.txt").unwrap();

    // part 1
    println!("part 1: {}", part1(&data));

    // part 2
    println!("part 2: {}", part2(&data));
}

fn part1(data: &str) -> u64 {
    data.lines()
        .map(|line| {
            let (ss, ns) = parse(line);
            count(&ss, &ns)
        })
        .sum()
}

#[allow(clippy::useless_vec)]
fn part2(data: &str) -> u64 {
    data.lines()
        .map(|line| {
            let (mut ss, ns) = parse(line);
            let so = ss.clone();
            for _ in 0..4 {
                ss.push('?');
                ss.extend(so.clone());
            }
            count(&ss, &vec![ns; 5].concat())
        })
        .sum()
}

fn parse(line: &str) -> (Vec<char>, Vec<u64>) {
    let split: Vec<_> = line.split_whitespace().collect();
    (
        split[0].chars().collect(),
        split[1].split(',').map(|s| s.parse().unwrap()).collect(),
    )
}

fn count(ss: &[char], ns: &[u64]) -> u64 {
    do_count(ss, ns, false, &mut HashMap::new())
}

fn do_count(
    ss: &[char],
    ns: &[u64],
    matching: bool,
    cache: &mut HashMap<(Vec<char>, Vec<u64>, bool), u64>,
) -> u64 {
    // first, check to see if we have a memoized value for these parameters
    let k = (ss.to_vec(), ns.to_vec(), matching);
    if let Some(&v) = cache.get(&k) {
        return v;
    }

    // if not, calculate the number of possible matches
    let v = if ss.is_empty() && ns.is_empty() {
        // we found a valid match when both vectors are empty
        1
    } else if ns.is_empty() && matches!(ss[0], '.' | '?') {
        // we are out of counts, continue matching operational/damaged springs
        do_count(&ss[1..], ns, matching, cache)
    } else if ss.is_empty() && ns[0] == 0 {
        // we are out of strings, continue matching if we have counts of 0
        do_count(ss, &ns[1..], matching, cache)
    } else if ss.is_empty() || ns.is_empty() {
        // we found an invalid match if one of the lists remains empty but not the other
        0
    } else if ns[0] == 0 {
        // we have counted down the damaged spring to 0,
        // so we now must have a working spring next or an unknown
        if matches!(ss[0], '.' | '?') {
            do_count(&ss[1..], &ns[1..], false, cache)
        } else {
            0
        }
    } else if ss[0] == '.' {
        // if we are currently inside a damaged spring match and we hit a working spring,
        // fail the match, otherwise ignore the working spring and continue
        if matching {
            0
        } else {
            do_count(&ss[1..], ns, matching, cache)
        }
    } else if ss[0] == '?' {
        // if we are not currently inside a damaged spring match, first count all the
        // matches for an assumed undamaged spring
        let other = if matching {
            0
        } else {
            do_count(&ss[1..], ns, matching, cache)
        };

        // add this to the number of ways to match the current damaged spring
        let mut ns = ns.to_vec();
        ns[0] -= 1;
        other + do_count(&ss[1..], &ns, true, cache)
    } else if ss[0] == '#' {
        // begin/continue the match for the current damaged spring
        let mut ns = ns.to_vec();
        ns[0] -= 1;
        do_count(&ss[1..], &ns, true, cache)
    } else {
        0
    };

    // memoize and return the match counts
    cache.insert(k, v);
    v
}

#[cfg(test)]
mod tests {
    use super::*;

    const DATA: &str = "\
        ???.### 1,1,3\n\
        .??..??...?##. 1,1,3\n\
        ?#?#?#?#?#?#?#? 1,3,1,6\n\
        ????.#...#... 4,1,1\n\
        ????.######..#####. 1,6,5\n\
        ?###???????? 3,2,1\n\
    ";

    #[test]
    fn test_part1() {
        assert_eq!(part1(DATA), 21);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(DATA), 525152);
    }
}
