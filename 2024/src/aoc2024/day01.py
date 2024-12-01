import collections


def part1(data: str) -> int:
    l1, l2 = parse(data)
    return sum(abs(v2 - v1) for v1, v2 in zip(sorted(l1), sorted(l2)))


def part2(data: str) -> int:
    l1, l2 = parse(data)
    counter = collections.Counter(l2)
    return sum(v * counter[v] for v in l1)


def parse(data: str) -> zip:
    return zip(*[[int(v) for v in s.split()] for s in data.splitlines()])
