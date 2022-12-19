use std::collections::HashSet;

pub fn solve() {
    let data = parse(&std::fs::read_to_string("data/day18.txt").expect("data-file"));

    // part 1
    println!("part 1: {}", part1(&data));

    // part 2
    println!("part 2: {}", part2(&data));
}

fn parse(data: &str) -> Vec<(i32, i32, i32)> {
    data.lines()
        .map(|l| {
            let v: Vec<i32> = l
                .split(',')
                .map(|s| s.parse().expect("valid-coord"))
                .collect();
            if v.len() != 3 {
                panic!("invalid-coord");
            }
            (v[0], v[1], v[2])
        })
        .collect()
}

fn part1(points: &[(i32, i32, i32)]) -> usize {
    // there are a total of 6 sides in each cube
    let all_sides = 6 * points.len();

    // an adjacent cube is one with manhattan distance = 1
    let adj_sides = (0..points.len())
        .flat_map(|i| (i + 1..points.len()).map(move |j| distance(points[i], points[j])))
        .filter(|&d| d == 1)
        .count();

    // remove touching sides from each pair of all adjacent cubes
    all_sides - 2 * adj_sides
}

fn part2(points: &[(i32, i32, i32)]) -> usize {
    // compute a bounding box for the whole droplet
    let minx = points.iter().map(|p| p.0).min().expect("points") - 1;
    let maxx = points.iter().map(|p| p.0).max().expect("points") + 1;
    let miny = points.iter().map(|p| p.1).min().expect("points") - 1;
    let maxy = points.iter().map(|p| p.1).max().expect("points") + 1;
    let minz = points.iter().map(|p| p.2).min().expect("points") - 1;
    let maxz = points.iter().map(|p| p.2).max().expect("points") + 1;

    // track the points on the boundary of the droplet and all points outside it
    let boundary: HashSet<_> = points.iter().copied().collect();
    let mut outside = HashSet::new();

    // run a dfs for all points inside the bounding box to find points outside the droplet
    let mut stack = vec![(minx, miny, minz)];
    let mut done = HashSet::new();
    while !stack.is_empty() {
        let (x, y, z) = stack.pop().expect("nonempty-stack");
        outside.insert((x, y, z));
        for point @ (p, q, r) in [
            (x - 1, y, z),
            (x + 1, y, z),
            (x, y - 1, z),
            (x, y + 1, z),
            (x, y, z - 1),
            (x, y, z + 1),
        ] {
            if (minx..=maxx).contains(&p)
                && (miny..=maxy).contains(&q)
                && (minz..=maxz).contains(&r)
                && done.insert(point)
                && !boundary.contains(&point)
            {
                stack.push(point);
            }
        }
    }

    // now we can find all points inside the droplet, which are all
    // points inside the bounding box but not outside the droplet or on its boundary
    let inside: HashSet<_> = (minx..=maxx)
        .flat_map(|x| (miny..=maxy).flat_map(move |y| (minz..=maxz).map(move |z| (x, y, z))))
        .filter(|p| !outside.contains(p) && !boundary.contains(p))
        .collect();

    // compute the number of sides on the boundary that are adjacent to an interior point
    let interior_sides = points
        .iter()
        .flat_map(|&p| inside.iter().map(move |&q| distance(p, q)))
        .filter(|&d| d == 1)
        .count();

    // remove those interior-facing sides for the answer
    part1(points) - interior_sides
}

fn distance(p: (i32, i32, i32), q: (i32, i32, i32)) -> i32 {
    (q.0 - p.0).abs() + (q.1 - p.1).abs() + (q.2 - p.2).abs()
}

#[cfg(test)]
mod tests {
    use super::*;

    const DATA: &str = "\
        2,2,2\n\
        1,2,2\n\
        3,2,2\n\
        2,1,2\n\
        2,3,2\n\
        2,2,1\n\
        2,2,3\n\
        2,2,4\n\
        2,2,6\n\
        1,2,5\n\
        3,2,5\n\
        2,1,5\n\
        2,3,5\n\
    ";

    #[test]
    fn test_part1() {
        assert_eq!(part1(&parse(DATA)), 64);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&parse(DATA)), 58);
    }
}
