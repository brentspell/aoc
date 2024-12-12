from aoc2024.day12 import part1, part2

DATA1 = """
AAAA
BBCD
BBCC
EEEC
""".strip()

DATA2 = """
OOOOO
OXOXO
OOOOO
OXOXO
OOOOO
""".strip()

DATA3 = """
RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE
""".strip()

DATA4 = """
EEEEE
EXXXX
EEEEE
EXXXX
EEEEE
""".strip()

DATA5 = """
AAAAAA
AAABBA
AAABBA
ABBAAA
ABBAAA
AAAAAA
""".strip()


def test_part1() -> None:
    assert part1(DATA1) == 140
    assert part1(DATA2) == 772
    assert part1(DATA3) == 1930


def test_part2() -> None:
    assert part2(DATA1) == 80
    assert part2(DATA2) == 436
    assert part2(DATA3) == 1206
    assert part2(DATA4) == 236
    assert part2(DATA5) == 368
