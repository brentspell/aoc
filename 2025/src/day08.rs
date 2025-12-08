use std::collections::HashMap;

use itertools::Itertools;

pub fn solve() {
    let data = std::fs::read_to_string("data/day08.txt").unwrap();
    let lines: Vec<&str> = data.lines().collect();

    // part 1
    println!("part 1: {}", part1(&lines, 1000));

    // part 2
    println!("part 2: {}", part2(&lines));
}

fn part1(lines: &[&str], pairs: usize) -> u64 {
    let boxes = &parse(lines);

    // construct the initial union-find tree
    let mut tree = HashMap::new();
    for i in 0..boxes.len() {
        tree.insert(
            i,
            Node {
                id: i,
                parent: i,
                size: 1,
            },
        );
    }

    // merge nearest neighbors into connected components
    for &(i, j) in neighbors(boxes).iter().take(pairs) {
        union(&mut tree, i, j);
    }

    // find the 3 largest connected components and multiple their sizes
    tree.values()
        .filter(|n| n.parent == n.id)
        .map(|n| n.size)
        .sorted()
        .rev()
        .take(3)
        .product::<usize>() as u64
}

fn part2(lines: &[&str]) -> i64 {
    let boxes = &parse(lines);

    // construct the initial union-find tree
    let mut tree = HashMap::new();
    for i in 0..boxes.len() {
        tree.insert(
            i,
            Node {
                id: i,
                parent: i,
                size: 1,
            },
        );
    }

    // merge neighbors into connected components until all junction boxes are connected
    for (i, j) in neighbors(boxes) {
        let r = union(&mut tree, i, j);
        if tree[&r].size == boxes.len() {
            return boxes[i].0 * boxes[j].0;
        }
    }

    panic!("couldn't connect all junction boxes");
}

fn parse(lines: &[&str]) -> Vec<(i64, i64, i64)> {
    lines
        .iter()
        .map(|l| {
            let v = l
                .split(',')
                .map(|s| s.parse::<i64>().unwrap())
                .collect::<Vec<_>>();
            (v[0], v[1], v[2])
        })
        .collect::<Vec<_>>()
}

fn neighbors(boxes: &[(i64, i64, i64)]) -> Vec<(usize, usize)> {
    (0..boxes.len())
        .flat_map(|i| {
            (i + 1..boxes.len()).map(move |j| {
                let (x0, y0, z0) = boxes[i];
                let (x1, y1, z1) = boxes[j];
                let d = (x1 - x0).pow(2) + (y1 - y0).pow(2) + (z1 - z0).pow(2);
                (d, i, j)
            })
        })
        .sorted()
        .map(|(_d, i, j)| (i, j))
        .collect::<Vec<_>>()
}

#[derive(Debug)]
struct Node {
    id: usize,
    parent: usize,
    size: usize,
}

fn union(nodes: &mut HashMap<usize, Node>, i: usize, j: usize) -> usize {
    let mut ri = find(nodes, i);
    let mut rj = find(nodes, j);
    if ri != rj {
        if nodes[&ri].size < nodes[&rj].size {
            (ri, rj) = (rj, ri);
        }
        nodes.get_mut(&rj).unwrap().parent = ri;
        nodes.get_mut(&ri).unwrap().size += nodes.get(&rj).unwrap().size;
    }

    ri
}

fn find(nodes: &HashMap<usize, Node>, id: usize) -> usize {
    let node = nodes.get(&id).unwrap();
    if node.parent == id {
        id
    } else {
        find(nodes, node.parent)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const DATA: &[&str] = &[
        "162,817,812",
        "57,618,57",
        "906,360,560",
        "592,479,940",
        "352,342,300",
        "466,668,158",
        "542,29,236",
        "431,825,988",
        "739,650,466",
        "52,470,668",
        "216,146,977",
        "819,987,18",
        "117,168,530",
        "805,96,715",
        "346,949,466",
        "970,615,88",
        "941,993,340",
        "862,61,35",
        "984,92,344",
        "425,690,689",
    ];

    #[test]
    fn test_part1() {
        assert_eq!(part1(DATA, 10), 40);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(DATA), 25272);
    }
}
