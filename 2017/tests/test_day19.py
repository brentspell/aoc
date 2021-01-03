from day19 import part1, part2

DATA = """
     |
     |  +--+
     A  |  C
 F---|----E|--+
     |  |  |  D
     +B-+  +--+
"""[
    1:-1
]


def test_part1():
    assert part1(DATA) == "ABCDEF"


def test_part2():
    assert part2(DATA) == 38
