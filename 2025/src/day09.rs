use std::cmp::{max, min};

pub fn solve() {
    let data = std::fs::read_to_string("data/day09.txt").unwrap();
    let lines: Vec<&str> = data.lines().collect();

    // part 1
    println!("part 1: {}", part1(&lines));

    // part 2
    println!("part 2: {}", part2(&lines));
}

fn part1(lines: &[&str]) -> u64 {
    let coords = &parse(lines);

    (0..coords.len())
        .flat_map(|i| {
            (i + 1..coords.len()).map(move |j| {
                let (x0, y0) = coords[i];
                let (x1, y1) = coords[j];
                rect_size(x0, y0, x1, y1)
            })
        })
        .max()
        .unwrap()
}

fn part2(lines: &[&str]) -> u64 {
    let coords = &parse(lines);
    (0..coords.len())
        .flat_map(|i| (i + 1..coords.len()).map(move |j| (coords[i], coords[j])))
        .filter(|&((x0, y0), (x1, y1))| {
            let (x0, x1) = (min(x0, x1), max(x0, x1));
            let (y0, y1) = (min(y0, y1), max(y0, y1));

            // verify all of the red tiles (corners) are inside the polygon
            for (x, y) in [(x0, y0), (x0, y1), (x1, y0), (x1, y1)] {
                let mut xing = 0;
                for i in 0..coords.len() {
                    let j = (i + 1) % coords.len();
                    let (x2, y2) = coords[i];
                    let (x3, y3) = coords[j];
                    let (x2, x3) = (min(x2, x3), max(x2, x3));
                    let (y2, y3) = (min(y2, y3), max(y2, y3));
                    if x2 == x3 && x2 == x {
                        // test for point on vertical line
                        if y2 <= y && y <= y3 {
                            xing = 1;
                            break;
                        }
                    } else if y2 == y3 && y2 == y {
                        // test for point on horizontal line
                        if x2 <= x && x <= x3 {
                            xing = 1;
                            break;
                        }
                    } else if x2 == x3 && x2 > x {
                        // test for ray crossing vertical line
                        // update crossing based on edge winding
                        if y2 <= y && y <= y3 {
                            xing += 1;
                        } else if y3 <= y && y <= y2 {
                            xing -= 1;
                        }
                    }
                }

                // if crossings were matched, we are outside the polygon
                if xing == 0 {
                    return false;
                }
            }

            // verify there are no intersections between the rectangle and the polygon
            for i in 0..coords.len() {
                let j = (i + 1) % coords.len();
                let (x2, y2) = coords[i];
                let (x3, y3) = coords[j];
                let (x2, x3) = (min(x2, x3), max(x2, x3));
                let (y2, y3) = (min(y2, y3), max(y2, y3));
                if x2 == x3 {
                    // vertical edge
                    // check for intersection with rectangle horizontal lines
                    let x = x2;
                    if x0 < x && x < x1 && y2 < y1 && y3 > y0 {
                        return false;
                    }
                } else {
                    // horizontal edge
                    // check for intersection with rectangle vertical lines
                    let y = y2;
                    if y0 < y && y < y1 && x2 < x1 && x3 > x0 {
                        return false;
                    }
                }
            }

            true
        })
        .map(|((x0, y0), (x1, y1))| rect_size(x0, y0, x1, y1))
        .max()
        .unwrap()
}

fn parse(lines: &[&str]) -> Vec<(i64, i64)> {
    lines
        .iter()
        .map(|l| {
            let (x, y) = l.split_once(',').unwrap();
            (x.parse::<i64>().unwrap(), y.parse::<i64>().unwrap())
        })
        .collect()
}

fn rect_size(x0: i64, y0: i64, x1: i64, y1: i64) -> u64 {
    (x1.abs_diff(x0) + 1) * (y1.abs_diff(y0) + 1)
}

#[cfg(test)]
mod tests {
    use super::*;

    const DATA: &[&str] = &["7,1", "11,1", "11,7", "9,7", "9,5", "2,5", "2,3", "7,3"];

    #[test]
    fn test_part1() {
        assert_eq!(part1(DATA), 50);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(DATA), 24);
    }
}
