def part1(data):
    data = [[int(c) for c in s.split()] for s in data.split("\n")]
    return sum(max(r) - min(r) for r in data)


def part2(data):
    data = [[int(c) for c in s.split()] for s in data.split("\n")]
    return sum(x // y for r in data for x in r for y in r if x != y and x % y == 0)
