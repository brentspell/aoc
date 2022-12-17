use std::cmp::min;
use std::collections::HashMap;

pub fn solve() {
    let (flows, tunnels) = parse(&std::fs::read_to_string("data/day16.txt").expect("data-file"));
    let graph = floyd_warshall(&tunnels);

    // part 1
    println!("part 1: {}", part1(&graph, &flows));

    // part 2
    println!("part 2: {}", part2(&graph, &flows));
}

fn parse(data: &str) -> (HashMap<String, u32>, HashMap<String, Vec<String>>) {
    let mut flows = HashMap::new();
    let mut tunnels = HashMap::new();
    for line in data.lines() {
        let (v, t) = line.split_once("; ").expect("separated-line");

        // parse the valve name/flow spec
        let valve: String = v.chars().skip(6).take(2).collect();
        let flow: u32 = v
            .chars()
            .skip(23)
            .collect::<String>()
            .parse()
            .expect("valid-flow");
        if flow > 0 {
            flows.insert(valve.clone(), flow);
        }

        // parse the tunnel list
        let tunnel: Vec<String> = t
            .chars()
            .skip(if t.starts_with("tunnels") { 23 } else { 22 })
            .collect::<String>()
            .split(", ")
            .map(|s| s.to_string())
            .collect();
        tunnels.insert(valve, tunnel);
    }
    (flows, tunnels)
}

// use floyd-warshall to compute all-pairs shortest paths,
// in order to squash 0-flow valves to reduce the search space
fn floyd_warshall(graph: &HashMap<String, Vec<String>>) -> HashMap<String, HashMap<String, u32>> {
    let mut result: HashMap<String, HashMap<String, u32>> = graph
        .keys()
        .map(|k1| {
            (
                k1.clone(),
                graph
                    .keys()
                    .map(|k2| {
                        (
                            k2.clone(),
                            if graph.get(k1).map(|v| v.contains(k2)).unwrap_or(false) {
                                1
                            } else {
                                u32::MAX / 2
                            },
                        )
                    })
                    .collect(),
            )
        })
        .collect();

    for k in graph.keys() {
        for i in graph.keys() {
            for j in graph.keys() {
                *result
                    .get_mut(i)
                    .expect("graph-entry")
                    .get_mut(j)
                    .expect("graph-entry") = min(result[i][j], result[i][k] + result[k][j]);
            }
        }
    }

    result
}

fn part1(graph: &HashMap<String, HashMap<String, u32>>, flows: &HashMap<String, u32>) -> u32 {
    // perform a single search to find the highest amount of pressure that can be released
    search(
        graph,
        flows,
        &valve_bits(flows.keys()),
        "AA",
        30,
        0,
        &mut HashMap::new(),
    )
}

fn part2(graph: &HashMap<String, HashMap<String, u32>>, flows: &HashMap<String, u32>) -> u32 {
    let bits = valve_bits(flows.keys());
    let all_states = 2_u64.pow(bits.len() as u32) - 1;
    let mut cache = HashMap::new();
    // search over all possible states to maximize the sum of person/elephant searches
    // the elephant can only open valves that were not opened by the person
    (0..=all_states)
        .map(|s| {
            search(graph, flows, &bits, "AA", 26, s, &mut cache)
                + search(graph, flows, &bits, "AA", 26, !s & all_states, &mut cache)
        })
        .max()
        .expect("valid-states")
}

fn search(
    graph: &HashMap<String, HashMap<String, u32>>,
    flows: &HashMap<String, u32>,
    bits: &HashMap<String, u64>,
    valve: &str,
    time: u32,
    state: u64,
    cache: &mut HashMap<(String, u32, u64), u32>,
) -> u32 {
    // first see if we have memoized the current state
    let key = (valve.to_string(), time, state);
    if !cache.contains_key(&key) {
        // if not, compute the maximum pressure from the current valve recursively
        let pressure = flows
            .keys()
            .map(|v| {
                // only process the valve if we have enough time to open it
                // and we have not already been there in this search state
                if time > graph[valve][v] + 1 && (state & bits[v]) == 0 {
                    let t = time - graph[valve][v] - 1;
                    // open the current valve for t steps and continue the search
                    t * flows[v] + search(graph, flows, bits, v, t, state | bits[v], cache)
                } else {
                    0
                }
            })
            .max()
            .unwrap_or(0);

        // memoize the result
        cache.insert(key.clone(), pressure);
    }
    cache[&key]
}

fn valve_bits<'a>(valves: impl Iterator<Item = &'a String>) -> HashMap<String, u64> {
    valves
        .enumerate()
        .map(|(i, k)| (k.clone(), 1 << i))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    const DATA: &str = "\
        Valve AA has flow rate=0; tunnels lead to valves DD, II, BB\n\
        Valve BB has flow rate=13; tunnels lead to valves CC, AA\n\
        Valve CC has flow rate=2; tunnels lead to valves DD, BB\n\
        Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE\n\
        Valve EE has flow rate=3; tunnels lead to valves FF, DD\n\
        Valve FF has flow rate=0; tunnels lead to valves EE, GG\n\
        Valve GG has flow rate=0; tunnels lead to valves FF, HH\n\
        Valve HH has flow rate=22; tunnel leads to valve GG\n\
        Valve II has flow rate=0; tunnels lead to valves AA, JJ\n\
        Valve JJ has flow rate=21; tunnel leads to valve II\n\
    ";

    #[test]
    fn test_part1() {
        let (flows, tunnels) = parse(&DATA);
        let graph = floyd_warshall(&tunnels);
        assert_eq!(part1(&graph, &flows), 1651);
    }

    #[test]
    fn test_part2() {
        let (flows, tunnels) = parse(&DATA);
        let graph = floyd_warshall(&tunnels);
        assert_eq!(part2(&graph, &flows), 1707);
    }
}
