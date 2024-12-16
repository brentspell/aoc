import heapq


def part1(data: str) -> int:
    # find the starting point
    grid = data.splitlines()
    i, j = next((i, j) for i, r in enumerate(grid) for j, c in enumerate(r) if c == "S")

    # use dijkstra to find the best path through the maze
    queue = [(0, i, j, ">")]
    scores = {(i, j, ">"): 0}
    best = 1_000_000
    while queue:
        s, i, j, f = heapq.heappop(queue)

        # if we have reached the goal, check to see if this was the best path
        if grid[i][j] == "E":
            best = min(best, s)

        # find the next states based on the current position and facing
        match f:
            case "^":
                u, v, t = -1, 0, "<>"
            case "v":
                u, v, t = 1, 0, "<>"
            case "<":
                u, v, t = 0, -1, "^v"
            case ">":
                u, v, t = 0, 1, "^v"

        moves = [(s + 1, i + u, j + v, f)]
        turns = [(s + 1000, i, j, t) for t in t]

        # expand the search to the next states, updating the cost mapping
        # as cheaper costs are encountered
        for s, p, q, f in moves + turns:
            if grid[p][q] != "#" and scores.get((p, q, f), s + 1) > s:
                scores[(p, q, f)] = s
                heapq.heappush(queue, (s, p, q, f))

    return best


def part2(data: str) -> int:
    # find the starting point
    grid = data.splitlines()
    i, j = next((i, j) for i, r in enumerate(grid) for j, c in enumerate(r) if c == "S")

    # use dijkstra to find the best paths through the maze
    queue = [(0, i, j, ">", [(i, j)])]
    scores = {(i, j, ">"): 0}
    best = 1_000_000
    seats: set[tuple[int, int]] = set()
    while queue:
        s, i, j, f, b = heapq.heappop(queue)

        # if we have reached the goal, reset the set of seats along the path
        # if this is a new best score, otherwise just add them if it is the same score
        if grid[i][j] == "E":
            if s < best:
                best = s
                seats.clear()
            if s == best:
                seats |= set(b)

        # find the next states based on the current position and facing
        match f:
            case "^":
                u, v, t = -1, 0, "<>"
            case "v":
                u, v, t = 1, 0, "<>"
            case "<":
                u, v, t = 0, -1, "^v"
            case ">":
                u, v, t = 0, 1, "^v"

        moves = [(s + 1, i + u, j + v, f)]
        turns = [(s + 1000, i, j, t) for t in t]

        # expand the search to the next states, updating the cost mapping
        # as cheaper (or same) costs are encountered
        for s, p, q, f in moves + turns:
            if grid[p][q] != "#" and scores.get((p, q, f), s + 1) >= s:
                scores[(p, q, f)] = s
                heapq.heappush(queue, (s, p, q, f, b + [(p, q)]))

    return len(seats)
