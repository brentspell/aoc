import operator
import typing as T


def part1(data: str) -> int:
    return solve(data, [operator.add, operator.mul])


def part2(data: str) -> int:
    return solve(data, [operator.add, operator.mul, concat])


def solve(data: str, operators: list[T.Callable[[int, int], int]]) -> int:
    result = 0
    for total, operands in parse(data):
        totals = {operands[0]}
        for b in operands[1:]:
            totals = {c for a in totals for op in operators if (c := op(a, b)) <= total}
        if total in totals:
            result += total
    return result


def concat(a: int, b: int) -> int:
    return int(str(a) + str(b))


def parse(data: str) -> T.Iterator[tuple[int, list[int]]]:
    for line in data.splitlines():
        t, o = line.split(": ")
        yield int(t), [int(s) for s in o.split()]
