from day10 import part1, part2


def test_part1():
    assert part1("3, 4, 1, 5", 5) == 12


def test_part2():
    assert part2("") == "a2582a3a0e66e6e86e3812dcb672a272"
    assert part2("AoC 2017") == "33efeb34ea91902bb2f59c9920caa6cd"
    assert part2("1,2,3") == "3efbe78a8d82f29979031a4aa0b16a9d"
    assert part2("1,2,4") == "63960835bcdc130f0b66d7ff4f6a5a8e"
