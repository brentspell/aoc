import re


def part1(data: str) -> int:
    return sum(int(a) * int(b) for a, b in re.findall(r"mul\((\d+),(\d+)\)", data))


def part2(data: str) -> int:
    enabled = True
    total = 0
    for m in re.findall(r"(do)\(\)|(don't)\(\)|(mul)\((\d+),(\d+)\)", data):
        do, dont, mul, a, b = m
        if do:
            enabled = True
        elif dont:
            enabled = False
        elif mul and enabled:
            total += int(a) * int(b)
    return total
