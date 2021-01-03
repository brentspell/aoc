def part1(data):
    components = parse(data)
    return max((sum(map(sum, b)) for b in bridges(components, 0)), default=0)


def part2(data):
    components = parse(data)
    longest = 0
    strongest = 0
    for bridge in bridges(components, 0):
        strength = sum(map(sum, bridge))
        if len(bridge) == longest:
            strongest = max(strongest, strength)
        elif len(bridge) > longest:
            longest = len(bridge)
            strongest = strength
    return strongest


def parse(data):
    return {(int((s := line.split("/"))[0]), int(s[1])) for line in data.split("\n")}


def bridges(components, start):
    none = True
    for component in components:
        x, y = component
        if y == start:
            x, y = y, x
        if x == start:
            none = False
            for bridge in bridges(components - {component}, y):
                yield [component] + bridge
    if none:
        yield []
