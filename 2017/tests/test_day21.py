from day21 import part1

DATA = """
../.# => ##./#../...
.#./..#/### => #..#/..../..../#..#
""".strip()


def test_part1():
    assert part1(DATA, 2) == 12
