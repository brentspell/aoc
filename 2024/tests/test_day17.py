from aoc2024.day17 import part1, part2

DATA1 = """
Register A: 729
Register B: 0
Register C: 0

Program: 0,1,5,4,3,0
""".strip()

DATA2 = """
Register A: 2024
Register B: 0
Register C: 0

Program: 0,3,5,4,3,0

a := int(a / 2**3)
output a % 8
if a != 0: goto 0

""".strip()


def test_part1() -> None:
    assert part1(DATA1) == "4,6,3,5,6,3,5,2,1,0"


def test_part2() -> None:
    assert part2(DATA2) == 117440
