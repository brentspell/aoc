import re
import typing as T


def part1(data: str) -> int:
    # search for a combination of button presses that finds the prize
    # limit the search to a maximum of 100 a/b button presses
    return sum(
        min(
            (
                a * 3 + b * 1
                for a in range(min(px // ax + 1, py // ay + 1, 100))
                for b in range(min(px // bx + 1, py // by + 1, 100))
                if a * ax + b * bx == px and a * ay + b * by == py
            ),
            default=0,
        )
        for ax, bx, px, ay, by, py in parse(data)
    )


def part2(data: str) -> int:
    tokens = 0
    for ax, bx, px, ay, by, py in parse(data):
        c, d, x = ax, bx, px
        f, g, y = ay, by, py
        x += 10000000000000
        y += 10000000000000

        # solve 2 diophantine equations for 2 unknowns: a, b
        # ac + bd = x
        # af + bg = y
        # b = (y - af)/g
        # ac + d(y - af)/g = x
        # ac + dy/g - a(df/g) = x
        # a(c - df/g) = x - dy/g
        # a = (x - dy/g)/(c - df/g)
        # a = (xg - dy)/(cg - df)
        # b = (y - af)/g
        an = x * g - d * y
        ad = c * g - d * f

        # check for a solution and solve for a, b
        if an % ad == 0:
            a = an // ad
            bn = y - a * f
            bd = g
            if bn % bd == 0:
                b = bn // bd
                tokens += a * 3 + b * 1

    return tokens


def parse(data: str) -> T.Iterator[tuple[int, int, int, int, int, int]]:
    lines = data.splitlines()
    for i in range(0, len(lines), 4):
        m1 = re.match(r"Button A: X\+(\d+), Y\+(\d+)", lines[i + 0])
        m2 = re.match(r"Button B: X\+(\d+), Y\+(\d+)", lines[i + 1])
        m3 = re.match(r"Prize: X=(\d+), Y=(\d+)", lines[i + 2])
        assert m1 is not None
        assert m2 is not None
        assert m3 is not None

        ax, ay = [int(m1.group(i + 1)) for i in range(2)]
        bx, by = [int(m2.group(i + 1)) for i in range(2)]
        px, py = [int(m3.group(i + 1)) for i in range(2)]

        yield ax, bx, px, ay, by, py
