use std::collections::{BinaryHeap, HashMap, HashSet};

pub fn solve() {
    let data = parse(&std::fs::read_to_string("data/day24.txt").expect("data-file"));

    // part 1
    println!("part 1: {}", part1(&data));

    // part 2
    println!("part 2: {}", part2(&data));
}

fn parse(data: &str) -> Vec<Vec<char>> {
    data.lines().map(|l| l.chars().collect()).collect()
}

fn part1(data: &[Vec<char>]) -> u32 {
    let beg = (0, 1);
    let end = (data.len() - 1, data[0].len() - 2);

    // perform a single search through the maze
    search(data, &mut HashMap::new(), beg, end, 0)
}

fn part2(data: &[Vec<char>]) -> u32 {
    let mut cache = HashMap::new();
    let mut beg = (0, 1);
    let mut end = (data.len() - 1, data[0].len() - 2);
    let mut time = 0;

    // search through the maze 3 times, swapping the begin/end each time
    // and accumulating the total number of times
    for _ in 0..3 {
        time = search(data, &mut cache, beg, end, time);
        (beg, end) = (end, beg);
    }

    time
}

fn search(
    data: &[Vec<char>],
    cache: &mut HashMap<u32, HashMap<(usize, usize), Vec<char>>>,
    beg: (usize, usize),
    end: (usize, usize),
    time: u32,
) -> u32 {
    // use A* search to find the shortest path from beginning to end
    // the cost heuristic is the current number of timesteps + the manhattan distance to the end
    // use the done set to ensure we don't end up at a previously-seen point/time,
    // which can happen due to waiting
    let mut queue = BinaryHeap::from([(-(distance(beg, end) as i32), time, beg)]);
    let mut done = HashSet::new();
    loop {
        // check the current position to see if we have reached the end
        let (_cost, time, (i, j)) = queue.pop().expect("nonempty-queue");
        if (i, j) == end {
            return time;
        }

        // compute the next blizzard state (with a possible wait) and possible steps
        let time = time + 1;
        let bliz = blizzards(data, time, cache);
        let mut next = vec![(i, j)];
        if i > 0 {
            next.push((i - 1, j));
        }
        if i < data.len() - 1 {
            next.push((i + 1, j));
        }
        if j > 0 {
            next.push((i, j - 1));
        }
        if j < data[0].len() - 1 {
            next.push((i, j + 1));
        }

        // explore the next valid steps, ordered by the cost heuristic
        // don't step on any boundaries or blizzards or return to a seen state
        for (p, q) in next {
            if data[p][q] != '#' && !bliz.contains_key(&(p, q)) && done.insert((p, q, time)) {
                queue.push((-(time as i32 + distance((p, q), end) as i32), time, (p, q)));
            }
        }
    }
}

fn blizzards<'a>(
    data: &[Vec<char>],
    time: u32,
    cache: &'a mut HashMap<u32, HashMap<(usize, usize), Vec<char>>>,
) -> &'a HashMap<(usize, usize), Vec<char>> {
    // memoize the blizzard states, since they will be needed many times
    if !cache.contains_key(&time) {
        let bliz = if time == 0 {
            // at time 0, load the blizzard state from the map
            (0..data.len())
                .flat_map(|i| (0..data[0].len()).map(move |j| ((i, j), vec![data[i][j]])))
                .filter(|(_p, d)| "^>v<".contains(d[0]))
                .collect()
        } else {
            // otherwise, retrieve the previous timestep's map
            // and walk the simulation forward one step
            // store the blizzards in a list, since multiple
            // blizzards can occupy the same location
            let mut bliz = HashMap::new();
            for (&(i, j), ds) in blizzards(data, time - 1, cache) {
                for &d in ds {
                    bliz.entry(blizzard_next(data, i, j, d))
                        .or_insert_with(Vec::new)
                        .push(d);
                }
            }
            bliz
        };

        cache.insert(time, bliz);
    }

    &cache[&time]
}

fn blizzard_next(data: &[Vec<char>], i: usize, j: usize, d: char) -> (usize, usize) {
    let mut i = i;
    let mut j = j;

    // walk the blizzard one step
    match d {
        '^' => i -= 1,
        '>' => j += 1,
        'v' => i += 1,
        '<' => j -= 1,
        _ => panic!("invalid-dir"),
    }

    // if we reach an edge, wrap around
    if i == 0 {
        i = data.len() - 2;
    } else if i == data.len() - 1 {
        i = 1;
    }
    if j == 0 {
        j = data[0].len() - 2;
    } else if j == data[0].len() - 1 {
        j = 1;
    }

    (i, j)
}

// manhattan distance with usizes
fn distance(a: (usize, usize), b: (usize, usize)) -> usize {
    (if a.0 < b.0 { b.0 - a.0 } else { a.0 - b.0 })
        + (if a.1 < b.1 { b.1 - a.1 } else { a.1 - b.1 })
}

#[cfg(test)]
mod tests {
    use super::*;

    const DATA: &str = "\
#.######
#>>.<^<#
#.<..<<#
#>v.><>#
#<^v^^>#
######.#
";

    #[test]
    fn test_part1() {
        assert_eq!(part1(&parse(DATA)), 18);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&parse(DATA)), 54);
    }
}
