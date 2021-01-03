def part1(data, iterations=5):
    return run(data, iterations)


def part2(data, iterations=18):
    return run(data, iterations)


def run(data, iterations):
    # expand new rules for all rotations and flips of existing rules
    rules = {(s := line.split(" => "))[0]: s[1] for line in data.split("\n")}
    for image, rule in list(rules.items()):
        for _ in range(4):
            rules[image] = rule
            rules[hflip(image)] = rule
            rules[vflip(image)] = rule
            image = rotate(image)

    # process the pattern
    pat = ".#./..#/###".split("/")
    for _ in range(iterations):
        new = []
        mod = 2 if len(pat) % 2 == 0 else 3
        for i in range(0, len(pat), mod):
            for j in range(0, len(pat), mod):
                child = "/".join(r[j:][:mod] for r in pat[i:][:mod])
                child = rules[child].split("/")
                for child_i, row in enumerate(child):
                    parent_i = i // mod * len(child)
                    if parent_i + child_i >= len(new):
                        new.append("")
                    new[parent_i + child_i] += row
        pat = new

    return sum(c == "#" for c in "/".join(pat))


def rotate(image):
    image = image.split("/")
    count = len(image)
    return "/".join(
        "".join(image[count - j - 1][i] for j in range(count)) for i in range(count)
    )


def hflip(image):
    image = image.split("/")
    count = len(image)
    return "/".join(
        "".join(image[i][count - j - 1] for j in range(count)) for i in range(count)
    )


def vflip(image):
    image = image.split("/")
    count = len(image)
    return "/".join(
        "".join(image[count - i - 1][j] for j in range(count)) for i in range(count)
    )
