def part1(data):
    program = [int(s) for s in data.split("\n")]
    offset = 0
    count = 0
    while 0 <= offset < len(program):
        jmp = program[offset]
        program[offset] += 1
        offset += jmp
        count += 1
    return count


def part2(data):
    program = [int(s) for s in data.split("\n")]
    offset = 0
    count = 0
    while 0 <= offset < len(program):
        jmp = program[offset]
        program[offset] += 1 if jmp < 3 else -1
        offset += jmp
        count += 1
    return count
