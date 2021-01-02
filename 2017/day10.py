import operator
from functools import reduce


def part1(data, count=256):
    lengths = map(int, data.split(","))
    knot = list(range(count))
    tie(knot, lengths)
    return knot[0] * knot[1]


def part2(data):
    lengths = [ord(c) for c in data] + [17, 31, 73, 47, 23]
    knot = list(range(256))

    offset = 0
    skip = 0
    for _ in range(64):
        offset, skip = tie(knot, lengths, offset, skip)

    return "".join(f"{x:02x}" for x in densify(knot))


def tie(knot, lengths, offset=0, skip=0):
    for length in lengths:
        span = (knot + knot)[offset:][:length]
        for i, x in enumerate(reversed(span), offset):
            knot[i % len(knot)] = x
        offset = (offset + length + skip) % len(knot)
        skip += 1
    return offset, skip


def densify(knot):
    return [reduce(operator.xor, knot[i:][:16]) for i in range(0, len(knot), 16)]
