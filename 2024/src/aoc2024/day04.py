def part1(data: str) -> int:
    rows = data.splitlines()
    return sum(
        1
        for i in range(len(rows))
        for j in range(len(rows[0]))
        if rows[i][j] == "X"
        for u in (-1, 0, 1)
        if 0 <= i + 3 * u < len(rows)
        for v in (-1, 0, 1)
        if 0 <= j + 3 * v < len(rows[0])
        if rows[i + u][j + v] == "M"
        if rows[i + 2 * u][j + 2 * v] == "A"
        if rows[i + 3 * u][j + 3 * v] == "S"
    )


def part2(data: str) -> int:
    rows = data.splitlines()
    return sum(
        1
        for i in range(len(rows))
        if 0 <= i - 1 and i + 1 < len(rows)
        for j in range(len(rows[0]))
        if 0 <= j - 1 and j + 1 < len(rows[0])
        if rows[i][j] == "A"
        if (rows[i - 1][j - 1] == "M" and rows[i + 1][j + 1] == "S")
        or (rows[i - 1][j - 1] == "S" and rows[i + 1][j + 1] == "M")
        if (rows[i - 1][j + 1] == "M" and rows[i + 1][j - 1] == "S")
        or (rows[i - 1][j + 1] == "S" and rows[i + 1][j - 1] == "M")
    )
