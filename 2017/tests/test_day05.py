from day05 import part1, part2

DATA = """
0
3
0
1
-3
""".strip()


def test_part1():
    assert part1(DATA) == 5


def test_part2():
    assert part2(DATA) == 10
