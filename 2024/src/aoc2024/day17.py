import re


def part1(data: str) -> str:
    a, b, c, program = parse(data)
    output = simulate(program, a, b, c)
    return ",".join(str(o) for o in output)


def part2(data: str) -> int:
    _, b, c, program = parse(data)

    # from an analysis of the evolution of the program outputs, each ith element
    # of the output changes for every 2**(i * 3) starting value of `a`
    # so we can walk backward through the output digits and advance the values
    # of `a` accordingly until the whole output matches the program
    a = 0
    for i in range(len(program) - 1, -1, -1):
        while True:
            output = simulate(program, a, b, c)
            if output[i:] == program[i:]:
                break
            a += 2 ** (i * 3)

    assert output == program
    return a


def simulate(program: list[int], a: int, b: int, c: int) -> list[int]:
    ip = 0
    output = []
    while ip < len(program):
        operator, operand = program[ip], program[ip + 1]
        literal = operand

        # extract the combo parameter
        match operand:
            case 0 | 1 | 2 | 3:
                combo = literal
            case 4:
                combo = a
            case 5:
                combo = b
            case 6:
                combo = c
            case _:
                assert False

        # execute the current operator
        match operator:
            case 0:
                a //= 2**combo
            case 1:
                b ^= literal
            case 2:
                b = combo % 8
            case 3:
                if a != 0:
                    ip = literal
                    continue
            case 4:
                b ^= c
            case 5:
                output.append(combo % 8)
            case 6:
                b = a // 2**combo
            case 7:
                c = a // 2**combo
            case _:
                assert False

        # advance to the next instruction, skipping over the operand
        ip += 2

    return output


def parse(data: str) -> tuple[int, int, int, list[int]]:
    match = re.match(
        r"Register A: (\d+)\nRegister B: (\d+)\nRegister C: (\d+)\n\nProgram: ((?:\d+,?)+)",
        data,
    )
    assert match
    a, b, c = [int(match.group(i + 1)) for i in range(3)]
    program = [int(s) for s in match.group(4).split(",")]
    return a, b, c, program
