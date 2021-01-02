from day12 import part1, part2

DATA = """
0 <-> 2
1 <-> 1
2 <-> 0, 3, 4
3 <-> 2, 4
4 <-> 2, 3, 6
5 <-> 6
6 <-> 4, 5
""".strip()


def test_part1():
    assert part1(DATA) == 6


def test_part2():
    assert part2(DATA) == 2
