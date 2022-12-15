use regex::Regex;
use std::collections::HashSet;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

pub fn solve() {
    let data = parse(&std::fs::read_to_string("data/day15.txt").expect("data-file"));

    // part 1
    println!("part 1: {}", part1(&data, 2000000));

    // part 2
    println!("part 2: {}", part2(&data, 4000000));
}

fn parse(data: &str) -> Vec<(Point, Point)> {
    data.lines()
        .map(|line| {
            let coords: Vec<i32> = LINE_REGEX
                .captures(line)
                .expect("line-match")
                .iter()
                .skip(1)
                .map(|c| c.expect("coord").as_str().parse().expect("coord"))
                .collect();
            (
                Point {
                    x: coords[0],
                    y: coords[1],
                },
                Point {
                    x: coords[2],
                    y: coords[3],
                },
            )
        })
        .collect()
}

fn part1(data: &[(Point, Point)], y: i32) -> u32 {
    let mut covered: HashSet<i32> = HashSet::new();
    for (sensor, beacon) in data {
        let db = (sensor.x - beacon.x).abs() + (sensor.y - beacon.y).abs();
        let dy = (sensor.y - y).abs();
        if dy <= db {
            let dx = db - dy;
            for x in sensor.x - dx..=sensor.x + dx {
                covered.insert(x);
            }
        }
    }
    for (_sensor, beacon) in data {
        if beacon.y == y {
            covered.remove(&beacon.x);
        }
    }
    covered.len() as u32
}

fn part2(data: &[(Point, Point)], max_coord: i32) -> u64 {
    let db: Vec<i32> = data
        .iter()
        .map(|(s, b)| (s.x - b.x).abs() + (s.y - b.y).abs())
        .collect();

    // search for a row that is not covered by sensors
    for y in 0..=max_coord {
        // maintain a list of intervals in the row not covered by sensors
        let mut holes = vec![(0, max_coord)];
        for ((sensor, _beacon), &db) in data.iter().zip(&db) {
            let dy = (sensor.y - y).abs();
            if dy <= db {
                let dx = db - dy;
                let sl = sensor.x - dx;
                let sr = sensor.x + dx;
                // remove all points from the holes that are covered by the current sensor
                let mut i = 0;
                while i < holes.len() {
                    let (hl, hr) = holes[i];
                    if sl <= hl && hr <= sr {
                        // hole contained by sensor
                        holes.remove(i);
                    } else if sl <= hl && hl <= sr {
                        // sensor/hole overlap from the left
                        holes[i] = (sr + 1, hr);
                        i += 1;
                    } else if sl <= hr && hr <= sr {
                        // sensor/hole overlap from the right
                        holes[i] = (hl, sl - 1);
                        i += 1;
                    } else if hl <= sl && sr <= hr {
                        // sensor contained by hole
                        if hl == sl {
                            // the sensor is aligned on the left
                            holes[i] = (sr + 1, hr);
                            i += 1;
                        } else {
                            holes[i] = (hl, sl - 1);
                            i += 1;
                            // if the sensor is not aligned on the right,
                            // we must add a new hole for it
                            if sr != hr {
                                holes.insert(i, (sr + 1, hr));
                                i += 1;
                            }
                        }
                    } else {
                        // no overlap
                        i += 1;
                    }
                }
            }
        }

        // if there is anything not covered by a sensor, we have found the row
        if !holes.is_empty() {
            let mut covered: HashSet<i32> = HashSet::new();
            for ((sensor, _beacon), &db) in data.iter().zip(&db) {
                let dy = (sensor.y - y).abs();
                if dy <= db {
                    let dx = db - dy;
                    for x in sensor.x - dx..=sensor.x + dx {
                        covered.insert(x);
                    }
                }
            }
            for x in 0..=max_coord {
                if !covered.contains(&x) {
                    return x as u64 * 4000000 + y as u64;
                }
            }
        }
    }
    panic!("hole-not-found");
}

lazy_static! {
    static ref LINE_REGEX: Regex = Regex::new(
        r#"^Sensor at x=(-?\d+), y=(-?\d+): closest beacon is at x=(-?\d+), y=(-?\d+)$"#
    )
    .unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;

    const DATA: &str = "\
        Sensor at x=2, y=18: closest beacon is at x=-2, y=15\n\
        Sensor at x=9, y=16: closest beacon is at x=10, y=16\n\
        Sensor at x=13, y=2: closest beacon is at x=15, y=3\n\
        Sensor at x=12, y=14: closest beacon is at x=10, y=16\n\
        Sensor at x=10, y=20: closest beacon is at x=10, y=16\n\
        Sensor at x=14, y=17: closest beacon is at x=10, y=16\n\
        Sensor at x=8, y=7: closest beacon is at x=2, y=10\n\
        Sensor at x=2, y=0: closest beacon is at x=2, y=10\n\
        Sensor at x=0, y=11: closest beacon is at x=2, y=10\n\
        Sensor at x=20, y=14: closest beacon is at x=25, y=17\n\
        Sensor at x=17, y=20: closest beacon is at x=21, y=22\n\
        Sensor at x=16, y=7: closest beacon is at x=15, y=3\n\
        Sensor at x=14, y=3: closest beacon is at x=15, y=3\n\
        Sensor at x=20, y=1: closest beacon is at x=15, y=3\n\
    ";

    #[test]
    fn test_part1() {
        assert_eq!(part1(&parse(DATA), 10), 26);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&parse(DATA), 20), 56000011);
    }
}
