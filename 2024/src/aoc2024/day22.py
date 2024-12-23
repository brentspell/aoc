import collections
import typing as T


def part1(data: str) -> int:
    seeds = [int(s) for s in data.splitlines()]
    return sum(last(take(derive(s), 2000)) for s in seeds)


def part2(data: str) -> int:
    sums: dict[tuple[int, ...], int] = collections.defaultdict(int)
    for seed in [int(s) for s in data.splitlines()]:
        done = set()
        diff: tuple[int, ...] = ()
        prev = seed % 10
        for price in take(derive(seed), 2000):
            price %= 10
            diff = diff[-3:] + (price - prev,)
            prev = price
            if len(diff) == 4 and diff not in done:
                done.add(diff)
                sums[diff] += price
    return max(sums.values())


def derive(s: int) -> T.Iterator[int]:
    while True:
        s = ((s * 64) ^ s) % 16777216
        s = ((s // 32) ^ s) % 16777216
        s = ((s * 2048) ^ s) % 16777216
        yield s


def take(s: T.Iterator[int], n: int) -> T.Iterator[int]:
    for _ in range(n):
        yield next(s)


def last(s: T.Iterator[int]) -> int:
    for x in s:
        pass
    return x
