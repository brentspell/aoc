from day09 import part1, part2


def test_part1():
    assert part1("{}") == 1
    assert part1("{{{}}}") == 6
    assert part1("{{},{}}") == 5
    assert part1("{{{},{},{{}}}}") == 16
    assert part1("{<a>,<a>,<a>,<a>}") == 1
    assert part1("{{<ab>},{<ab>},{<ab>},{<ab>}}") == 9
    assert part1("{{<!!>},{<!!>},{<!!>},{<!!>}}") == 9
    assert part1("{{<a!>},{<a!>},{<a!>},{<ab>}}") == 3


def test_part2():
    assert part2("<>") == 0
    assert part2("<random characters>") == 17
    assert part2("<<<<>") == 3
    assert part2("<{!>}>") == 2
    assert part2("<!!>") == 0
    assert part2("<!!!>>") == 0
    assert part2('<{o"i!a,<{i<a>') == 10
