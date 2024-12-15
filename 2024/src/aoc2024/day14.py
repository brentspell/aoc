from dataclasses import dataclass


@dataclass
class Robot:
    px: int
    py: int
    vx: int
    vy: int


def part1(data: str, w: int = 101, h: int = 103) -> int:
    robots = parse(data)

    # advance the simulation by 100secs
    for _ in range(100):
        for r in robots:
            r.px = (r.px + r.vx) % w
            r.py = (r.py + r.vy) % h

    # count the robots in each quadrant
    cx, cy = w // 2, h // 2
    q1 = sum(0 <= r.px < cx and 0 <= r.py < cy for r in robots)
    q2 = sum(cx < r.px < w and 0 <= r.py < cy for r in robots)
    q3 = sum(0 <= r.px < cx and cy < r.py < h for r in robots)
    q4 = sum(cx < r.px < w and cy < r.py < h for r in robots)
    return q1 * q2 * q3 * q4


def part2(data: str, w: int = 101, h: int = 103) -> int:
    robots = parse(data)

    # advance the simulation until we find the tree
    t = 0
    while True:
        # advance the simulation
        pos = set()
        for r in robots:
            r.px = (r.px + r.vx) % w
            r.py = (r.py + r.vy) % h
            pos.add((r.px, r.py))
        t += 1

        # use dfs to count neighbors
        while len(pos) > 100:
            (i, j) = next(iter(pos))
            pos.remove((i, j))

            # find the neighborhood of the current robot
            stack = [(i, j)]
            count = 0
            while stack:
                i, j = stack.pop()
                count += 1
                for u, v in [(i - 1, j), (i + 1, j), (i, j - 1), (i, j + 1)]:
                    if (u, v) in pos:
                        pos.remove((u, v))
                        stack.append((u, v))

            # if we find a neighborhood of, say, more than 100 robots, we are done
            if count > 100:
                return t


def parse(data: str) -> list[Robot]:
    return [
        Robot(*(int(u) for t in s[2:].split(" v=") for u in t.split(",")))
        for s in data.splitlines()
    ]
