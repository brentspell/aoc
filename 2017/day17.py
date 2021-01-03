def part1(data):
    steps = int(data)
    lock = [0]
    pos = 0
    for i in range(1, 2018):
        pos = (pos + steps) % len(lock) + 1
        lock.insert(pos, i)
    return lock[pos + 1]


def part2(data):
    steps = int(data)
    pos = 0
    zero = 0
    result = 0
    length = 1
    for i in range(1, 50000001):
        pos = (pos + steps) % length + 1
        if pos <= zero:
            zero += 1
        elif pos == zero + 1:
            result = i
        length += 1
    return result
