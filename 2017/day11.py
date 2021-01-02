STEP = {
    "n": (0, 1, -1),
    "ne": (1, 0, -1),
    "se": (1, -1, 0),
    "s": (0, -1, 1),
    "sw": (-1, 0, 1),
    "nw": (-1, 1, 0),
}


def part1(data):
    point = [0, 0, 0]
    for step in data.split(","):
        point = take(step, point)
    return norm(point)


def part2(data):
    point = [0, 0, 0]
    max_ = 0
    for step in data.split(","):
        point = take(step, point)
        max_ = max(max_, norm(point))
    return max_


def take(step, point):
    return [x + dx for x, dx in zip(point, STEP[step])]


def norm(point):
    return sum(abs(x) for x in point) // 2
