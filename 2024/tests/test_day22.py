from aoc2024.day22 import part1, part2

DATA1 = """
1
10
100
2024
""".strip()


DATA2 = """
1
2
3
2024
""".strip()


def test_part1() -> None:
    assert part1(DATA1) == 37327623


def test_part2() -> None:
    assert part2(DATA2) == 23
