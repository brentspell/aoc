def part1(data):
    firewall = parse(data)
    return sum(
        depth * height if depth % (height * 2 - 2) == 0 else 0
        for depth, height in firewall
    )


def part2(data):
    firewall = parse(data)
    delay = 0
    caught = True
    while caught:
        delay += 1
        caught = False
        scanner = delay
        layer = 0
        for depth, height in firewall:
            scanner += depth - layer
            layer = depth
            if scanner % (height * 2 - 2) == 0:
                caught = True
                break
    return delay


def parse(data):
    return [[int(w) for w in s.split(": ")] for s in data.split("\n")]
