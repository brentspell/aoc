def part1(data):
    return run(data)[0]


def part2(data):
    return run(data)[1]


def run(data):
    lines = data.split("\n")
    lines = [s.ljust(max(map(len, lines))) for s in lines]
    y, x = 0, lines[0].index("|")
    dy, dx = 1, 0
    result = ""
    steps = 0
    while (char := lines[y][x]) != " ":
        if char == "+":
            if dx == 0:
                dy = 0
                if x + 1 < len(lines[y]) and lines[y][x + 1] != " ":
                    dx = 1
                elif x - 1 > 0 and lines[y][x - 1] != " ":
                    dx = -1
            else:
                dx = 0
                if y + 1 < len(lines) and lines[y + 1][x] != " ":
                    dy = 1
                elif y - 1 > 0 and lines[y - 1][x] != " ":
                    dy = -1
        elif char.isalpha():
            result += char
        y += dy
        x += dx
        steps += 1
    return result, steps
