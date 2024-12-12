import collections


def part1(data: str) -> int:
    grid = data.splitlines()

    done = set()
    price = 0
    for i in range(len(grid)):
        for j in range(len(grid[0])):
            if (i, j) not in done:
                done.add((i, j))

                # use DFS to find the area and perimeter of this region
                stack = [(i, j)]
                a = p = 0
                while stack:
                    # accumulate area and perimeter, overcounting edges
                    r, s = stack.pop()
                    a += 1
                    p += 4

                    # expand the search to neighbors
                    for u, v in [(r - 1, s), (r + 1, s), (r, s - 1), (r, s + 1)]:
                        if 0 <= u < len(grid) and 0 <= v < len(grid[0]):
                            if grid[u][v] == grid[r][s]:
                                # correct for overcounting and expand the search
                                p -= 1
                                if (u, v) not in done:
                                    done.add((u, v))
                                    stack.append((u, v))

                price += a * p

    return price


def part2(data: str) -> int:
    grid = data.splitlines()

    done = set()
    price = 0
    for i in range(len(grid)):
        for j in range(len(grid[0])):
            if (i, j) not in done:
                done.add((i, j))

                # use DFS to find the area and number of edges of this region
                # since the number of edges is equal to the number of corners,
                # we can find all box vertices around each plant and use the
                # distinct counts to determine the number of corners
                stack = [(i, j)]
                a = 0
                verts = collections.defaultdict(list)
                while stack:
                    r, s = stack.pop()

                    # add a vertex for each corner of the current plant
                    for w, x in [(-1, -1), (1, -1), (-1, 1), (1, 1)]:
                        y, z = r + 0.5 * w, s + 0.5 * x
                        verts[(y, z)].append((r, s))

                    # accumulate area
                    a += 1

                    # expand the search to neighbors
                    for u, c in [(r - 1, s), (r + 1, s), (r, s - 1), (r, s + 1)]:
                        if 0 <= u < len(grid) and 0 <= c < len(grid[0]):
                            if grid[u][c] == grid[r][s]:
                                if (u, c) not in done:
                                    done.add((u, c))
                                    stack.append((u, c))

                # count the corners of the region
                # . if there is an odd number of plants at a vertex, we have a corner
                # . if there are two plants that touch on a diagonal, we have 2 corners
                # . otherwise, we have a horizontal or vertical line or a 4-plant box,
                #   so no corners
                c = sum(
                    1
                    if len(p) % 2 == 1
                    else 2
                    if len(p) == 2 and p[0][0] != p[1][0] and p[0][1] != p[1][1]
                    else 0
                    for p in verts.values()
                )
                price += a * c

    return price
