use std::collections::{HashMap, HashSet};

pub fn solve() {
    let data = std::fs::read_to_string("data/day22.txt").unwrap();

    // part 1
    println!("part 1: {}", part1(&data));

    // part 2
    println!("part 2: {}", part2(&data));
}

fn part1(data: &str) -> u64 {
    let bricks = settle(parse(data));
    let (hold, held) = graph(&bricks);

    // simply find all the bricks that only hold bricks that are
    // held by at least one other brick
    (0..bricks.len())
        .filter(|i| hold[i].iter().all(|j| held[j].len() > 1))
        .count() as u64
}

fn part2(data: &str) -> u64 {
    let bricks = settle(parse(data));
    let (hold, held) = graph(&bricks);

    (0..bricks.len())
        .map(|i| {
            // use DFS to find all the bricks that would be dropped
            // by the current brick
            let mut d = HashSet::new();
            let mut q = vec![i];
            while let Some(i) = q.pop() {
                d.insert(i);
                for j in hold[&i].iter() {
                    // if all bricks holding up the current brick have been
                    // dropped or disintegrated, drop the current brick
                    if held[j].iter().all(|i| d.contains(i)) {
                        q.push(*j);
                    }
                }
            }

            // don't count the disintegrated brick
            d.len() as u64 - 1
        })
        .sum::<u64>()
}

fn parse(data: &str) -> Vec<Brick> {
    data.lines()
        .map(|l| {
            let v: Vec<Vec<u64>> = l
                .split('~')
                .map(|s| s.split(',').map(|n| n.parse().unwrap()).collect())
                .collect();

            Brick {
                b: Coord {
                    x: v[0][0],
                    y: v[0][1],
                    z: v[0][2],
                },
                e: Coord {
                    x: v[1][0],
                    y: v[1][1],
                    z: v[1][2],
                },
            }
        })
        .collect()
}

fn settle(mut bricks: Vec<Brick>) -> Vec<Brick> {
    // we need the bricks to be sorted so that we can find all
    // bricks that have been settled before the current brick
    bricks.sort_by_key(|brick| brick.b.z);

    for i in 0..bricks.len() {
        // find the lowest heigh that the current brick can
        // drop, based on xy plane collisions with already
        // settled bricks
        let mut z = 1;
        for j in 0..i {
            if bricks[i].collide_xy(&bricks[j]) {
                z = std::cmp::max(z, bricks[j].e.z + 1);
            }
        }

        // settle the current brick at the new height
        bricks[i].e.z = z + bricks[i].e.z - bricks[i].b.z;
        bricks[i].b.z = z;
    }

    bricks
}

fn graph(bricks: &[Brick]) -> (HashMap<usize, Vec<usize>>, HashMap<usize, Vec<usize>>) {
    // create two mappings here, the list of bricks that each brick is holding, and
    // the list of bricks that each brick is held by
    let mut hold: HashMap<_, _> = (0..bricks.len()).map(|i| (i, vec![])).collect();
    let mut held = hold.clone();
    for i in 0..bricks.len() {
        for j in i + 1..bricks.len() {
            // if the current brick is one height above the previous and collides
            // in the xy plane, establish the holds/held by relation
            if bricks[i].collide_xy(&bricks[j]) && bricks[j].b.z == bricks[i].e.z + 1 {
                hold.get_mut(&i).unwrap().push(j);
                held.get_mut(&j).unwrap().push(i);
            }
        }
    }

    (hold, held)
}

struct Brick {
    b: Coord,
    e: Coord,
}

struct Coord {
    x: u64,
    y: u64,
    z: u64,
}

impl Brick {
    fn collide_xy(&self, other: &Self) -> bool {
        (self.b.x <= other.e.x && other.b.x <= self.e.x)
            && (self.b.y <= other.e.y && other.b.y <= self.e.y)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const DATA: &str = "\
        1,0,1~1,2,1\n\
        0,0,2~2,0,2\n\
        0,2,3~2,2,3\n\
        0,0,4~0,2,4\n\
        2,0,5~2,2,5\n\
        0,1,6~2,1,6\n\
        1,1,8~1,1,9\n\
    ";

    #[test]
    fn test_part1() {
        assert_eq!(part1(DATA), 5);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(DATA), 7);
    }
}
