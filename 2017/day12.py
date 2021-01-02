def part1(data):
    return len(group(parse(data), "0"))


def part2(data):
    links = parse(data)
    done = set()
    groups = 0
    for key in links.keys():
        if key not in done:
            groups += 1
            done |= group(links, key)
    return groups


def parse(data):
    return {
        (s := line.split(" <-> "))[0]: set(s[1].split(", "))
        for line in data.split("\n")
    }


def group(links, start):
    done = set()
    todo = [start]
    while todo:
        node = todo.pop()
        done.add(node)
        todo += [n for n in links[node] if n not in done]
    return done
