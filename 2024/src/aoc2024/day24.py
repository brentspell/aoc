import typing as T


def part1(data: str) -> int:
    wires, gates = parse(data)

    changed = True
    while changed:
        changed = False
        for c, (a, op, b) in gates.items():
            if c not in wires and a in wires and b in wires:
                match op:
                    case "AND":
                        wires[c] = wires[a] & wires[b]
                    case "OR":
                        wires[c] = wires[a] | wires[b]
                    case "XOR":
                        wires[c] = wires[a] ^ wires[b]
                changed = True

    return sum(
        wires[k] << i
        for i, k in enumerate(sorted(k for k in wires.keys() if k.startswith("z")))
    )


def part2(data: str) -> str:
    _, gates = parse(data)
    zs = sorted([k for k in gates.keys() if k.startswith("z")])

    # first, produce the set of all canonical subexpressions
    # needed to produce the output bits of the summation
    canons = set()
    for i, a in enumerate(zs):
        expr: tuple | str = gen_sum(i) if a != zs[-1] else gen_carry(i)
        for e in flatten(expr):
            canons.add(e)

    swapped = []
    for i, a in enumerate(zs):
        # build a mapping of expression->symbol name, used to identify
        # symbols that match canonical subexpressions
        # also canonicalize any reversed subexpressions
        expr2sym = {}
        changed = True
        while changed:
            changed = False
            for sym in gates.keys():
                expr = expand(gates, sym)
                if expr in canons:
                    expr2sym[expr] = sym
                elif (expr := expr[::-1]) in canons:
                    expr2sym[expr] = sym
                    gates[sym] = gates[sym][::-1]
                    changed = True

        # attempt to match up the current z expression
        # to its canonical version
        expect = gen_sum(i) if a != zs[-1] else gen_carry(i - 1)
        actual = expand(gates, a)
        if actual[0] == expect[2] or actual[2] == expect[0]:
            actual = actual[::-1]

        # if it doesn't match, attempt to repair it
        if actual != expect:
            if b := expr2sym.get(expect):
                # in this case, the z term itself needs to be swapped
                pass
            elif actual[0] != expect[0]:
                # here, the lhs of the expression needs to be swapped
                a = expr2sym[actual[0]]
                b = expr2sym[expect[0]]
            elif actual[2] != expect[2]:
                # here the rhs of the expression needs to be swapped
                a = expr2sym[actual[2]]
                b = expr2sym[expect[2]]
            else:
                assert False

            # record and apply the swap, then continue searching
            gates[a], gates[b] = gates[b], gates[a]
            swapped.extend([a, b])

    return ",".join(sorted(swapped))


def gen_sum(i: int) -> tuple:
    xi = f"x{i:02d}"
    yi = f"y{i:02d}"
    s: tuple = (xi, "XOR", yi)
    if i > 0:
        s = (s, "XOR", gen_carry(i - 1))
    return s


def gen_carry(i: int) -> tuple:
    xi = f"x{i:02d}"
    yi = f"y{i:02d}"
    c: tuple = (xi, "AND", yi)
    if i > 0:
        c = (c, "OR", ((xi, "XOR", yi), "AND", (gen_carry(i - 1))))
    return c


def flatten(expr: tuple | str) -> T.Iterator[tuple | str]:
    yield expr
    if isinstance(expr, tuple):
        a, _, b = expr
        yield from flatten(a)
        yield from flatten(b)


def expand(gates: dict[str, tuple], wire: str) -> tuple | str:
    if wire in gates:
        a, op, b = gates[wire]
        return (expand(gates, a), op, expand(gates, b))
    else:
        return wire


def parse(data: str) -> tuple[dict[str, int], dict[str, tuple]]:
    s1, s2 = data.split("\n\n", maxsplit=1)
    w = {k: int(v) for s in s1.splitlines() for k, v in [s.split(": ")]}
    g = {b: tuple(a.split()) for s in s2.splitlines() for a, b in [s.split(" -> ")]}
    return w, g
