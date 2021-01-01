import re


def part1(data):
    nodes = parse(data)
    return [n for n in nodes if not any(n in c for _w, c in nodes.values())][0]


def part2(data):
    nodes = parse(data)
    for node in nodes:
        weights = {c: holding(nodes, c) for c in nodes[node][1]}
        # create a children by weight mapping
        freq = {}
        for key, value in weights.items():
            if value not in freq:
                freq[value] = set()
            freq[value].add(key)
        if len(freq) > 1:
            # find the odd child out, and the remaining weight
            # is the expected weight
            (odd,) = min(freq.values(), key=len)
            freq.pop(weights[odd])
            (expect,) = freq.keys()
            # adjust the node's local weight to make it's total match expected
            return nodes[odd][0] + expect - weights[odd]
    return None


def holding(nodes, node):
    weight, children = nodes[node]
    return weight + sum(holding(nodes, n) for n in children)


def parse(data):
    nodes = {}
    for line in data.split("\n"):
        match = re.search(r"(\w+) \((\d+)\)( -> (.*))?", line)
        weight = int(match.group(2))
        children = {c for c in (match.group(4) or "").split(", ") if c}
        nodes[match.group(1)] = (weight, children)
    return nodes
