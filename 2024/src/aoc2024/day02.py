def part1(data: str) -> int:
    return sum(is_safe(ls) for ls in parse(data))


def part2(data: str) -> int:
    return sum(
        is_safe(ls)
        or any(
            is_safe([ls[i] for i in range(len(ls)) if i != j]) for j in range(len(ls))
        )
        for ls in parse(data)
    )


def parse(data: str) -> list[list[int]]:
    return [[int(v) for v in s.split()] for s in data.splitlines()]


def is_safe(levels: list[int]) -> bool:
    sa = 0
    for v1, v2 in zip(levels[:-1], levels[1:]):
        d = v2 - v1
        if not (1 <= abs(d) <= 3):
            return False

        s = 1 if d > 0 else -1
        if sa != 0 and s != sa:
            return False
        sa = s

    return True
