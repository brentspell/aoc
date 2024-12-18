import collections


def part1(data: str, dim: int = 70, corrupt: int = 1024) -> int:
    done = set(parse(data)[:corrupt])

    # bfs to find the shortest unweighted path
    done.add((0, 0))
    queue = collections.deque([(0, 0, 0)])
    while queue:
        i, j, c = queue.pop()
        if (i, j) == (dim, dim):
            return c

        # expand the search to uncorrupted neighbors not already passed
        for p, q in [(i - 1, j), (i + 1, j), (i, j - 1), (i, j + 1)]:
            if 0 <= p <= dim and 0 <= q <= dim and (p, q) not in done:
                done.add((p, q))
                queue.appendleft((p, q, c + 1))

    assert False


def part2(data: str, dim: int = 70) -> str:
    corrupt = parse(data)

    # try every subsequence of corrupted bytes to find the one that blocks the exit
    for k in range(len(corrupt)):
        done = set(corrupt[: k + 1])

        # use dfs to determine whether a path exists
        done.add((0, 0))
        stack = [(0, 0)]
        found = False
        while stack:
            i, j = stack.pop()
            if (i, j) == (dim, dim):
                found = True
                break

            # expand the search to uncorrupted neighbors not already passed
            for p, q in [(i - 1, j), (i + 1, j), (i, j - 1), (i, j + 1)]:
                if 0 <= p <= dim and 0 <= q <= dim and (p, q) not in done:
                    done.add((p, q))
                    stack.append((p, q))

        # if no path was found, we are done
        if not found:
            x, y = corrupt[k]
            return f"{x},{y}"

    assert False


def parse(data: str) -> list[tuple[int, ...]]:
    return [tuple([int(c) for c in s.split(",")]) for s in data.splitlines()]
