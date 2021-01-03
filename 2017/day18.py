from collections import defaultdict


def part1(data):
    program = [s.split(" ") for s in data.split("\n")]
    regs = defaultdict(int)
    while 0 <= regs["ip"] < len(program):
        code, x, y = (program[regs["ip"]] + [None])[:3]
        if code == "snd":
            regs["snd"] = decode(regs, x)
        elif code == "set":
            regs[x] = decode(regs, y)
        elif code == "add":
            regs[x] += decode(regs, y)
        elif code == "mul":
            regs[x] *= decode(regs, y)
        elif code == "mod":
            regs[x] %= decode(regs, y)
        elif code == "rcv":
            if regs[x] != 0:
                regs["rcv"] = regs["snd"]
                break
        elif code == "jgz":
            if regs[x] > 0:
                regs["ip"] += decode(regs, y) - 1
        regs["ip"] += 1
    return regs["rcv"]


def part2(data):
    program = [s.split(" ") for s in data.split("\n")]
    processes = [defaultdict(int, p=0), defaultdict(int, p=1)]
    queues = [[], []]
    result = 0
    done = False
    while not done:
        done = True
        for regs, input_, output in zip(processes, queues, reversed(queues)):
            if 0 <= regs["ip"] < len(program):
                code, x, y = (program[regs["ip"]] + [None])[:3]
                if code == "snd":
                    output.append(decode(regs, x))
                    result += regs is processes[1]
                elif code == "set":
                    regs[x] = decode(regs, y)
                elif code == "add":
                    regs[x] += decode(regs, y)
                elif code == "mul":
                    regs[x] *= decode(regs, y)
                elif code == "mod":
                    regs[x] %= decode(regs, y)
                elif code == "rcv":
                    if not input_:
                        continue
                    regs[x] = input_.pop(0)
                elif code == "jgz":
                    if decode(regs, x) > 0:
                        regs["ip"] += decode(regs, y) - 1
                regs["ip"] += 1
                done = False
    return result


def decode(regs, value):
    if value.isalpha():
        return regs[value]
    return int(value)
