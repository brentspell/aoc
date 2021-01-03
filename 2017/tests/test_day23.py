from day23 import part1, part2

DATA = """
mul b 100
sub b -100000
set h 5
""".strip()


def test_part1():
    assert part1(DATA) == 1


def test_part2():
    assert part2(DATA) == 5
