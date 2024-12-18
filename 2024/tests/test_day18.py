from aoc2024.day18 import part1, part2

DATA = """
5,4
4,2
4,5
3,0
2,1
6,3
2,4
1,5
0,6
3,3
2,6
5,1
1,2
5,5
2,5
6,5
1,4
0,4
6,4
1,1
6,1
1,0
0,5
1,6
2,0
""".strip()


def test_part1() -> None:
    assert part1(DATA, dim=6, corrupt=12) == 22


def test_part2() -> None:
    assert part2(DATA, dim=6) == "6,1"
