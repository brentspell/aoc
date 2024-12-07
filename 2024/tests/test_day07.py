from aoc2024.day07 import part1, part2

DATA = """
190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20
""".strip()


def test_part1() -> None:
    assert part1(DATA) == 3749


def test_part2() -> None:
    assert part2(DATA) == 11387
