from aoc2024.day21 import part1

DATA = """
029A
980A
179A
456A
379A
""".strip()


def test_part1() -> None:
    assert part1(DATA) == 126384
