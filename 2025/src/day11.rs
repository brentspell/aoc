use std::collections::HashMap;

use cached::{UnboundCache, proc_macro::cached};

pub fn solve() {
    let data = std::fs::read_to_string("data/day11.txt").unwrap();
    let lines: Vec<&str> = data.lines().collect();

    // part 1
    println!("part 1: {}", part1(&lines));

    // part 2
    println!("part 2: {}", part2(&lines));
}

fn part1(lines: &[&str]) -> u64 {
    let graph = parse(lines);

    // dfs to find the number of paths from you to out
    let mut stack = vec!["you"];
    let mut total = 0;
    while let Some(node) = stack.pop() {
        if node == "out" {
            total += 1;
        }
        stack.extend(graph.get(node).unwrap_or(&vec![]));
    }

    total
}

fn part2(lines: &[&str]) -> u64 {
    let graph = parse(lines);

    // dfs to find the number of paths from svr to out that pass through dac+fft
    search(&graph, "svr", "out", false, false)
}

// DP: memoize the search function
#[cached(
    ty = "UnboundCache<(String, String, bool, bool), u64>",
    create = "{ UnboundCache::new() }",
    convert = r#"{ (src.to_string(), tgt.to_string(), hasdac, hasfft) }"#
)]
fn search(
    graph: &HashMap<&str, Vec<&str>>,
    src: &str,
    tgt: &str,
    hasdac: bool,
    hasfft: bool,
) -> u64 {
    // only include paths that pass through dac+fft
    if src == tgt {
        return if hasdac && hasfft { 1 } else { 0 };
    }

    // dfs to find the number of paths from children to the target
    let mut total = 0;
    for &src in graph.get(src).unwrap_or(&vec![]) {
        total += search(
            graph,
            src,
            tgt,
            hasdac || src == "dac",
            hasfft || src == "fft",
        );
    }

    total
}

fn parse<'a>(lines: &'a [&str]) -> HashMap<&'a str, Vec<&'a str>> {
    let mut graph = HashMap::new();
    for &line in lines {
        let (src, tgt) = line.split_once(':').unwrap();
        for tgt in tgt.split_whitespace() {
            graph.entry(src).or_insert(vec![]).push(tgt);
        }
    }

    graph
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        const DATA: &[&str] = &[
            "aaa: you hhh",
            "you: bbb ccc",
            "bbb: ddd eee",
            "ccc: ddd eee fff",
            "ddd: ggg",
            "eee: out",
            "fff: out",
            "ggg: out",
            "hhh: ccc fff iii",
            "iii: out",
        ];

        assert_eq!(part1(DATA), 5);
    }

    #[test]
    fn test_part2() {
        const DATA: &[&str] = &[
            "svr: aaa bbb",
            "aaa: fft",
            "fft: ccc",
            "bbb: tty",
            "tty: ccc",
            "ccc: ddd eee",
            "ddd: hub",
            "hub: fff",
            "eee: dac",
            "dac: fff",
            "fff: ggg hhh",
            "ggg: out",
            "hhh: out",
        ];

        assert_eq!(part2(DATA), 2);
    }
}
