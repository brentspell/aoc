def part1(data: str, save: int = 100) -> int:
    grid, beg, end = parse(data)

    # first, compute the non-cheating costs of each point on the path
    costs = path_costs(grid, beg, end)

    # use dfs to calculate the cheating costs
    cheats = 0
    stack = [(*beg, *beg, 0, False)]
    while stack:
        i, j, p, q, n, c = stack.pop()

        # if we are cheating and we land back on the path,
        # see if we have saved enough time with the cheat
        if c and grid[i][j] != "#":
            if n + costs[(i, j)] <= costs[beg] - save:
                cheats += 1
            continue

        # expand the search, allowing for exactly one cheat
        for u, v in [(i - 1, j), (i + 1, j), (i, j - 1), (i, j + 1)]:
            if (u, v) != (p, q) and 0 <= u < len(grid) and 0 <= v < len(grid[0]):
                if grid[u][v] != "#":
                    stack.append((u, v, i, j, n + 1, c))
                elif not c:
                    stack.append((u, v, i, j, n + 1, True))

    return cheats


def part2(data: str, save: int = 100) -> int:
    grid, beg, end = parse(data)

    # first, compute the non-cheating costs of each point on the path
    costs = path_costs(grid, beg, end)

    # use dfs to calculate the cheating costs
    cheats = 0
    stack = [(*beg, *beg, 0)]
    while stack:
        i, j, p, q, n = stack.pop()
        if (i, j) == end:
            break

        # try to cheat at this point on the path
        # find all coordinates within the maximum allowed cheating L1 distance
        # for any points that land on the path, see if the cheat saved enough time
        # these cheats are all unique, since (i, j) is unique on the path,
        # and we only check (u, v) once
        for u in range(i - 20, i + 20 + 1):
            for v in range(j - 20, j + 20 + 1):
                if (d := abs(i - u) + abs(j - v)) <= 20:
                    if 0 <= u < len(grid) and 0 <= v < len(grid[0]):
                        if grid[u][v] != "#":
                            if n + d + costs[(u, v)] <= costs[beg] - save:
                                cheats += 1

        # expand the search along the path
        for u, v in [(i - 1, j), (i + 1, j), (i, j - 1), (i, j + 1)]:
            if (u, v) != (p, q) and grid[u][v] != "#":
                stack.append((u, v, i, j, n + 1))

    return cheats


def path_costs(
    grid: list[str],
    beg: tuple[int, int],
    end: tuple[int, int],
) -> dict[tuple[int, int], int]:
    costs = {}

    # use dfs (backwards) to calculate costs from every point on the path to the end
    stack = [(*end, *end, 0)]
    while stack:
        i, j, p, q, n = stack.pop()

        # save the cost from the current point to the end, stop at the beginning
        costs[(i, j)] = n
        if (i, j) == beg:
            break

        # expand the search along the path
        for u, v in [(i - 1, j), (i + 1, j), (i, j - 1), (i, j + 1)]:
            if (u, v) != (p, q) and grid[u][v] != "#":
                stack.append((u, v, i, j, n + 1))

    return costs


def parse(data: str) -> tuple[list[str], tuple[int, int], tuple[int, int]]:
    grid = data.splitlines()
    beg = next((i, j) for i, r in enumerate(grid) for j, c in enumerate(r) if c == "S")
    end = next((i, j) for i, r in enumerate(grid) for j, c in enumerate(r) if c == "E")
    return grid, beg, end
