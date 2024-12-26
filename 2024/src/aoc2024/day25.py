def part1(data: str) -> int:
    keys = []
    lcks = []
    for block in data.split("\n\n"):
        grid = block.splitlines()
        heights = [sum(row[j] == "#" for row in grid) - 1 for j in range(len(grid[0]))]
        if grid[0] == "#####":
            lcks.append(heights)
        else:
            keys.append(heights)

    return sum(
        all(a + b <= 5 for a, b in zip(lck, key)) for lck in lcks for key in keys
    )


def part2(data: str) -> int:
    return 0
