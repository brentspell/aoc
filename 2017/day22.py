ORIENT = {
    "N": {"L": "W", "R": "E"},
    "S": {"L": "E", "R": "W"},
    "E": {"L": "N", "R": "S"},
    "W": {"L": "S", "R": "N"},
}

MOVE = {"N": (0, 1), "S": (0, -1), "E": (1, 0), "W": (-1, 0)}

REVERSE = {"N": "S", "S": "N", "E": "W", "W": "E"}


def part1(data):
    infected = parse(data)

    x, y = 0, 0
    orient = "N"
    result = 0
    for _ in range(10000):
        orient = ORIENT[orient]["R" if (x, y) in infected else "L"]
        if (x, y) in infected:
            infected.remove((x, y))
        else:
            infected.add((x, y))
            result += 1
        dx, dy = MOVE[orient]
        x += dx
        y += dy
    return result


def part2(data, iterations=10000000):
    infected = parse(data)
    weakened = set()
    flagged = set()

    x, y = 0, 0
    orient = "N"
    result = 0
    for _ in range(iterations):
        if (x, y) in infected:
            orient = ORIENT[orient]["R"]
        elif (x, y) in flagged:
            orient = REVERSE[orient]
        elif (x, y) not in weakened:
            orient = ORIENT[orient]["L"]

        if (x, y) in infected:
            infected.remove((x, y))
            flagged.add((x, y))
        elif (x, y) in flagged:
            flagged.remove((x, y))
        elif (x, y) in weakened:
            weakened.remove((x, y))
            infected.add((x, y))
            result += 1
        else:
            weakened.add((x, y))

        dx, dy = MOVE[orient]
        x += dx
        y += dy
    return result


def parse(data):
    rows = data.split("\n")
    return {
        (x - len(rows) // 2, len(rows) // 2 - y)
        for y, row in enumerate(rows)
        for x, col in enumerate(row)
        if col == "#"
    }
