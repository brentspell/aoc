import typing as T


def part1(data: str) -> int:
    grid, origin = parse(data)

    return len({(i, j) for i, j, _u, _v in walk(grid, origin)})


def part2(data: str) -> int:
    grid, origin = parse(data)

    obstacles = set()
    for i, j, _u, _v in walk(grid, origin):
        if (i, j) != origin and (i, j) not in obstacles:
            grid[i][j] = "#"
            path = set()
            for pos in walk(grid, origin):
                if pos in path:
                    obstacles.add((i, j))
                    break
                path.add(pos)
            grid[i][j] = "."

    return len(obstacles)


def parse(data: str) -> tuple[list[list[str]], tuple[int, int]]:
    grid = [list(s) for s in data.splitlines()]
    return grid, next(
        (i, j)
        for i in range(len(grid))
        for j in range(len(grid[0]))
        if grid[i][j] == "^"
    )


def walk(
    grid: list[list[str]],
    origin: tuple[int, int],
) -> T.Iterator[tuple[int, int, int, int]]:
    M, N = len(grid), len(grid[0])
    i, j, u, v = *origin, -1, 0
    while True:
        yield i, j, u, v

        if not (0 <= (p := i + u) < M and 0 <= (q := j + v) < N):
            break
        elif grid[p][q] != "#":
            i, j = p, q
        elif u == -1:
            u, v = 0, 1
        elif u == 1:
            u, v = 0, -1
        elif v == -1:
            u, v = -1, 0
        elif v == 1:
            u, v = 1, 0
