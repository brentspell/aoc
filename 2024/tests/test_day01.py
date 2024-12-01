from aoc2024.day01 import part1, part2

DATA = """
3   4
4   3
2   5
1   3
3   9
3   3
""".strip()


def test_part1() -> None:
    assert part1(DATA) == 11


def test_part2() -> None:
    assert part2(DATA) == 31
