def part1(data, count=40000000):
    a, b = [int(w) for w in data.split(",")]
    total = 0
    for _ in range(count):
        a = a * 16807 % 2147483647
        b = b * 48271 % 2147483647
        if a & 0xFFFF == b & 0xFFFF:
            total += 1
    return total


def part2(data, count=5000000):
    a, b = [int(w) for w in data.split(",")]
    total = 0
    for _ in range(count):
        while (a := a * 16807 % 2147483647) % 4 != 0:
            pass
        while (b := b * 48271 % 2147483647) % 8 != 0:
            pass
        if a & 0xFFFF == b & 0xFFFF:
            total += 1
    return total
