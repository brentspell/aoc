from aoc2024.day06 import part1, part2

DATA = """
....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...
""".strip()


def test_part1() -> None:
    assert part1(DATA) == 41


def test_part2() -> None:
    assert part2(DATA) == 6
