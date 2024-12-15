def part1(data: str) -> int:
    grid, dirs = parse(data)

    # find the robot's position and run the simulation
    i, j = next((i, j) for i, r in enumerate(grid) for j, c in enumerate(r) if c == "@")
    for move in dirs:
        u, v = 0, 0
        match move:
            case "^":
                u = -1
            case "v":
                u = +1
            case "<":
                v = -1
            case ">":
                v = +1

        # attempt to move the robot and push any boxes
        if part1_push(grid, i, j, u, v):
            i, j = i + u, j + v

    return sum(
        100 * i + j for i, r in enumerate(grid) for j, c in enumerate(r) if c == "O"
    )


def part1_push(grid: list[list[str]], i: int, j: int, u: int, v: int) -> bool:
    p, q = i + u, j + v
    if grid[p][q] == "#":
        # no pushing walls
        return False
    elif grid[p][q] == "." or part1_push(grid, p, q, u, v):
        # execute the push if we are moving into a free space
        # or if we could recursively move everything in front of the robot
        grid[i][j], grid[p][q] = grid[p][q], grid[i][j]
        return True
    else:
        return False


def part2(data: str) -> int:
    grid, dirs = parse(data)

    # expand the width of the grid
    grid = [
        [d for c in r for d in ("@." if c == "@" else "[]" if c == "O" else c + c)]
        for r in grid
    ]

    # find the robot's position and run the simulation
    i, j = next((i, j) for i, r in enumerate(grid) for j, c in enumerate(r) if c == "@")
    for move in dirs:
        u, v = 0, 0
        match move:
            case "^":
                u = -1
            case "v":
                u = +1
            case "<":
                v = -1
            case ">":
                v = +1

        # first, see if we can recursively savely push everything, before committing
        if part2_can_push(grid, i, j, u, v):
            part2_push(grid, i, j, u, v)
            i, j = i + u, j + v

    return sum(
        100 * i + j for i, r in enumerate(grid) for j, c in enumerate(r) if c == "["
    )


def part2_can_push(grid: list[list[str]], i: int, j: int, u: int, v: int) -> bool:
    p, q = i + u, j + v
    if grid[p][q] == "#":
        # can't push into a wall
        return False
    elif grid[p][q] == ".":
        # can push into a free space
        return True
    elif u == 0 and grid[p][q] in "[]":
        # if we are pushing left/right, recursively check for free spaces
        return part2_can_push(grid, p, q, u, v)
    elif grid[p][q] == "[":
        # if we are moving up/down and we hit the left side of a box,
        # make sure we can move its right side as well
        return part2_can_push(grid, p, q, u, v) and part2_can_push(grid, p, q + 1, u, v)
    elif grid[p][q] == "]":
        # similarly, if we hit the right side of a box, check the right side
        return part2_can_push(grid, p, q, u, v) and part2_can_push(grid, p, q - 1, u, v)
    else:
        return False


def part2_push(
    grid: list[list[str]],
    i: int,
    j: int,
    u: int,
    v: int,
    pair: bool = False,
) -> None:
    p, q = i + u, j + v
    # can't push walls or free spaces
    if grid[i][j] not in "#.":
        # if we are moving up/down and push the side of a box, push the other side
        if u != 0 and not pair:
            if grid[i][j] == "[":
                part2_push(grid, i, j + 1, u, v, pair=True)
            elif grid[i][j] == "]":
                part2_push(grid, i, j - 1, u, v, pair=True)

        # recursively push other items, then swap the current item
        part2_push(grid, p, q, u, v)
        grid[i][j], grid[p][q] = grid[p][q], grid[i][j]


def parse(data: str) -> tuple[list[list[str]], str]:
    grid, dirs = data.split("\n\n", maxsplit=1)
    return [list(s) for s in grid.splitlines()], dirs.replace("\n", "")
