from day13 import part1, part2

DATA = """
0: 3
1: 2
4: 4
6: 4
""".strip()


def test_part1():
    assert part1(DATA) == 24


def test_part2():
    assert part2(DATA) == 10
