use std::collections::{HashMap, VecDeque};

pub fn solve() {
    let data = std::fs::read_to_string("data/day20.txt").unwrap();

    // part 1
    println!("part 1: {}", part1(&data));

    // part 2
    println!("part 2: {}", part2(&data));
}

fn part1(data: &str) -> u64 {
    // parse the set of modules as a graph
    let modules: HashMap<_, _> = data
        .lines()
        .map(|l| {
            let m = Module::parse(l);
            (m.name.clone(), m)
        })
        .collect();

    // initialize the state of each flipflop
    let mut flipflops: HashMap<&str, _> = modules
        .values()
        .filter_map(|m| match m.typ {
            ModuleType::FlipFlop => Some((&m.name as &str, false)),
            _ => None,
        })
        .collect();

    // initialize the state of each conjunction
    let mut conjunctions: HashMap<&str, _> = modules
        .values()
        .filter_map(|m| match m.typ {
            ModuleType::Conjunction => {
                let c: HashMap<&str, _> = modules
                    .values()
                    .filter_map(|o| {
                        if o.outputs.contains(&m.name) {
                            Some((&o.name as &str, false))
                        } else {
                            None
                        }
                    })
                    .collect();
                Some((&m.name as &str, c))
            }
            _ => None,
        })
        .collect();

    // run the requested number of simulations
    let mut lo = 0;
    let mut hi = 0;
    let mut queue = VecDeque::new();
    for _ in 0..1000 {
        // use BFS from the broadcaster node to propagate the pulses through the network
        queue.push_back(("button", "broadcaster", false));
        while let Some((from, to, value)) = queue.pop_front() {
            // calculate the total number of low/high transitions
            if value {
                lo += 1;
            } else {
                hi += 1;
            }

            if let Some(m) = modules.get(to) {
                match m.typ {
                    // for the broadcaster node, trigger a low pulse to all connected nodes
                    ModuleType::Broadcaster => {
                        for o in m.outputs.iter() {
                            queue.push_back((&m.name, o, value));
                        }
                    }

                    // for a flipflop, if the pulse is low, flip the internal state and
                    // trigger the resulting state as a pulse to all connected nodes
                    ModuleType::FlipFlop => {
                        if !value {
                            let v = flipflops.get_mut(to).unwrap();
                            *v = !*v;
                            for o in m.outputs.iter() {
                                queue.push_back((&m.name, o, *v));
                            }
                        }
                    }

                    // for a conjunction, update the node's local slot for the sender, and
                    // if all senders have sent this node high pulses, send a low pulse
                    // to all connected nodes
                    ModuleType::Conjunction => {
                        let c = conjunctions.get_mut(to).unwrap();
                        c.insert(from, value);
                        let v = c.values().all(|&v| v);
                        for o in m.outputs.iter() {
                            queue.push_back((&m.name, o, !v));
                        }
                    }
                }
            }
        }
    }

    lo * hi
}

fn part2(data: &str) -> u64 {
    // parse the set of modules as a graph
    let modules: HashMap<_, _> = data
        .lines()
        .map(|l| {
            let m = Module::parse(l);
            (m.name.clone(), m)
        })
        .collect();

    // we make the assumption here that all flipflops have inbound connections
    // only from the broadcaster node or other flipflops
    // so we can track the transition cycle of each flipflop by its distance
    // from the broadcaster, taken to the power of 2
    // these distances can be found with DFS
    let mut flipflops = HashMap::new();
    let mut stack = vec![("broadcaster", 0_u64)];
    while let Some((from, depth)) = stack.pop() {
        if let Some(m) = modules.get(from) {
            // we consider only transitions from the broadcaster or other flipflops
            // there are conjunction->flipflop transitions, but they exist only to
            // reset the conjunction's cycle and are thus not relevant
            if matches!(m.typ, ModuleType::Broadcaster | ModuleType::FlipFlop) {
                for o in m.outputs.iter() {
                    if let Some(o) = modules.get(o) {
                        if matches!(o.typ, ModuleType::FlipFlop) {
                            flipflops.insert(&o.name as &str, 1 << depth);
                            stack.push((&o.name as &str, depth + 1));
                        }
                    }
                }
            }
        }
    }

    // in order to process the conjunction nodes (and the goal), we first create
    // a dependency tree that inverts the module graph
    let deps: HashMap<&str, Vec<&str>> = modules
        .values()
        .flat_map(|m| &m.outputs)
        .map(|s| {
            (
                &s as &str,
                modules
                    .values()
                    .filter_map(|m| {
                        if m.outputs.contains(s) {
                            Some(&m.name as &str)
                        } else {
                            None
                        }
                    })
                    .collect::<Vec<&str>>(),
            )
        })
        .collect();

    // now we can recursively calculate the total button presses
    calculate_presses(&modules, &deps, &flipflops, "rx")
}

fn calculate_presses(
    modules: &HashMap<String, Module>,
    deps: &HashMap<&str, Vec<&str>>,
    flipflops: &HashMap<&str, u64>,
    node: &str,
) -> u64 {
    // if we reach a flipflop node, we have already calculated the cycle length, so use it
    if let Some(&p) = flipflops.get(node) {
        return p;
    }

    // otherwise, we are on a conjunction node, so combine the cycle lengths of its children
    let mut r = 0;
    for &o in deps[node].iter().rev() {
        let p = calculate_presses(modules, deps, flipflops, o);
        match modules[o].typ {
            // if the conjunction node has flipflops on its inbound edges,
            // the cycle length of the node is the sum of the cycle lengths of the flipflops,
            // which is when all of the flipflops will have the same state
            ModuleType::FlipFlop => r += p,

            // for inbound conjunctions, the cycle length is the least common multiple of
            // the cycle lengths of the dependency conjunctions, which is when all of the
            // conjunctions will have the same state
            ModuleType::Conjunction => r = lcm(if r == 0 { 1 } else { r }, p),

            _ => panic!("invalid module type"),
        }
    }
    r
}

#[derive(Clone, Debug)]
struct Module {
    typ: ModuleType,
    name: String,
    outputs: Vec<String>,
}

#[derive(Clone, Copy, Debug)]
enum ModuleType {
    Broadcaster,
    FlipFlop,
    Conjunction,
}

impl Module {
    fn parse(s: &str) -> Self {
        let s: Vec<_> = s.split(" -> ").collect();
        let o = s[1].split(", ").map(|s| s.to_string()).collect();
        match &s[0][0..1] {
            "%" => Self {
                typ: ModuleType::FlipFlop,
                name: s[0][1..].to_string(),
                outputs: o,
            },
            "&" => Self {
                typ: ModuleType::Conjunction,
                name: s[0][1..].to_string(),
                outputs: o,
            },
            _ => Self {
                typ: ModuleType::Broadcaster,
                name: s[0].to_string(),
                outputs: o,
            },
        }
    }
}

fn gcd(a: u64, b: u64) -> u64 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

fn lcm(a: u64, b: u64) -> u64 {
    a * b / gcd(a, b)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let data = "\
            broadcaster -> a, b, c\n\
            %a -> b\n\
            %b -> c\n\
            %c -> inv\n\
            &inv -> a\n\
        ";
        assert_eq!(part1(data), 32000000);

        let data = "\
            broadcaster -> a\n\
            %a -> inv, con\n\
            &inv -> b\n\
            %b -> con\n\
            &con -> output\n\
        ";
        assert_eq!(part1(data), 11687500);
    }

    #[test]
    fn test_part2() {}
}
