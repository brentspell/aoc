from day18 import part1, part2

DATA1 = """
set a 1
add a 2
mul a a
mod a 5
snd a
set a 0
rcv a
jgz a -1
set a 1
jgz a -2
""".strip()

DATA2 = """
snd 1
snd 2
snd p
rcv a
rcv b
rcv c
rcv d
""".strip()


def test_part1():
    assert part1(DATA1) == 4


def test_part2():
    assert part2(DATA2) == 3
