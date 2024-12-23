import collections
import functools

NUMPAD = tuple(
    """
789
456
123
#0A
""".strip().splitlines()
)

DIRPAD = tuple(
    """
#^A
<v>
""".strip().splitlines()
)


def part1(data: str) -> int:
    return count_complexity(data, 2)


def part2(data: str) -> int:
    return count_complexity(data, 25)


def count_complexity(data: str, depth: int) -> int:
    return sum(
        min(find_depth_length(c, depth) for c in all_paths(NUMPAD, code))
        * int(code[:-1])
        for code in data.splitlines()
    )


@functools.cache
def find_depth_length(code: str, depth: int) -> int:
    # find the lengths sequences at each depth
    if depth == 0:
        return len(code)

    # we can break the problem down into smaller pieces and memoize,
    # since each code separated by "A" can be treated independently
    return sum(
        min(find_depth_length(s, depth - 1) for s in all_paths(DIRPAD, c))
        for c in dirsplit(code)
    )


def all_paths(keypad: tuple[str, ...], code: str) -> list[str]:
    # find all shortest paths that can be used to emit a code
    beg = "A"
    paths = [""]
    for end in code:
        paths = [p1 + p2 for p2 in search_keypad(keypad, beg, end) for p1 in paths]
        beg = end
    return paths


@functools.cache
def search_keypad(keypad: tuple[str, ...], beg: str, end: str) -> list[str]:
    keys = {c: (i, j) for i, r in enumerate(keypad) for j, c in enumerate(r)}

    # use bfs to find all shortest paths between two keys
    paths = []
    queue = collections.deque([(*keys[beg], "", {keys[beg]})])
    min_length: int | None = None
    while queue:
        i, j, p, done = queue.popleft()

        # emit all paths that are of the shortest length
        if (i, j) == keys[end]:
            if min_length is not None and len(p) > min_length:
                break
            min_length = len(p)
            paths.append(p + "A")
            continue

        # find neighboring keys and record the directions
        for d in "^v<>":
            match d:
                case "^":
                    u, v = i - 1, j
                case "v":
                    u, v = i + 1, j
                case "<":
                    u, v = i, j - 1
                case ">":
                    u, v = i, j + 1

            # stay on the pad and don't step on gaps
            if 0 <= u < len(keypad) and 0 <= v < len(keypad[0]):
                if keypad[u][v] != "#" and (u, v) not in done:
                    done.add((u, v))
                    queue.append((u, v, p + d, done | {(u, v)}))

    return paths


def dirsplit(s: str) -> list[str]:
    # key sequences between "A" can be treated independently, so split on them
    chs = list(s)
    out = [""]
    for ch in chs:
        if ch == "A":
            out.append("")
        else:
            out[-1] += ch
    out.pop()
    return [s + "A" for s in out]
