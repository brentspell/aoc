def part1(data):
    blocks = [int(w) for w in data.split()]
    uniq = set()
    while (sig := tuple(blocks)) not in uniq:
        uniq.add(sig)
        _redistribute(blocks)
    return len(uniq)


def part2(data):
    blocks = [int(w) for w in data.split()]
    for _ in range(2):
        uniq = set()
        while (sig := tuple(blocks)) not in uniq:
            uniq.add(sig)
            _redistribute(blocks)
    return len(uniq)


def _redistribute(blocks):
    count, _, offset = max((v, -i, i) for i, v in enumerate(blocks))
    blocks[offset] = 0
    while count > 0:
        offset = (offset + 1) % len(blocks)
        blocks[offset] += 1
        count -= 1
