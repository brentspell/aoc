import operator
import re
from collections import defaultdict, namedtuple

Instruction = namedtuple(
    "Instruction", ["output", "op", "param", "input", "compare", "check"]
)

OPS = {
    "inc": operator.add,
    "dec": operator.sub,
    "==": operator.eq,
    "!=": operator.ne,
    "<=": operator.le,
    ">=": operator.ge,
    "<": operator.lt,
    ">": operator.gt,
}


def part1(data):
    return max(run([parse(s) for s in data.split("\n")])[0].values())


def part2(data):
    return run([parse(s) for s in data.split("\n")])[1]


def parse(line):
    match = re.match(r"(\w+) (inc|dec) (-?\d+) if (\w+) ([<>=!]+) (-?\d+)", line)
    return Instruction(
        output=match.group(1),
        op=OPS[match.group(2)],
        param=int(match.group(3)),
        input=match.group(4),
        compare=OPS[match.group(5)],
        check=int(match.group(6)),
    )


def run(program):
    regs = defaultdict(int)
    max_ = 0
    for code in program:
        if code.compare(regs[code.input], code.check):
            regs[code.output] = code.op(regs[code.output], code.param)
            max_ = max(regs[code.output], max_)
    return regs, max_
