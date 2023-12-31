use rand::prelude::*;
use std::collections::{HashMap, HashSet};
use std::sync::atomic;

pub fn solve() {
    let data = std::fs::read_to_string("data/day25.txt").unwrap();

    // part 1
    println!("part 1: {}", part1(&data));
}

fn part1(data: &str) -> u64 {
    let graph = Graph::parse(data);
    loop {
        // run the randomized min-cut algorithm to merge all edges
        let graph = fastmincut(graph.clone());

        // if we got down to a cut of size 3, we found the desired pair of multi-vertices,
        // so calculate the product of their counts
        if graph.edges.values().next().unwrap().count == 3 {
            return graph.verts.values().map(|v| v.count).product();
        }
    }
}

fn fastmincut(graph: Graph) -> Graph {
    // karger-stein heuristic: https://en.wikipedia.org/wiki/Karger%27s_algorithm#Karger%E2%80%93Stein_algorithm
    // instead of running the contraction once all the way down, this heuristic runs a tree
    // of contractions, retrying a graph once at each level before starting from scratch
    if graph.verts.len() <= 6 {
        // for a small enough graph, just go ahead and contract completely
        contract(graph, 2)
    } else {
        // split the graph into two contractions of at most ceil(1 + N / sqrt(2)) vertices
        // and continue hte descent
        let t = (1.0 + graph.verts.len() as f64 / std::f64::consts::SQRT_2).ceil() as usize;
        let g1 = fastmincut(contract(graph.clone(), t));
        let g2 = fastmincut(contract(graph, t));

        // return the contraction with the smallest edge cut
        let e1 = g1.edges.values().map(|e| e.count).min().unwrap();
        let e2 = g2.edges.values().map(|e| e.count).min().unwrap();
        if e1 < e2 {
            g1
        } else {
            g2
        }
    }
}

fn contract(mut graph: Graph, size: usize) -> Graph {
    // karger's contraction: https://en.wikipedia.org/wiki/Karger%27s_algorithm#Contraction_algorithm
    // we repeatedly select a random multi-edge to remove, and merge the two
    // vertices into a single vertex, until we get to the target graph size
    while graph.verts.len() > size {
        // choose an edge to remove, and remove its associated vertices
        let victim = *graph.edges.keys().choose(&mut rand::thread_rng()).unwrap();
        let e = graph.edges.remove(&victim).unwrap();
        let v1 = graph.verts.remove(&e.v1).unwrap();
        let v2 = graph.verts.remove(&e.v2).unwrap();

        // detach the removed vertices from their other neighbors,
        // and collect the distinct count of those edges so they
        // can be re-attached to the new vertex
        let mut es = HashMap::new();
        for v in [&v1, &v2] {
            for ve in v.edges.iter().copied() {
                if let Some(eo) = graph.edges.remove(&ve) {
                    let o = if eo.v1 == e.v1 || eo.v1 == e.v2 {
                        eo.v2
                    } else {
                        eo.v1
                    };
                    graph.verts.get_mut(&o).unwrap().edges.remove(&ve);
                    *es.entry(o).or_insert(0) += eo.count;
                }
            }
        }

        // insert new edges for all the detached neighbors to the new vertex
        let v = gen_id();
        let mut ves = HashSet::new();
        for (o, n) in es {
            let e = gen_id();
            graph.edges.insert(
                e,
                Edge {
                    count: n,
                    v1: v,
                    v2: o,
                },
            );
            graph.verts.get_mut(&o).unwrap().edges.insert(e);
            ves.insert(e);
        }

        // create the new vertex, aggregating the counts from the removed vertices
        graph.verts.insert(
            v,
            Vertex {
                count: v1.count + v2.count,
                edges: ves,
            },
        );
    }

    graph
}

#[derive(Clone, Debug)]
struct Graph {
    verts: HashMap<u64, Vertex>,
    edges: HashMap<u64, Edge>,
}

#[derive(Clone, Debug)]
struct Vertex {
    count: u64,
    edges: HashSet<u64>,
}

#[derive(Clone, Debug)]
struct Edge {
    count: u64,
    v1: u64,
    v2: u64,
}

impl Graph {
    fn parse(data: &str) -> Graph {
        let mut verts = HashMap::new();
        let mut edges = HashMap::new();
        let mut ids = HashMap::new();
        for line in data.lines() {
            let s: Vec<_> = line.split(": ").collect();

            // generate a unique id for the vertices if not yet seen
            let v1 = *ids.entry(s[0]).or_insert_with(gen_id);
            for s in s[1].split_whitespace() {
                let v2 = *ids.entry(s).or_insert_with(gen_id);

                // create an edge for the pair of vertices
                let e = gen_id();
                edges.insert(e, Edge { count: 1, v1, v2 });

                // create the vertices and attach the edge to them
                for v in [v1, v2] {
                    verts
                        .entry(v)
                        .or_insert_with(|| Vertex {
                            count: 1,
                            edges: HashSet::new(),
                        })
                        .edges
                        .insert(e);
                }
            }
        }

        Graph { verts, edges }
    }
}

// a simple id generator, to avoid storing strings in the graph
// and to support dynamic vertices/edges
fn gen_id() -> u64 {
    lazy_static! {
        static ref ID: atomic::AtomicU64 = atomic::AtomicU64::new(0);
    }

    ID.fetch_add(1, atomic::Ordering::Relaxed)
}

#[cfg(test)]
mod tests {
    use super::*;

    const DATA: &str = "\
        jqt: rhn xhk nvd\n\
        rsh: frs pzl lsr\n\
        xhk: hfx\n\
        cmg: qnr nvd lhk bvb\n\
        rhn: xhk bvb hfx\n\
        bvb: xhk hfx\n\
        pzl: lsr hfx nvd\n\
        qnr: nvd\n\
        ntq: jqt hfx bvb xhk\n\
        nvd: lhk\n\
        lsr: lhk\n\
        rzs: qnr cmg lsr rsh\n\
        frs: qnr lhk lsr\n\
    ";

    #[test]
    fn test_part1() {
        assert_eq!(part1(DATA), 54);
    }
}
