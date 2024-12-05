import collections
import functools


def part1(data: str) -> int:
    deps, upds = parse(data)

    return sum(u[len(u) // 2] for u in upds if is_valid(u, deps))


def part2(data: str) -> int:
    deps, upds = parse(data)

    def compare(x1: int, x2: int) -> int:
        return -1 if x1 in deps[x2] else 1 if x2 in deps[x1] else 0

    return sum(
        sorted(u, key=functools.cmp_to_key(compare))[len(u) // 2]
        for u in upds
        if not is_valid(u, deps)
    )


def is_valid(upd: list[int], deps: dict[int, set[int]]) -> bool:
    invalid = set()
    for x in upd:
        if x in invalid:
            return False
        invalid |= deps[x]
    return True


def parse(data: str) -> tuple[dict[int, set[int]], list[list[int]]]:
    s1, s2 = data.split("\n\n")

    deps = collections.defaultdict(set)
    for s in s1.splitlines():
        v1, v2 = s.split("|")
        deps[int(v2)].add(int(v1))

    upds = [[int(v) for v in s.split(",")] for s in s2.splitlines()]

    return deps, upds
