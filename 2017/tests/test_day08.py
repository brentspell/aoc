from day08 import part1, part2

DATA = """
b inc 5 if a > 1
a inc 1 if b < 5
c dec -10 if a >= 1
c inc -20 if c == 10
""".strip()


def test_part1():
    assert part1(DATA) == 1


def test_part2():
    assert part2(DATA) == 10
