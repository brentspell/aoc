import operator
from functools import reduce

N = 128


def part1(data):
    return sum(
        1 if k & (1 << (7 - j)) else 0
        for i in range(N)
        for k in knothash(f"{data}-{i}")
        for j in range(8)
    )


def part2(data):
    grid = [
        [
            "#" if k & (1 << (7 - j)) else "."
            for k in knothash(f"{data}-{i}")
            for j in range(8)
        ]
        for i in range(N)
    ]

    done = set()
    regions = 0
    for i, row in enumerate(grid):
        for j, col in enumerate(row):
            if col == "#" and (i, j) not in done:
                regions += 1
                search(grid, done, i, j)

    return regions


def search(grid, done, i, j):
    todo = [(i, j)]
    while todo:
        x, y = todo.pop()
        done.add((x, y))
        for x, y in [(x - 1, y), (x + 1, y), (x, y - 1), (x, y + 1)]:
            if 0 <= x < N and 0 <= y < N and grid[x][y] == "#":
                if (x, y) not in done:
                    todo.append((x, y))


def knothash(data):
    lengths = [ord(c) for c in data] + [17, 31, 73, 47, 23]
    knot = list(range(256))

    offset = 0
    skip = 0
    for _ in range(64):
        for length in lengths:
            span = (knot + knot)[offset:][:length]
            for i, x in enumerate(reversed(span), offset):
                knot[i % len(knot)] = x
            offset = (offset + length + skip) % len(knot)
            skip += 1

    return [reduce(operator.xor, knot[i:][:16]) for i in range(0, len(knot), 16)]
