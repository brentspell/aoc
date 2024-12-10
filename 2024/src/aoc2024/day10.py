import typing as T


def part1(data: str) -> int:
    return len(set(search(data)))


def part2(data: str) -> int:
    return sum(1 for _ in search(data))


def search(data: str) -> T.Iterator[tuple[int, int, int, int]]:
    grid = [[int(c) for c in s] for s in data.splitlines()]

    # find all of the trailheads
    for i in range(len(grid)):
        for j in range(len(grid[0])):
            if grid[i][j] == 0:
                # dfs from this trailhead to each goal node
                stack = [(i, j)]
                while stack:
                    p, q = stack.pop()
                    if grid[p][q] == 9:
                        yield i, j, p, q

                    # expand to neighbors with height + 1
                    for r, s in [(p - 1, q), (p + 1, q), (p, q - 1), (p, q + 1)]:
                        if 0 <= r < len(grid) and 0 <= s < len(grid[0]):
                            if grid[r][s] == grid[p][q] + 1:
                                stack.append((r, s))
