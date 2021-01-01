from day02 import part1, part2


def test_part1():
    assert (
        part1(
            """
                 5 1 9 5
                 7 5 3
                 2 4 6 8
                 """.strip()
        )
        == 18
    )


def test_part2():
    assert (
        part2(
            """
                 5 9 2 8
                 9 4 7 3
                 3 8 6 5
                 """.strip()
        )
        == 9
    )
