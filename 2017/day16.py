def part1(data, count=16):
    programs = [chr(i + ord("a")) for i in range(count)]
    run(programs, data)
    return "".join(programs)


def part2(data, count=16, iterations=1000000000):
    programs = [chr(i + ord("a")) for i in range(count)]
    done = set()
    i = 0
    while i < iterations:
        run(programs, data)
        if "".join(programs) in done:
            i = iterations - iterations % 36
            done.clear()
        done.add("".join(programs))
        i += 1
    return "".join(programs)


def run(programs, data):
    for move in data.split(","):
        if move[0] == "s":
            spin = int(move[1:])
            left, right = programs[-spin:], programs[:-spin]
            programs.clear()
            programs.extend(left + right)
        elif move[0] == "x":
            a, b = map(int, move[1:].split("/"))
            programs[a], programs[b] = programs[b], programs[a]
        elif move[0] == "p":
            a, b = [programs.index(x) for x in move[1:].split("/")]
            programs[a], programs[b] = programs[b], programs[a]
