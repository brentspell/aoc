from day11 import part1


def test_part1():
    assert part1("ne,ne,ne") == 3
    assert part1("ne,ne,sw,sw") == 0
    assert part1("ne,ne,s,s") == 2
    assert part1("se,sw,se,sw,sw") == 3
