def part1(data):
    return sum(len(s := line.split()) == len(set(s)) for line in data.split("\n"))


def part2(data):
    return sum(
        len(s := ["".join(sorted(s)) for s in line.split()]) == len(set(s))
        for line in data.split("\n")
    )
