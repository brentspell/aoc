def part1(data):
    tree, _, _ = parse(data)
    return score(tree)


def part2(data):
    _, _, garbage = parse(data)
    return garbage


def parse(data):
    tree = []
    length = 0
    total = 0
    garbage = False
    while length < len(data):
        char = data[length]
        length += 1
        if garbage:
            if char == "!":
                length += 1
            elif char == ">":
                garbage = False
            else:
                total += 1
        elif char == "<":
            garbage = True
        elif char == "{":
            subtree, sublength, subtotal = parse(data[length:])
            tree.append(subtree)
            length += sublength
            total += subtotal
        elif char == "}":
            break
    return tree, length, total


def score(tree, base=0):
    return base + sum(score(c, base + 1) for c in tree)
