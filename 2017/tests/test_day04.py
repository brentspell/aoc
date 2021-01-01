from day04 import part1, part2


def test_part1():
    assert part1("aa bb cc dd ee") == 1
    assert part1("aa bb cc dd aa") == 0
    assert part1("aa bb cc dd aaa") == 1


def test_part2():
    assert part2("abcde fghij") == 1
    assert part2("abcde xyz ecdab") == 0
    assert part2("a ab abc abd abf abj") == 1
    assert part2("iiii oiii ooii oooi oooo") == 1
    assert part2("oiii ioii iioi iiio") == 0
