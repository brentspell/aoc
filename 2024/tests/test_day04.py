from aoc2024.day04 import part1, part2

DATA = """
MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX
""".strip()


def test_part1() -> None:
    assert part1(DATA) == 18


def test_part2() -> None:
    assert part2(DATA) == 9
