from aoc2024.day09 import part1, part2

DATA = "2333133121414131402".strip()


def test_part1() -> None:
    assert part1(DATA) == 1928


def test_part2() -> None:
    assert part2(DATA) == 2858
