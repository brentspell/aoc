from day24 import part1, part2

DATA = """
0/2
2/2
2/3
3/4
3/5
0/1
10/1
9/10
""".strip()


def test_part1():
    assert part1(DATA) == 31


def test_part2():
    assert part2(DATA) == 19
