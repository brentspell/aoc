from day03 import part1, part2


def test_part1():
    assert part1(1) == 0
    assert part1(2) == 1
    assert part1(3) == 2
    assert part1(4) == 1

    assert part1(5) == 2
    assert part1(6) == 1
    assert part1(7) == 2
    assert part1(8) == 1

    assert part1(9) == 2
    assert part1(25) == 4
    assert part1(49) == 6

    assert part1(10) == 3
    assert part1(11) == 2
    assert part1(12) == 3
    assert part1(13) == 4
    assert part1(13) == 4
    assert part1(15) == 2
    assert part1(16) == 3
    assert part1(17) == 4

    assert part1(1) == 0
    assert part1(12) == 3
    assert part1(10) == 3
    assert part1(23) == 2
    assert part1(1024) == 31


def test_part2():
    assert part2(800) == 806
