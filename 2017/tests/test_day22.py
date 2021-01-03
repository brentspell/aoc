from day22 import part1, part2

DATA = """
..#
#..
...
""".strip()


def test_part1():
    assert part1(DATA) == 5587


def test_part2():
    assert part2(DATA, 100) == 26
