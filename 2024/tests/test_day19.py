from aoc2024.day19 import part1, part2

DATA = """
r, wr, b, g, bwu, rb, gb, br

brwrr
bggr
gbbr
rrbgbr
ubwu
bwurrg
brgr
bbrgwb
""".strip()


def test_part1() -> None:
    assert part1(DATA) == 6


def test_part2() -> None:
    assert part2(DATA) == 16
