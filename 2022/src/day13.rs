use std::cmp::{max, Ordering};

#[derive(Clone, Debug, Eq, PartialEq)]
enum List {
    Num(u32),
    Vec(Vec<List>),
}

pub fn solve() {
    let data = std::fs::read_to_string("data/day13.txt").expect("data-file");

    // part 1
    println!("part 1: {}", part1(&parse1(&data)));

    // part 2
    println!("part 2: {}", part2(&parse2(&data)));
}

fn parse1(data: &str) -> Vec<(List, List)> {
    data.lines()
        .collect::<Vec<_>>()
        .chunks(3)
        .map(|lines| (List::parse(lines[0]), List::parse(lines[1])))
        .collect()
}

fn parse2(data: &str) -> Vec<List> {
    data.lines()
        .filter(|l| !l.is_empty())
        .map(List::parse)
        .collect()
}

fn part1(lists: &[(List, List)]) -> usize {
    lists
        .iter()
        .enumerate()
        .map(|(i, (lhs, rhs))| if lhs <= rhs { i + 1 } else { 0 })
        .sum()
}

fn part2(lists: &[List]) -> usize {
    let mut lists = Vec::from(lists);
    let l1 = List::parse("[[2]]");
    let l2 = List::parse("[[6]]");
    lists.push(l1.clone());
    lists.push(l2.clone());
    lists.sort();

    let mut i1 = 0;
    let mut i2 = 0;
    for (i, l) in lists.iter().enumerate() {
        if i1 == 0 && *l == l1 {
            i1 = i + 1;
        } else if i2 == 0 && *l == l2 {
            i2 = i + 1;
        }
    }

    i1 * i2
}

impl List {
    fn parse(data: &str) -> List {
        let (list, _) = List::doparse(&data.chars().collect::<Vec<_>>(), 0);
        list
    }

    fn doparse(data: &[char], offset: usize) -> (List, usize) {
        let mut offset = offset;
        let mut vec = Vec::new();

        offset += 1;
        while data[offset] != ']' {
            if data[offset] == '[' {
                // parse a sub-list
                let (child_list, child_offset) = List::doparse(data, offset);
                offset = child_offset;
                vec.push(child_list);
            } else {
                // parse a number
                let mut end = offset + 1;
                while data[end] != ',' && data[end] != ']' {
                    end += 1
                }
                vec.push(List::Num(
                    data[offset..end]
                        .iter()
                        .collect::<String>()
                        .parse()
                        .expect("number"),
                ));
                offset = end;
            }
            if data[offset] == ',' {
                offset += 1;
            }
        }
        (List::Vec(vec), offset + 1)
    }
}

impl PartialOrd for List {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for List {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            // simple number-number comparison
            (List::Num(x), List::Num(y)) => x.cmp(y),

            // number-list comparisons
            (&List::Num(x), y @ List::Vec(_)) => List::Vec(vec![List::Num(x)]).cmp(y),
            (x @ List::Vec(_), &List::Num(y)) => x.cmp(&List::Vec(vec![List::Num(y)])),

            // list-list comparisons
            (List::Vec(x), List::Vec(y)) => {
                for i in 0..max(x.len(), y.len()) {
                    // handle short lists
                    if i >= x.len() {
                        return Ordering::Less;
                    } else if i >= y.len() {
                        return Ordering::Greater;
                    }

                    // break on the first non-equal list element
                    match x[i].cmp(&y[i]) {
                        Ordering::Equal => {}
                        cmp => return cmp,
                    }
                }
                Ordering::Equal
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const DATA: &str = "\
[1,1,3,1,1]
[1,1,5,1,1]

[[1],[2,3,4]]
[[1],4]

[9]
[[8,7,6]]

[[4,4],4,4]
[[4,4],4,4,4]

[7,7,7,7]
[7,7,7]

[]
[3]

[[[]]]
[[]]

[1,[2,[3,[4,[5,6,7]]]],8,9]
[1,[2,[3,[4,[5,6,0]]]],8,9]
";

    #[test]
    fn test_part1() {
        assert_eq!(part1(&parse1(DATA)), 13);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&parse2(DATA)), 140);
    }
}
