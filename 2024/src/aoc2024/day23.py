import collections
import typing as T


def part1(data: str) -> int:
    edges = parse(data)

    return len(
        {
            frozenset([v0, v1, v2])
            for v0 in edges.keys()
            if v0.startswith("t")
            for v1 in edges[v0]
            for v2 in edges[v1]
            if v0 in edges[v2]
        }
    )


def part2(data: str) -> str:
    edges = parse(data)

    # find the longest clique and sort its vertices
    clique = max(
        cliques(edges, set(), set(edges.keys()), set()),
        key=len,
    )
    return ",".join(sorted(clique))


def cliques(
    edges: dict[str, set[str]],
    r: set[str],
    p: set[str],
    x: set[str],
) -> T.Iterator[set[str]]:
    # Bron–Kerbosch algorithm for maximal clique detection with pivoting
    if not p and not x:
        yield r
    else:
        for v in min((p - edges[u] for u in p | x), key=len):
            yield from cliques(
                edges,
                r | {v},
                p & edges[v],
                x & edges[v],
            )
            p.remove(v)
            x.add(v)


def parse(data: str) -> dict[str, set[str]]:
    edges = collections.defaultdict(set)
    for s in data.splitlines():
        p, q = s.split("-")
        edges[p].add(q)
        edges[q].add(p)
    return edges
