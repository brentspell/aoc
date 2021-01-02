from day16 import part1, part2


def test_part1():
    assert part1("s1,x3/4,pe/b", 5) == "baedc"


def test_part2():
    assert part2("s1,x3/4,pe/b", 5, 2) == "ceadb"
