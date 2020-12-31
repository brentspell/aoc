def part1(data):
    return sum(int(x) for x, y in zip(data, data[1:] + data[0]) if x == y)


def part2(data):
    start = len(data) // 2
    return sum(int(x) for x, y in zip(data, data[start:] + data) if x == y)
