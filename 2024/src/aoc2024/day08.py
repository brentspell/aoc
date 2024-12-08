import collections


def part1(data: str) -> int:
    grid, ants = parse(data)

    antis = set()
    for group in ants.values():
        for i1, j1 in group:
            for i2, j2 in group:
                if (i1, j1) != (i2, j2):
                    i = 2 * i1 - i2
                    j = 2 * j1 - j2
                    if 0 <= i < len(grid) and 0 <= j < len(grid[0]):
                        antis.add((i, j))

    return len(antis)


def part2(data: str) -> int:
    grid, ants = parse(data)

    antis = set()
    for group in ants.values():
        for i1, j1 in group:
            for i2, j2 in group:
                if (i1, j1) != (i2, j2):
                    i, j = i1, j1
                    di, dj = i1 - i2, j1 - j2
                    while 0 <= i < len(grid) and 0 <= j < len(grid[0]):
                        antis.add((i, j))
                        i += di
                        j += dj

    return len(antis)


def parse(data: str) -> tuple[list[str], dict[str, list[tuple[int, int]]]]:
    grid = data.splitlines()
    ants = collections.defaultdict(list)
    for i in range(len(grid)):
        for j in range(len(grid[0])):
            if (f := grid[i][j]) != ".":
                ants[f].append((i, j))

    return grid, ants
