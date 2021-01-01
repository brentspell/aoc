def part1(data):
    target = int(data)
    value = 1
    radius = 0
    x, y = 0, 0
    while value < target:
        x += 1
        value += 1
        radius += 1
        while value < target and y < radius:
            y += 1
            value += 1
        while value < target and x > -radius:
            x -= 1
            value += 1
        while value < target and y > -radius:
            y -= 1
            value += 1
        while value < target and x < radius:
            x += 1
            value += 1
    return abs(x) + abs(y)


def part2(data):
    x, y = 0, 0
    dx, dy = 1, 0
    radius = 1
    value = 1
    values = {(x, y): value}
    while value < int(data):
        x += dx
        y += dy
        if x == radius:
            # head right to next square, then up
            radius += 1
            dx, dy = 0, 1
        elif y == radius:
            # head left
            y -= 1
            dx, dy = -1, 0
        elif x == -radius:
            # head down
            x += 1
            dx, dy = 0, -1
        elif y == -radius:
            # head right
            y += 1
            dx, dy = 1, 0
        # sum the filled neighboring squares
        value = 0
        for i in [-1, 0, 1]:
            for j in [-1, 0, 1]:
                if i != 0 or j != 0:
                    value += values.get((x + i, y + j), 0)
        values[(x, y)] = value
    return value
