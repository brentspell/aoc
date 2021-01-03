from collections import defaultdict


def part1(data):
    program = [s.split(" ") for s in data.split("\n")]
    regs = defaultdict(int)
    result = 0
    while 0 <= regs["ip"] < len(program):
        code, x, y = program[regs["ip"]]
        if code == "set":
            regs[x] = decode(regs, y)
        elif code == "sub":
            regs[x] -= decode(regs, y)
        elif code == "mul":
            regs[x] *= decode(regs, y)
            result += 1
        elif code == "jnz":
            if decode(regs, x) != 0:
                regs["ip"] += decode(regs, y) - 1
        regs["ip"] += 1
    return result


def part2(data):
    program = [s.split(" ") for s in data.split("\n")]
    regs = defaultdict(int, a=1)
    while 0 <= regs["ip"] < len(program):
        # hook the program at line 11 and
        # implement a faster primality test
        if regs["ip"] == 11:
            if not is_prime(regs["b"]):
                regs["f"] = 0
            regs["d"] = regs["b"]
            regs["e"] = regs["b"]
            regs["g"] = 0
            regs["ip"] = 24

        # otherwise, execute normally
        code, x, y = program[regs["ip"]]
        if code == "set":
            regs[x] = decode(regs, y)
        elif code == "sub":
            regs[x] -= decode(regs, y)
        elif code == "mul":
            regs[x] *= decode(regs, y)
        elif code == "jnz":
            if decode(regs, x) != 0:
                regs["ip"] += decode(regs, y) - 1
        regs["ip"] += 1
    return regs["h"]


def decode(regs, value):
    if value.isalpha():
        return regs[value]
    return int(value)


def is_prime(n):
    if n <= 3:
        return n > 1
    if n % 2 == 0 or n % 3 == 0:
        return False
    i = 5
    while i ** 2 <= n:
        if n % i == 0 or n % (i + 2) == 0:
            return False
        i += 6
    return True
