use std::cmp::{max, min};
use std::ops::Range;

pub fn solve() {
    let data = std::fs::read_to_string("data/day05.txt").unwrap();
    let (seeds, maps) = parse(&data);

    // part 1
    println!("part 1: {}", part1(&seeds, &maps));

    // part 2
    println!("part 2: {}", part2(&seeds, &maps));
}

fn part1(seeds: &[i64], maps: &[Vec<Mapping>]) -> i64 {
    // pass all the seeds through each of the mappings,
    // and map it to the target offset of the intersecting interval, if any
    // take the smallest resulting id
    seeds
        .iter()
        .map(|&x| {
            let mut x = x;
            for ms in maps.iter() {
                for m in ms.iter() {
                    if let Some((_, y, _)) = m.split(&(x..x + 1)) {
                        x = y.start;
                        break;
                    }
                }
            }
            x
        })
        .min()
        .unwrap()
}

fn part2(seeds: &[i64], maps: &[Vec<Mapping>]) -> i64 {
    // first, convert the list of seed ids into ranges for part 2
    let mut ranges: Vec<_> = (0..seeds.len() / 2)
        .map(|i| seeds[2 * i]..seeds[2 * i] + seeds[2 * i + 1])
        .collect();

    // pass the id ranges through each mapping successively,
    // splitting them and updating the id values that overlap with a mapping
    for ms in maps.iter() {
        // map all of the ranges currently in the list
        let mut next = vec![];
        while let Some(range) = ranges.pop() {
            let split = 'split: {
                for m in ms.iter() {
                    // compute the intersection and difference
                    // between the current range and mapping
                    if let Some((lhs, out, rhs)) = m.split(&range) {
                        // if we found a match, place the mapped intersection into
                        // the next queue, and put the lhs/rhs differences
                        // back on the range list for further processing
                        next.push(out);
                        if !lhs.is_empty() {
                            ranges.push(lhs);
                        }
                        if !rhs.is_empty() {
                            ranges.push(rhs);
                        }
                        break 'split true;
                    }
                }
                false
            };

            // if no overlapping mapping was found,
            // forward the range on to the next layer as-is
            if !split {
                next.push(range);
            }
        }
        ranges = next;
    }

    ranges.iter().map(|r| r.start).min().unwrap()
}

fn parse(data: &str) -> (Vec<i64>, Vec<Vec<Mapping>>) {
    // split the input into sections, delimited by double newlines
    let sections: Vec<_> = data.split("\n\n").collect();

    // parse the list of seed ids (these are ranges for part 2)
    let seeds: Vec<_> = sections[0][7..]
        .split_whitespace()
        .map(|n| n.parse::<i64>().unwrap())
        .collect();

    // parse the mappings in each section after the seed list
    let maps: Vec<Vec<Mapping>> = sections[1..]
        .iter()
        .map(|section| {
            section
                .lines()
                .skip(1)
                .map(|line| {
                    let t: Vec<_> = line
                        .split_whitespace()
                        .map(|n| n.parse::<i64>().unwrap())
                        .collect();
                    Mapping {
                        offset: t[0],
                        range: t[1]..t[1] + t[2],
                    }
                })
                .collect()
        })
        .collect();

    (seeds, maps)
}

#[derive(Debug)]
struct Mapping {
    offset: i64,
    range: Range<i64>,
}

impl Mapping {
    fn split(&self, source: &Range<i64>) -> Option<(Range<i64>, Range<i64>, Range<i64>)> {
        let r = &self.range;

        // compute and map the interval intersection
        let int = max(source.start, r.start)..min(source.end, r.end);
        if int.is_empty() {
            None
        } else {
            let map = int.start - r.start + self.offset..int.end - r.start + self.offset;

            // compute the interval left/right side differences
            let lhs = source.start..min(source.end, r.start);
            let rhs = max(source.start, r.end)..source.end;

            Some((lhs, map, rhs))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const DATA: &str = "\
        seeds: 79 14 55 13\n\
        \n\
        seed-to-soil map:\n\
        50 98 2\n\
        52 50 48\n\
        \n\
        soil-to-fertilizer map:\n\
        0 15 37\n\
        37 52 2\n\
        39 0 15\n\
        \n\
        fertilizer-to-water map:\n\
        49 53 8\n\
        0 11 42\n\
        42 0 7\n\
        57 7 4\n\
        \n\
        water-to-light map:\n\
        88 18 7\n\
        18 25 70\n\
        \n\
        light-to-temperature map:\n\
        45 77 23\n\
        81 45 19\n\
        68 64 13\n\
        \n\
        temperature-to-humidity map:\n\
        0 69 1\n\
        1 0 69\n\
        \n\
        humidity-to-location map:\n\
        60 56 37\n\
        56 93 4\n\
    ";

    #[test]
    fn test_part1() {
        let (seeds, maps) = parse(DATA);
        assert_eq!(part1(&seeds, &maps), 35);
    }

    #[test]
    fn test_part2() {
        let (seeds, maps) = parse(DATA);
        assert_eq!(part2(&seeds, &maps), 46);
    }
}
